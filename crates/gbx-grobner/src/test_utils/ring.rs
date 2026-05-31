use gbx_field::fp::Fp;
use gbx_poly::order::MonomialOrder;
use gbx_poly::ring::{Ring, RingCtx, RingError};

/// Basic function for use in tests.
///
/// # Errors
///
/// Returns a field or ring construction error if the modulus or variable count
/// is invalid.
pub fn test_ring<O: MonomialOrder>(p: u32, nvars: usize, order: O) -> Result<RingCtx<Fp, O>, RingError> {
    let field = Fp::prime(p)?;
    Ring::builder()
        .field(field)
        .order(order)
        .nvars(nvars)
        .build()
}
