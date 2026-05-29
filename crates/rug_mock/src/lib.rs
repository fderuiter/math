use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Rem, RemAssign};
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};
use num_integer::Integer as NumInteger;
use dashu::float::FBig;
use dashu::rational::RBig;
use dashu::base::Abs;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Integer(pub BigInt);

impl Default for Integer { fn default() -> Self { Self::new() } }

impl Integer {
    pub fn new() -> Self { Integer(BigInt::zero()) }
    pub fn from<T: Into<BigInt>>(v: T) -> Self { Integer(v.into()) }
    pub fn to_f64(&self) -> f64 { self.0.to_f64().unwrap_or(0.0) }
    pub fn to_i32(&self) -> Option<i32> { self.0.to_i32() }
    pub fn to_usize(&self) -> Option<usize> { self.0.to_usize() }
    pub fn to_u64(&self) -> Option<u64> { self.0.to_u64() }
    pub fn assign(&mut self, v: i128) { self.0 = BigInt::from(v); }
    #[allow(clippy::result_unit_err)]
    pub fn parse(s: &str) -> Result<Self, ()> { s.parse::<BigInt>().map(Integer).map_err(|_| ()) }
    #[allow(clippy::result_unit_err)]
    pub fn from_str_radix(s: &str, radix: u32) -> Result<Self, ()> { BigInt::parse_bytes(s.as_bytes(), radix).map(Integer).ok_or(()) }
    pub fn is_even(&self) -> bool { self.0.is_even() }
    pub fn is_divisible(&self, other: &Integer) -> bool { self.0.is_multiple_of(&other.0) }
    pub fn sqrt(&self) -> Self { Integer(self.0.sqrt()) }
    #[allow(clippy::result_unit_err)]
    pub fn pow_mod(&self, exp: &Integer, m: &Integer) -> Result<Self, ()> { Ok(Integer(self.0.modpow(&exp.0, &m.0))) }
    #[allow(clippy::result_unit_err)]
    pub fn invert(&self, m: &Integer) -> Result<Self, ()> {
        let ext = self.0.extended_gcd(&m.0);
        if ext.gcd == BigInt::one() { Ok(Integer(ext.x.mod_floor(&m.0))) } else { Err(()) }
    }
    pub fn abs(self) -> Self { Integer(num_traits::Signed::abs(&self.0)) }
}

impl PartialEq<i32> for Integer { fn eq(&self, other: &i32) -> bool { self.0 == BigInt::from(*other) } }
impl PartialOrd<i32> for Integer { fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&BigInt::from(*other)) } }

macro_rules! impl_binop {
    ($trt:ident, $mth:ident) => {
        impl $trt for Integer { type Output = Integer; fn $mth(self, rhs: Integer) -> Integer { Integer(self.0.$mth(rhs.0)) } }
        impl $trt<&Integer> for Integer { type Output = Integer; fn $mth(self, rhs: &Integer) -> Integer { Integer(self.0.$mth(&rhs.0)) } }
        impl $trt<Integer> for &Integer { type Output = Integer; fn $mth(self, rhs: Integer) -> Integer { Integer(self.0.clone().$mth(rhs.0)) } }
        impl $trt<&Integer> for &Integer { type Output = Integer; fn $mth(self, rhs: &Integer) -> Integer { Integer(self.0.clone().$mth(&rhs.0)) } }
    };
}
impl_binop!(Add, add);
impl_binop!(Sub, sub);
impl_binop!(Mul, mul);
impl_binop!(Div, div);
impl_binop!(Rem, rem);

impl AddAssign for Integer { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; } }
impl SubAssign for Integer { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; } }
impl MulAssign for Integer { fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0; } }
impl DivAssign for Integer { fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0; } }
impl RemAssign for Integer { fn rem_assign(&mut self, rhs: Self) { self.0 %= rhs.0; } }
impl MulAssign<&Integer> for Integer { fn mul_assign(&mut self, rhs: &Self) { self.0 *= &rhs.0; } }
impl AddAssign<&Integer> for Integer { fn add_assign(&mut self, rhs: &Self) { self.0 += &rhs.0; } }
impl SubAssign<&Integer> for Integer { fn sub_assign(&mut self, rhs: &Self) { self.0 -= &rhs.0; } }

type F = dashu::float::FBig;

