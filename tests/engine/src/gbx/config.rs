#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GbxReducerKind {
    Dense,
    Roman,
    RomanParallel,
}

impl GbxReducerKind {
    pub fn all() -> Vec<Self> {
        vec![Self::Dense, Self::Roman, Self::RomanParallel]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dense => "dense",
            Self::Roman => "roman",
            Self::RomanParallel => "roman_parallel",
        }
    }

    pub const fn backend_name(self) -> &'static str {
        match self {
            Self::Dense => "gbx_dense",
            Self::Roman => "gbx_roman",
            Self::RomanParallel => "gbx_roman_parallel",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GbxConfig {
    pub reducer: GbxReducerKind,
}

impl GbxConfig {
    pub const fn new(reducer: GbxReducerKind) -> Self {
        Self { reducer }
    }
}

impl Default for GbxConfig {
    fn default() -> Self {
        Self { reducer: GbxReducerKind::Dense }
    }
}
