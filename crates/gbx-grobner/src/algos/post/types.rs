use std::time::Duration;

#[derive(Debug, Default)]
pub struct PostReduceStats {
    pub(crate) input_len: usize,
    pub(crate) after_remove_zero_len: usize,
    pub(crate) after_minimize_len: usize,
    pub(crate) after_reverse_len: usize,
    pub(crate) final_len: usize,

    pub(crate) reverse_changed: usize,
    pub(crate) selective_passes: usize,
    pub(crate) selective_changed: usize,

    pub(crate) initial_violations_after_reverse: usize,
    pub(crate) final_violations: usize,

    pub(crate) elapsed: Duration,
}