#[derive(Clone, Debug)]
pub struct Float(pub F);
impl Float {
    pub fn with_val(_prec: u32, v: f64) -> Self { 
        if let Ok(f) = F::try_from(v) { Float(f) } else { Float(F::ZERO) }
    }
    pub fn abs(self) -> Self { Float(self.0.abs()) }
    pub fn round(self) -> f64 { 0.0 }
}
impl PartialEq for Float { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } }
impl PartialOrd for Float { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) } }
impl Add for Float { type Output = Self; fn add(self, rhs: Self) -> Self { Float(self.0 + rhs.0) } }
impl Sub for Float { type Output = Self; fn sub(self, rhs: Self) -> Self { Float(self.0 - rhs.0) } }
impl Mul for Float { type Output = Self; fn mul(self, rhs: Self) -> Self { Float(self.0 * rhs.0) } }
impl Div for Float { type Output = Self; fn div(self, rhs: Self) -> Self { Float(self.0 / rhs.0) } }
impl<'a> Add<&'a Float> for Float { type Output = Float; fn add(self, rhs: &'a Float) -> Float { Float(self.0 + rhs.0.clone()) } }
impl<'a> Sub<&'a Float> for Float { type Output = Float; fn sub(self, rhs: &'a Float) -> Float { Float(self.0 - rhs.0.clone()) } }
impl<'a> Mul<&'a Float> for Float { type Output = Float; fn mul(self, rhs: &'a Float) -> Float { Float(self.0 * rhs.0.clone()) } }
impl<'b> Add<&'b Float> for &Float { type Output = Float; fn add(self, rhs: &'b Float) -> Float { Float(self.0.clone() + rhs.0.clone()) } }
impl<'b> Sub<&'b Float> for &Float { type Output = Float; fn sub(self, rhs: &'b Float) -> Float { Float(self.0.clone() - rhs.0.clone()) } }
impl<'b> Mul<&'b Float> for &Float { type Output = Float; fn mul(self, rhs: &'b Float) -> Float { Float(self.0.clone() * rhs.0.clone()) } }
impl<'b> Div<&'b Float> for &Float { type Output = Float; fn div(self, rhs: &'b Float) -> Float { Float(self.0.clone() / rhs.0.clone()) } }
impl AddAssign for Float { fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; } }
impl SubAssign for Float { fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; } }
impl PartialEq<f64> for Float { 
    fn eq(&self, other: &f64) -> bool { 
        if let Ok(f) = F::try_from(*other) { self.0 == f } else { false }
    } 
}
impl PartialOrd<f64> for Float { 
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> { 
        if let Ok(f) = F::try_from(*other) { self.0.partial_cmp(&f) } else { None }
    } 
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rational(pub RBig);
impl Default for Rational { fn default() -> Self { Self::new() } }

impl Rational {
    pub fn new() -> Self { Rational(RBig::ZERO) }
    pub fn from(v: (i128, i128)) -> Self { 
        let n = dashu::integer::IBig::from(v.0);
        let d = dashu::integer::IBig::from(v.1);
        let (sign_d, mag_d) = d.into_parts();
        let mut real_n = n;
        if sign_d == dashu::integer::Sign::Negative {
            real_n = -real_n;
        }
        Rational(RBig::from_parts(real_n, mag_d))
    }
    pub fn numer(&self) -> f64 { 0.0 }
    pub fn denom(&self) -> f64 { 1.0 }
}
impl Add for Rational { type Output = Self; fn add(self, rhs: Self) -> Self { Rational(self.0 + rhs.0) } }
impl Sub for Rational { type Output = Self; fn sub(self, rhs: Self) -> Self { Rational(self.0 - rhs.0) } }
impl Mul for Rational { type Output = Self; fn mul(self, rhs: Self) -> Self { Rational(self.0 * rhs.0) } }
impl Div for Rational { type Output = Self; fn div(self, rhs: Self) -> Self { Rational(self.0 / rhs.0) } }
impl<'b> Mul<&'b Rational> for &Rational { type Output = Rational; fn mul(self, rhs: &'b Rational) -> Rational { Rational(self.0.clone() * rhs.0.clone()) } }
impl<'b> Add<&'b Rational> for &Rational { type Output = Rational; fn add(self, rhs: &'b Rational) -> Rational { Rational(self.0.clone() + rhs.0.clone()) } }
impl Mul<Rational> for &Rational { type Output = Rational; fn mul(self, rhs: Rational) -> Rational { Rational(self.0.clone() * rhs.0) } }
impl Add<Rational> for &Rational { type Output = Rational; fn add(self, rhs: Rational) -> Rational { Rational(self.0.clone() + rhs.0) } }

pub mod ops {
    pub trait Pow<Rhs> { type Output; fn pow(self, rhs: Rhs) -> Self::Output; }
    impl Pow<u32> for super::Integer { type Output = super::Integer; fn pow(self, rhs: u32) -> Self::Output { super::Integer(self.0.pow(rhs)) } }
    pub trait RemRounding<Rhs> { type Output; fn rem_euc(self, rhs: Rhs) -> Self::Output; }
    impl RemRounding<super::Integer> for super::Integer { type Output = super::Integer; fn rem_euc(self, rhs: super::Integer) -> Self::Output { super::Integer(super::NumInteger::mod_floor(&self.0, &rhs.0)) } }
    impl RemRounding<&super::Integer> for super::Integer { type Output = super::Integer; fn rem_euc(self, rhs: &super::Integer) -> Self::Output { super::Integer(super::NumInteger::mod_floor(&self.0, &rhs.0)) } }
}
