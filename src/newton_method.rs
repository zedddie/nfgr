use crate::complex::{Complex, DComplex};

fn newton_stream<F>(mut f: F, start: Complex) -> impl Iterator<Item = Complex>
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    std::iter::successors(Some(start), move |&z| {
        let y = f(DComplex::var(z));
        Some(z - (y.val / y.der))
    })
}
