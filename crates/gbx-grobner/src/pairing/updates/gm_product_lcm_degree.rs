use crate::pairing::leading_mono_at;
use crate::pairing::updates::gm_product_lcm_degree_trace::{GmProductLcmDegreeUpdateCounters, GmProductLcmDegreeUpdateTimes, GmUpdatePhase};
use crate::pairing::updates::{PairUpdateError, Result};
use crate::{GrobnerBasis, Pair, PairQueue, PairUpdate};

use gbx_poly::monomial::{checked_lcm_degree, gcd_is_one, Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

use std::collections::HashMap;
use std::time::Instant;

/// Conservative Gebauer–Möller-style pair updater specialized for:
///
/// - product criterion
/// - key = `deg(lcm(LM_i, LM_new))`
#[derive(Debug, Default, Clone)]
pub struct GmProductLcmDegreeUpdater {
    trace: GmProductLcmDegreeUpdateState,
}

#[derive(Debug, Default, Clone)]
struct GmProductLcmDegreeUpdateState {
    counters: GmProductLcmDegreeUpdateCounters,
    times: GmProductLcmDegreeUpdateTimes,
}

impl GmProductLcmDegreeUpdater {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            trace: GmProductLcmDegreeUpdateState {
                counters: GmProductLcmDegreeUpdateCounters {
                    update_calls: 0,
                    candidates_considered: 0,
                    rejected_missing_lm: 0,
                    rejected_by_product: 0,
                    rejected_lcm_failure: 0,
                    rejected_key_failure: 0,
                    dedup_collisions: 0,
                    survivors_after_dedup: 0,
                    survivors_after_prune: 0,
                    pairs_pushed: 0,
                },
                times: GmProductLcmDegreeUpdateTimes { scan: std::time::Duration::ZERO, dedup: std::time::Duration::ZERO, prune: std::time::Duration::ZERO, push: std::time::Duration::ZERO },
            },
        }
    }

    #[must_use]
    pub fn counters(&self) -> &GmProductLcmDegreeUpdateCounters {
        &self.trace.counters
    }

    #[must_use]
    pub fn times(&self) -> &GmProductLcmDegreeUpdateTimes {
        &self.trace.times
    }

    #[inline]
    fn add_time(&mut self, phase: GmUpdatePhase, dt: std::time::Duration) {
        self.trace.times.add(phase, dt);
    }
}

/// Newly generated candidate pair associated with a fixed `new_index`.
#[derive(Debug, Clone)]
struct Candidate<M> {
    i: usize,
    key: u32,
    lcm: M,
}

impl<M> Candidate<M> {
    #[inline]
    fn better_than(&self, other: &Self) -> bool {
        (self.key, self.i) < (other.key, other.i)
    }
}

impl<P> PairUpdate<P> for GmProductLcmDegreeUpdater
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + std::hash::Hash,
{
    type Key = u32;

    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) -> Result<()>
    where
        Q: PairQueue<Key = Self::Key> + crate::pairing::filters::PairSetView,
    {
        self.trace.counters.update_calls += 1;

        if new_index >= gb.len() {
            return Err(PairUpdateError::InvariantViolation);
        }

        let Some(lm_new) = leading_mono_at(gb, new_index) else {
            return Ok(());
        };

        let scan_t0 = Instant::now();

        let mut by_lcm: HashMap<<P::Term as TermView>::Mono, Candidate<<P::Term as TermView>::Mono>> = HashMap::new();

        for i in 0..new_index {
            self.trace.counters.candidates_considered += 1;

            let Some(lm_i) = leading_mono_at(gb, i) else {
                self.trace.counters.rejected_missing_lm += 1;
                continue;
            };

            if gcd_is_one(lm_i, lm_new) {
                self.trace.counters.rejected_by_product += 1;
                continue;
            }

            let Ok(lcm) = lm_i.checked_lcm(lm_new) else {
                self.trace.counters.rejected_lcm_failure += 1;
                continue;
            };

            let Some(key) = checked_lcm_degree(lm_i, lm_new).ok() else {
                self.trace.counters.rejected_key_failure += 1;
                continue;
            };

            let cand = Candidate { i, key, lcm: lcm.clone() };

            match by_lcm.get_mut(&lcm) {
                Some(existing) => {
                    self.trace.counters.dedup_collisions += 1;
                    if cand.better_than(existing) {
                        *existing = cand;
                    }
                }
                None => {
                    by_lcm.insert(lcm, cand);
                }
            }
        }

        self.add_time(GmUpdatePhase::Scan, scan_t0.elapsed());

        let dedup_t0 = Instant::now();

        if by_lcm.is_empty() {
            self.add_time(GmUpdatePhase::Dedup, dedup_t0.elapsed());
            return Ok(());
        }

        let deduped: Vec<Candidate<<P::Term as TermView>::Mono>> = by_lcm.into_values().collect();
        self.trace.counters.survivors_after_dedup += deduped.len() as u64;

        self.add_time(GmUpdatePhase::Dedup, dedup_t0.elapsed());

        let prune_t0 = Instant::now();

        let survivors: Vec<Candidate<<P::Term as TermView>::Mono>> = if deduped.len() <= 1 {
            deduped
        } else {
            let mut minimal: Vec<Candidate<<P::Term as TermView>::Mono>> = Vec::new();

            for cand in deduped {
                let mut dominated = false;

                // If some existing minimal LCM strictly divides or equals this one,
                // then this candidate is unnecessary.
                for existing in &minimal {
                    if matches!(cand.lcm.checked_div_by(&existing.lcm), Ok(Some(_))) {
                        dominated = true;
                        break;
                    }
                }

                if dominated {
                    continue;
                }

                // Remove existing candidates dominated by the new minimal LCM.
                minimal.retain(|existing| !matches!(existing.lcm.checked_div_by(&cand.lcm), Ok(Some(_))));

                minimal.push(cand);
            }

            minimal
        };

        self.trace.counters.survivors_after_prune += survivors.len() as u64;

        self.add_time(GmUpdatePhase::Prune, prune_t0.elapsed());

        let push_t0 = Instant::now();

        for cand in survivors {
            pairs.push(Pair::new(cand.key, cand.i, new_index));
            self.trace.counters.pairs_pushed += 1;
        }

        self.add_time(GmUpdatePhase::Push, push_t0.elapsed());

        Ok(())
    }
}
