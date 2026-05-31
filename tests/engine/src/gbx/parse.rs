use anyhow::{Context, Result, bail};

/// Parse a polynomial string into list of (coeff_mod_p, exponent-vector) terms.
/// `exponent-vector` length equals `vars.len()`.
///
/// Minimal grammar (good enough for your TOML cases):
/// - Terms separated by `+` / `-`
/// - Each term is a product of factors:
///   - integer coefficient
///   - variable name in `vars` optionally followed by `^k`
/// - Multiplication may be explicit (`*`) or implicit by adjacency/whitespace.
///
/// Examples:
/// - "x^2 + y^2 - 1"
/// - "x^3 - y"
/// - "3x^2y - 7xy + 2"
pub fn parse_poly_terms(input: &str, vars: &[String], p: u32) -> Result<Vec<(u32, Vec<u32>)>> {
    if vars.is_empty() {
        bail!("vars must be non-empty");
    }
    if p <= 1 {
        bail!("Fp requires p>1, got {}", p);
    }

    let s = input.trim();
    if s.is_empty() {
        bail!("empty polynomial string");
    }

    let mut var_index = std::collections::HashMap::<&str, usize>::new();
    for (i, v) in vars.iter().enumerate() {
        var_index.insert(v.as_str(), i);
    }

    let mut pos = 0usize;
    let chars: Vec<char> = s.chars().collect();

    let nvars = vars.len();
    let mut out: Vec<(u32, Vec<u32>)> = Vec::new();

    // helper closures
    let skip_ws = |pos: &mut usize| {
        while *pos < chars.len() && chars[*pos].is_whitespace() {
            *pos += 1;
        }
    };

    let peek = |pos: usize| -> Option<char> { if pos < chars.len() { Some(chars[pos]) } else { None } };

    let parse_u64 = |pos: &mut usize| -> Option<u64> {
        skip_ws(pos);
        let start = *pos;
        while *pos < chars.len() && chars[*pos].is_ascii_digit() {
            *pos += 1;
        }
        if *pos > start {
            let num: String = chars[start..*pos].iter().collect();
            num.parse::<u64>().ok()
        } else {
            None
        }
    };

    let parse_ident = |pos: &mut usize| -> Option<String> {
        skip_ws(pos);
        let start = *pos;
        if *pos < chars.len() && (chars[*pos].is_ascii_alphabetic() || chars[*pos] == '_') {
            *pos += 1;
            while *pos < chars.len() && (chars[*pos].is_ascii_alphanumeric() || chars[*pos] == '_') {
                *pos += 1;
            }
            Some(chars[start..*pos].iter().collect())
        } else {
            None
        }
    };

    let parse_term = |pos: &mut usize| -> Result<(i64, Vec<u32>)> {
        skip_ws(pos);

        // Default coeff = 1
        let mut coeff: i64 = 1;
        let mut exps = vec![0u32; nvars];

        let mut saw_any_factor = false;

        loop {
            skip_ws(pos);
            match peek(*pos) {
                None => break,
                Some('+') | Some('-') => break,
                Some('*') => {
                    *pos += 1;
                    continue;
                }
                _ => {}
            }

            // number factor?
            if let Some(num) = parse_u64(pos) {
                saw_any_factor = true;
                // multiply coeff
                let num_i64 = i64::try_from(num).context("coefficient too large")?;
                coeff = coeff.saturating_mul(num_i64);
                continue;
            }

            // ident factor?
            if let Some(id) = parse_ident(pos) {
                saw_any_factor = true;

                let Some(&idx) = var_index.get(id.as_str()) else {
                    bail!("unknown variable '{id}' in '{input}'");
                };

                // optional exponent
                skip_ws(pos);
                let pow: u32 = if peek(*pos) == Some('^') {
                    *pos += 1;
                    let Some(k) = parse_u64(pos) else {
                        bail!("expected exponent after '^' in '{input}'");
                    };
                    u32::try_from(k).context("exponent too large")?
                } else {
                    1
                };

                exps[idx] = exps[idx].saturating_add(pow);
                continue;
            }

            // unknown token
            let ch = peek(*pos).unwrap();
            bail!(
                "unexpected character '{}' at position {} in '{}'",
                ch,
                *pos,
                input
            );
        }

        if !saw_any_factor {
            bail!("expected term at position {} in '{}'", *pos, input);
        }

        Ok((coeff, exps))
    };

    // Main loop: parse signed terms separated by +/-.
    // Default sign is +.
    let mut sign: i64 = 1;

    while pos < chars.len() {
        skip_ws(&mut pos);

        // consume leading +/-
        match peek(pos) {
            Some('+') => {
                sign = 1;
                pos += 1;
                skip_ws(&mut pos);
            }
            Some('-') => {
                sign = -1;
                pos += 1;
                skip_ws(&mut pos);
            }
            _ => {
                // keep previous sign; for first term it's +1
            }
        }

        // parse the term product
        let (coeff_abs, exps) = parse_term(&mut pos)?;
        let coeff_signed = sign.saturating_mul(coeff_abs);

        // reduce coeff mod p into [0, p)
        let coeff_mod = mod_u32_from_i64(coeff_signed, p);

        // keep even if coeff_mod == 0: downstream normalization will drop it
        out.push((coeff_mod, exps));

        // next loop will pick up +/-
        sign = 1;
        skip_ws(&mut pos);
    }

    Ok(out)
}

fn mod_u32_from_i64(x: i64, p: u32) -> u32 {
    let p_i64 = p as i64;
    let mut r = x % p_i64;
    if r < 0 {
        r += p_i64;
    }
    r as u32
}
