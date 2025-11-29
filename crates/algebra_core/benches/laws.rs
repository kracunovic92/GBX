#![allow(missing_docs)]

use algebra_core::{AddGroup, Additive, Zero};
use criterion::{criterion_group, criterion_main, Criterion};

fn check_add_laws_i32(values: &[i32]) {
    // Associativity
    for &a in values {
        for &b in values {
            for &c in values {
                let lhs = a.add(b).add(c);
                let rhs = a.add(b.add(c));
                assert_eq!(lhs, rhs);
            }
        }
    }

    // Identity
    let zero = i32::zero();
    for &a in values {
        assert_eq!(zero.add(a), a);
        assert_eq!(a.add(zero), a);
    }

    // Inverse
    for &a in values {
        let neg = AddGroup::neg(a);
        assert_eq!(a.add(neg), zero);
        assert_eq!(neg.add(a), zero);
    }
}

fn bench_add_laws(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_laws_i32");

    for &n in &[5usize, 7, 9] {
        group.bench_with_input(format!("n={n}"), &n, |b, &n| {
            let values: Vec<i32> = (-(n as i32)..=(n as i32)).collect();
            b.iter(|| check_add_laws_i32(&values));
        });
    }

    group.finish();
}

criterion_group!(laws, bench_add_laws);
criterion_main!(laws);
