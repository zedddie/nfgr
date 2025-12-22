use crate::math::complex::{Complex, DComplex};

pub fn newton_stream<F>(f: F, start: Complex) -> impl Iterator<Item = Complex>
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    std::iter::successors(Some(start), move |&z| {
        let y = f(DComplex::var(z));
        Some(z - (y.val / y.der))
    })
}
pub fn calc_root<F>(start_z: Complex, f: F, max_iter: usize) -> (Complex, usize)
where
    F: Fn(DComplex) -> DComplex + Clone,
{
    newton_stream(f, start_z)
        .enumerate()
        .scan(start_z, |prev_z, (i, curr_z)| {
            let converged = (curr_z - *prev_z).n_sqr() < 1e-12;
            let limit = i >= max_iter;

            if converged || limit {
                None
            } else {
                *prev_z = curr_z;
                Some((curr_z, i))
            }
        })
        .last()
        .unwrap_or((start_z, 0))
}
fn closest_root(z: Complex, roots: &[Complex]) -> Option<usize> {
    roots
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (z - **a)
                .n_sqr()
                .partial_cmp(&(z - **b).n_sqr())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(index, _)| index)
}
