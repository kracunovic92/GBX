use gbx_grobner::GbDisplay;

use super::types::{GbxPoly, GrevlexRing};

pub fn print_poly_preview(title: &str, ring: &GrevlexRing, polys: &[GbxPoly], vars: &[String], limit: usize) {
    let shown = limit.min(polys.len());
    println!("--- {title} (first {shown}) ---");

    for (i, poly) in polys.iter().take(shown).enumerate() {
        println!("#{i}: {}", gbx_poly::pretty_str!(ring, poly, vars));
    }
}

pub fn print_basis(ring: &GrevlexRing, basis: &[GbxPoly], vars: &[String], dump_basis: bool) {
    if dump_basis {
        println!("--- grobner basis (pretty) ---");
        let disp = GbDisplay::pretty_lines(ring, basis, vars);
        print!("{disp}");

        println!("--- grobner basis (tuple) ---");
        let disp = GbDisplay::tuple_lines(ring, basis, vars);
        print!("{disp}");
    } else {
        let shown = 5usize.min(basis.len());
        println!("--- grobner basis preview (first {shown}) ---");
        for (i, poly) in basis.iter().take(shown).enumerate() {
            println!("#{i}: {}", gbx_poly::pretty_str!(ring, poly, vars));
        }
    }
}
