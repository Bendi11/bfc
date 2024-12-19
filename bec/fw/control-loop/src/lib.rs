use std::ops::Neg;

use nalgebra::{ClosedAddAssign, ClosedMulAssign, Matrix2, RowVector2, Scalar, Vector2};
use num_traits::{ConstOne, Inv, One, Zero};

pub trait MatrixVectorMultiplicable: ClosedMulAssign + ClosedAddAssign + Scalar + One + Zero {}
impl<T: ClosedMulAssign + ClosedAddAssign + Scalar + One + Zero> MatrixVectorMultiplicable for T {}

pub trait SquareRootState<T> {
    fn sqrt(&self, val: T) -> T;
}

pub trait TrigonometryState<T> {
    fn sin_cos(&self, val: T) -> (T, T);
}

pub struct AlphaBetaGamma<T>(RowVector2<T>);

impl<T> AlphaBetaGamma<T> {
    pub fn precompute<S>(sqrtctx: &S) -> Self
    where T: Inv<Output = T> + ClosedAddAssign + ConstOne + Clone,
    S: SquareRootState<T> {
        let inv_sqrt3 = sqrtctx.sqrt(T::ONE + T::ONE + T::ONE).inv();
        let beta_vec = RowVector2::new(inv_sqrt3.clone(), inv_sqrt3.clone() + inv_sqrt3);
        Self(beta_vec)
    }

    pub fn apply(&self, ab: Vector2<T>) -> Vector2<T>
    where T: MatrixVectorMultiplicable {
        Vector2::new(
            ab.x.clone(),
            (self.0.clone() * ab).into_scalar()
        )
    }
}

pub fn dqz<T, S>(trig: &S, alpha_beta: Vector2<T>, theta: T) -> Vector2<T>
where S: TrigonometryState<T>,
T: MatrixVectorMultiplicable + Neg<Output = T> + Clone {
    let (sin, cos) = trig.sin_cos(theta);

    let rotation = Matrix2::new(
        cos.clone(), -sin.clone(),
        sin,        cos
    );

    rotation * alpha_beta
}

#[cfg(feature="std")]
impl<F: num_traits::Float> SquareRootState<F> for () {
     fn sqrt(&self, val: F) -> F {
         val.sqrt()
     }
}

#[cfg(feature="std")]
impl<F: num_traits::Float> TrigonometryState<F> for () {
    fn sin_cos(&self, val: F) -> (F, F) {
        val.sin_cos()
    }
}
