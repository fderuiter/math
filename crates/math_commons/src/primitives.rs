use std::ops::Deref;

/// A macro to define a bounded float primitive.
#[macro_export]
macro_rules! define_bounded_float {
    (
        $name:ident,
        $min:expr,
        $max:expr,
        $doc:expr
    ) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl $name {
            /// Creates a new instance, validating bounds.
            pub fn new(value: f64) -> Result<Self, String> {
                if ($min..=$max).contains(&value) && value.is_finite() {
                    Ok(Self(value))
                } else {
                    Err(format!("Value {} is outside the allowed range [{}, {}]", value, $min, $max))
                }
            }
            
            /// Retrieves the raw value.
            pub fn value(&self) -> f64 {
                self.0
            }
        }

        impl Deref for $name {
            type Target = f64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl From<$name> for f64 {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
}

/// A macro to define a strictly bounded float primitive (min < value < max).
#[macro_export]
macro_rules! define_strictly_bounded_float {
    (
        $name:ident,
        $min:expr,
        $max:expr,
        $doc:expr
    ) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl $name {
            /// Creates a new instance, validating bounds strictly.
            pub fn new(value: f64) -> Result<Self, String> {
                if value > $min && value < $max && value.is_finite() {
                    Ok(Self(value))
                } else {
                    Err(format!("Value {} is outside the strictly allowed range ({}, {})", value, $min, $max))
                }
            }
            
            /// Retrieves the raw value.
            pub fn value(&self) -> f64 {
                self.0
            }
        }

        impl Deref for $name {
            type Target = f64;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl From<$name> for f64 {
            fn from(val: $name) -> Self {
                val.0
            }
        }
    };
}

define_bounded_float!(
    UnitInterval,
    0.0,
    1.0,
    "A floating point value strictly bounded between 0.0 and 1.0 inclusive."
);

impl UnitInterval {
    /// Returns the complementary probability (1.0 - self.value()).
    pub fn complement(&self) -> f64 {
        1.0 - self.0
    }
}

define_strictly_bounded_float!(
    PositiveFloat,
    0.0,
    f64::INFINITY,
    "A floating point value strictly greater than 0.0."
);

define_bounded_float!(
    NonNegativeFloat,
    0.0,
    f64::INFINITY,
    "A floating point value bounded between 0.0 and infinity (inclusive of 0.0)."
);
