use std::{fmt::{self, Display}, marker::PhantomData, ops::{Add, AddAssign, Div, Mul, Sub}};

use num_traits::{CheckedMul, One, PrimInt};
use typenum::{IsLess, IsLessOrEqual, Unsigned, B1, U0, U100, U128, U20, U24, U3, U32, U64, U8, U90};

pub struct FixedPoint<T, Q>(T, PhantomData<Q>);

pub trait FixedInt: PrimInt {
    type BitWidth: Unsigned;
}

impl FixedInt for u8 { type BitWidth = U8; }

impl FixedInt for i64 { type BitWidth = U64; }
impl FixedInt for i128 { type BitWidth = U128; }

pub trait ValidFraction<I: FixedInt>: Unsigned {}
impl<I: FixedInt, Q: Unsigned> ValidFraction<I> for Q where Q: IsLessOrEqual<I::BitWidth, Output=B1> {}

impl<T: FixedInt, Q: Unsigned + ValidFraction<T>> FixedPoint<T, Q> {
    pub fn new(num: T) -> Self {
        Self(num << Q::USIZE, PhantomData)
    }

    pub fn raw(num: T) -> Self {
        Self(num, PhantomData)
    }
}

impl<LT, RT, OT, Q: Unsigned> Add<FixedPoint<RT, Q>> for FixedPoint<LT, Q>
where LT: Add<RT, Output = OT>,
OT: FixedInt,
Q: ValidFraction<OT> {
    type Output = FixedPoint<OT, Q>;
    fn add(self, rhs: FixedPoint<RT, Q>) -> Self::Output {
        FixedPoint::raw(self.0 + rhs.0)
    }
}

impl<LT, RT, Q: Unsigned> AddAssign<FixedPoint<RT, Q>> for FixedPoint<LT, Q>
where LT: AddAssign<RT> {
    fn add_assign(&mut self, rhs: FixedPoint<RT, Q>) {
        self.0 += rhs.0;
    }
}

impl<LT, RT, OT, Q> Sub<FixedPoint<RT, Q>> for FixedPoint<LT, Q>
where LT: Sub<RT, Output = OT>,
OT: FixedInt,
Q: ValidFraction<OT> {
    type Output = FixedPoint<OT, Q>;
    fn sub(self, rhs: FixedPoint<RT, Q>) -> Self::Output {
        FixedPoint::raw(self.0 - rhs.0)
    }
}

impl<LT, RT, OT, LQ: Unsigned, RQ: Unsigned> Mul<FixedPoint<RT, RQ>> for FixedPoint<LT, LQ>
where LT: Mul<RT, Output = OT>,
OT: FixedInt,
LQ: Add<RQ>,
<LQ as Add<RQ>>::Output: Unsigned + ValidFraction<OT> {
    type Output = FixedPoint<OT, <LQ as Add<RQ>>::Output>;
    fn mul(self, rhs: FixedPoint<RT, RQ>) -> Self::Output {
        FixedPoint::raw(self.0 * rhs.0)
    }
}



impl<LT, RT, OT, LQ: Unsigned, RQ: Unsigned> Div<FixedPoint<RT, RQ>> for FixedPoint<LT, LQ>
where LT: Div<RT, Output = OT>,
OT: FixedInt,
LQ: Sub<RQ>,
<LQ as Sub<RQ>>::Output: Unsigned + ValidFraction<OT> {
    type Output = FixedPoint<OT, <LQ as Sub<RQ>>::Output>;
    fn div(self, rhs: FixedPoint<RT, RQ>) -> Self::Output {
        FixedPoint::raw(self.0 / rhs.0)
    }
}

impl<F: FixedInt, Q> FixedPoint<F, Q> {
    pub fn cast<T>(self) -> FixedPoint<T, Q> 
    where T: FixedInt,
        F::BitWidth: IsLessOrEqual<T::BitWidth, Output = B1>,
        T: From<F> {
        FixedPoint(<T as From<F>>::from(self.0), PhantomData)
    }
}

#[test]
fn test() {
    let fixed = FixedPoint::<i128, U90>::new(3000000000) / FixedPoint::<i128, U0>::new(10);
    let result = fixed + (FixedPoint::<i128, U90>::new(55) / FixedPoint::<i128, U0>::new(100));
    panic!("{:b} = {}", result, result)
}

impl<T: PrimInt + Display + std::fmt::Binary, Q: Unsigned> Display for FixedPoint<T, Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_digits = T::zero().count_zeros() as usize;
        let whole_digits = max_digits - Q::USIZE;
        let mut digits = vec![0u32 ; max_digits];

        let abs = if self.0 < T::from(0).unwrap() {
            !self.0 + T::one()
        } else {
            self.0
        };
        
        let ten = T::from(10).unwrap();
        let mut val = abs >> Q::USIZE;
        for digit in digits.iter_mut().rev().skip(Q::USIZE) {
            *digit = (val % ten).to_u32().unwrap();
            val = val / ten;
        }

        let mut val = abs << whole_digits;
        val = val.unsigned_shr(whole_digits as u32);
        for digit in digits.iter_mut().skip(whole_digits) {
            val = val.checked_mul(&ten).unwrap_or_else(|| { println!("Multiply {}", val); ten });
            *digit = val.unsigned_shr(Q::USIZE as u32).to_u32().unwrap();
            val = val.unsigned_shl(whole_digits as u32).unsigned_shr(whole_digits as u32);
        }

        if self.0 < T::from(0).unwrap() {
            write!(f, "-")?;
        }

        for d in digits.iter().take(whole_digits).skip_while(|d| **d == 0) {
            write!(f, "{}", d)?;
        }

        write!(f, ".")?;
        for d in digits.iter().skip(whole_digits) {
            write!(f, "{}", d)?;
        }

        Ok(())
    }
}

impl<T: FixedInt + fmt::Binary, Q: Unsigned> fmt::Binary for FixedPoint<T, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02$b}.{:03$b}", self.0.unsigned_shr(Q::U32), self.0 & ((T::one() << Q::USIZE) - T::one()), T::BitWidth::USIZE - Q::USIZE, Q::USIZE)
    }
}
