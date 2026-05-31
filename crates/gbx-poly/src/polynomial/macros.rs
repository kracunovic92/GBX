/// Builds a vector of terms.
#[macro_export]
macro_rules! terms {
    ($( ($c:expr, [$($e:expr),* $(,)?]) ),* $(,)?) => {{
        vec![
            $(
                $crate::term::Term::new(
                    ::core::convert::Into::into($c),
                    $crate::monomial::Monomial::from_slice(&[$($e as u32),*]),
                )
            ),*
        ]
    }};

    ($( ($c:expr, $m:expr) ),* $(,)?) => {{
        vec![
            $(
                $crate::term::Term::new(
                    ::core::convert::Into::into($c),
                    $m,
                )
            ),*
        ]
    }};
}

/// Builds a polynomial in the given ring context.
#[macro_export]
macro_rules! poly {
    ($ctx:expr; $( ($c:expr, [$($e:expr),* $(,)?]) ),* $(,)?) => {{
        let __ctx = $ctx;

        let __terms = vec![
            $(
                $crate::term::Term::new(
                    $crate::ring::FieldCtx::elem(&__ctx.field, $c as u32),
                    $crate::monomial::Monomial::from_slice(&[$($e as u32),*]),
                )
            ),*
        ];

        $crate::polynomial::Polynomial::from_terms_in(__ctx, __terms)
    }};

    ($ctx:expr; $( ($c:expr, $m:expr) ),* $(,)?) => {{
        let __ctx = $ctx;

        let __terms = vec![
            $(
                $crate::term::Term::new(
                    $crate::ring::FieldCtx::elem(&__ctx.field, $c as u32),
                    $m,
                )
            ),*
        ];

        $crate::polynomial::Polynomial::from_terms_in(__ctx, __terms)
    }};
}

/// Builds a polynomial from an existing term vector.
#[macro_export]
macro_rules! poly_terms {
    ($ctx:expr; $terms:expr) => {{ $crate::polynomial::Polynomial::from_terms_in($ctx, $terms) }};
}
