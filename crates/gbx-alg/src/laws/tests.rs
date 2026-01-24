#![cfg(feature = "laws")]
#![allow(unused_imports)]
use crate::laws;

#[test]
fn laws_compile_for_primitives() {
    let elems = [0i32, 1i32, 2i32, -3i32];
    laws::check_ring(&elems);

    let elems_u = [0u32, 1u32, 2u32, 3u32];
    laws::check_semiring(&elems_u);
}
