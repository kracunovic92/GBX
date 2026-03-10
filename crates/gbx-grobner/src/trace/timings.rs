#[derive(Debug, Default)]
pub struct PhaseTimes {
    pub init_ms: u128,
    pub seed_ms: u128,
    pub while_ms: u128,
    pub post_ms: u128,
}

#[derive(Debug, Default)]
pub struct WhileTimes {
    pub s_poly_ms: u128,
    pub nf_ms: u128,
    pub rem_norm_ms: u128,
}
