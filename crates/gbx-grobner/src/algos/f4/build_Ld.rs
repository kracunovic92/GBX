use crate::algos::f4::critical_pair::CriticalPair;
use gbx_poly::monomial::{Monomial, MonomialAlgos};

#[derive(Debug, Clone)]
pub struct SymbolicSeed<M> {
    pub basis_index: usize,
    pub multiplier: M,
}

pub fn build_ld<M: Clone>(pairs: &[CriticalPair<M>]) -> Vec<SymbolicSeed<M>>
where
    M: Clone + Monomial + MonomialAlgos,
{
    let mut out = Vec::with_capacity(pairs.len() * 2);

    for pair in pairs {
        let left = pair.left();
        let right = pair.right();

        out.push(SymbolicSeed { basis_index: left.basis_index, multiplier: left.multiplier.clone() });

        out.push(SymbolicSeed { basis_index: right.basis_index, multiplier: right.multiplier.clone() });
    }

    out
}
