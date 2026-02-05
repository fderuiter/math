/// Macro to implement common arithmetic operations for compartmental states.
#[macro_export]
macro_rules! impl_compartmental_ops {
    ($type:ty, $($field:ident),+) => {
        impl std::ops::Add for $type {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($field: self.$field + rhs.$field),+
                }
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)+
            }
        }

        impl std::ops::Mul<f64> for $type {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self {
                Self {
                    $($field: self.$field * scalar),+
                }
            }
        }

        impl std::ops::MulAssign<f64> for $type {
            fn mul_assign(&mut self, scalar: f64) {
                $(self.$field *= scalar;)+
            }
        }

        impl crate::pure_math::analysis::ode::VectorOperations for $type {
            fn scale_add(&mut self, other: &Self, scale: f64) {
                $(self.$field += other.$field * scale;)+
            }

            fn copy_from(&mut self, other: &Self) {
                *self = *other;
            }
        }
    };
}
