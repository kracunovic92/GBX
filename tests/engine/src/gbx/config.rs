#[derive(Debug, Clone)]
pub struct GbxConfig {
    /// Normalize inputs / remainders (when algorithm supports it)
    pub normalize: bool,
    /// (future) enable/disable Buchberger criteria
    pub criteria: bool,
}
