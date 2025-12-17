use crate::complex::{Complex, DComplex};

fn newton_method<F>(mut f: F, z: Complex) -> Complex
where
    F: FnMut(DComplex) -> DComplex,
{
    let z_dual = DComplex::var(z);
    let f_z = f(z_dual);

    z - (f_z.val / f_z.der)
}
