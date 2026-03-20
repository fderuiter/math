//! # Engineering Utilities
//!
//! Practical formulas and calculations for hardware engineering and reliability analysis.
//!
//! This module collects utility functions that don't fit into the larger simulation frameworks
//! but are essential for "back-of-the-napkin" estimation in embedded systems and manufacturing.

use super::engineering_error::EngineeringError;

/// A baud rate in symbols per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaudRate(u32);

impl BaudRate {
    /// Creates a new `BaudRate`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// The number of payload data bits in a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataBits(u32);

impl DataBits {
    /// Creates a new `DataBits`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// The total number of bits in a frame (start, data, parity, stop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TotalBits(u32);

impl TotalBits {
    /// Creates a new `TotalBits`.
    ///
    /// # Panics
    /// Panics if `value` is 0, since a frame cannot have 0 bits.
    pub fn new(value: u32) -> Self {
        if value == 0 {
            panic!("TotalBits must be greater than 0");
        }
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Calculates the Effective UART Throughput using type-safe inputs.
///
/// In real-world communications, the "Baud Rate" is not the actual data transfer rate.
/// Overhead from start bits, stop bits, and parity bits reduces the effective throughput.
///
/// $$ \text{Eff. Speed} = \frac{\text{Baud Rate} \times \text{Data Bits}}{\text{Total Bits per Frame}} $$
///
/// # Arguments
///
/// * `baud_rate` - [`BaudRate`]
/// * `data_bits` - [`DataBits`]
/// * `total_bits` - [`TotalBits`]
///
/// # Returns
///
/// * `f64` - Effective bits per second.
///
/// # Example
///
/// ```
/// use math_explorer::applied::engineering::{calculate_uart_throughput, BaudRate, DataBits, TotalBits};
///
/// // Standard 9600 8N1
/// let throughput = calculate_uart_throughput(
///     BaudRate::new(9600),
///     DataBits::new(8),
///     TotalBits::new(10),
/// );
/// assert_eq!(throughput, 7680.0);
/// ```
pub fn calculate_uart_throughput(
    baud_rate: BaudRate,
    data_bits: DataBits,
    total_bits: TotalBits,
) -> f64 {
    (baud_rate.value() as f64 * data_bits.value() as f64) / total_bits.value() as f64
}

/// Calculates the Effective UART Throughput.
///
/// In real-world communications, the "Baud Rate" is not the actual data transfer rate.
/// Overhead from start bits, stop bits, and parity bits reduces the effective throughput.
///
/// $$ \text{Eff. Speed} = \frac{\text{Baud Rate} \times \text{Data Bits}}{\text{Total Bits per Frame}} $$
///
/// # Arguments
///
/// * `baud_rate` - Symbols per second (e.g., 9600, 115200).
/// * `data_bits` - Number of payload bits (usually 8).
/// * `total_bits` - Total bits including start, stop, and parity (usually 10: 1 start + 8 data + 1 stop).
///
/// # Returns
///
/// * `f64` - Effective bits per second.
///
/// # Example
///
/// ```
/// use math_explorer::applied::engineering::uart_effective_throughput;
///
/// // Standard 9600 8N1
/// #[allow(deprecated)]
/// let throughput = uart_effective_throughput(9600, 8, 10);
/// assert_eq!(throughput, 7680.0);
/// ```
#[deprecated(
    since = "0.2.0",
    note = "Use `calculate_uart_throughput` with semantic types instead to prevent primitive obsession."
)]
pub fn uart_effective_throughput(baud_rate: u32, data_bits: u32, total_bits: u32) -> f64 {
    if total_bits == 0 {
        return 0.0;
    }
    (baud_rate as f64 * data_bits as f64) / total_bits as f64
}

/// The cutting speed ($V$) in meters per minute (m/min).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CuttingSpeed(f64);

impl CuttingSpeed {
    /// Creates a new `CuttingSpeed`.
    ///
    /// # Panics
    /// Panics if `value` is negative or zero.
    pub fn new(value: f64) -> Self {
        if value <= 0.0 {
            panic!("CuttingSpeed must be positive");
        }
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A material-specific constant ($C$) for Taylor's equation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaterialConstant(f64);

impl MaterialConstant {
    /// Creates a new `MaterialConstant`.
    ///
    /// # Panics
    /// Panics if `value` is negative.
    pub fn new(value: f64) -> Self {
        if value < 0.0 {
            panic!("MaterialConstant must be non-negative");
        }
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The exponent ($n$) dependent on tool material for Taylor's equation.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ToolExponent(f64);

impl ToolExponent {
    /// Creates a new `ToolExponent`.
    ///
    /// # Panics
    /// Panics if `value` is zero, to prevent division by zero in the exponent.
    pub fn new(value: f64) -> Self {
        if value == 0.0 {
            panic!("ToolExponent cannot be zero");
        }
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// The estimated tool life in minutes.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ToolLife(f64);

impl ToolLife {
    /// Creates a new `ToolLife`.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the inner value.
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Estimates Fault Lifetime using a Modified Taylor's Equation.
///
/// Used in predictive maintenance (Prognostics) for CNC machines.
/// It models the relationship between cutting speed and tool life.
///
/// The standard Taylor's Tool Life Equation is $V T^n = C$.
/// This implementation calculates $T$ (Time) given $V$ (Velocity).
///
/// $$ T = \left( \frac{C}{V} \right)^{1/n} $$
///
/// # Arguments
///
/// * `cutting_speed` ($V$) - The cutting speed in m/min.
/// * `constant_c` ($C$) - A material-specific constant (e.g., depends on the workpiece and tool).
/// * `exponent_n` ($n$) - Exponent depending on tool material (e.g., 0.1 for HSS, 0.25 for Carbide).
///
/// # Returns
///
/// * `f64` - Estimated Tool life ($T$) in minutes.
///
/// # Example
///
/// ```
/// use math_explorer::applied::engineering::taylor_tool_life;
///
/// // High Speed Steel (HSS) tool cutting steel
/// let v = 40.0; // 40 m/min
/// let c = 100.0;
/// let n = 0.1;
///
/// let life = taylor_tool_life(v, c, n);
/// // Expecting (100/40)^(1/0.1) = 2.5^10 = 9536.7...
/// assert!(life > 9000.0);
/// ```
/// Estimates Fault Lifetime using a Modified Taylor's Equation and type-safe inputs.
///
/// Used in predictive maintenance (Prognostics) for CNC machines.
/// It models the relationship between cutting speed and tool life.
///
/// The standard Taylor's Tool Life Equation is $V T^n = C$.
/// This implementation calculates $T$ (Time) given $V$ (Velocity).
///
/// $$ T = \left( \frac{C}{V} \right)^{1/n} $$
///
/// # Arguments
///
/// * `cutting_speed` ($V$) - [`CuttingSpeed`] in m/min.
/// * `constant_c` ($C$) - [`MaterialConstant`].
/// * `exponent_n` ($n$) - [`ToolExponent`].
///
/// # Returns
///
/// * [`ToolLife`] - Estimated Tool life ($T$) in minutes.
///
/// # Example
///
/// ```
/// use math_explorer::applied::engineering::{calculate_taylor_tool_life, CuttingSpeed, MaterialConstant, ToolExponent};
///
/// // High Speed Steel (HSS) tool cutting steel
/// let life = calculate_taylor_tool_life(
///     CuttingSpeed::new(40.0),
///     MaterialConstant::new(100.0),
///     ToolExponent::new(0.1),
/// );
/// // Expecting (100/40)^(1/0.1) = 2.5^10 = 9536.7...
/// assert!(life.value() > 9000.0);
/// ```
pub fn calculate_taylor_tool_life(
    cutting_speed: CuttingSpeed,
    constant_c: MaterialConstant,
    exponent_n: ToolExponent,
) -> ToolLife {
    let result = (constant_c.value() / cutting_speed.value()).powf(1.0 / exponent_n.value());
    ToolLife::new(result)
}

/// Estimates Fault Lifetime using a Modified Taylor's Equation.
///
/// Used in predictive maintenance (Prognostics) for CNC machines.
/// It models the relationship between cutting speed and tool life.
///
/// The standard Taylor's Tool Life Equation is $V T^n = C$.
/// This implementation calculates $T$ (Time) given $V$ (Velocity).
///
/// $$ T = \left( \frac{C}{V} \right)^{1/n} $$
///
/// # Arguments
///
/// * `cutting_speed` ($V$) - The cutting speed in m/min.
/// * `constant_c` ($C$) - A material-specific constant (e.g., depends on the workpiece and tool).
/// * `exponent_n` ($n$) - Exponent depending on tool material (e.g., 0.1 for HSS, 0.25 for Carbide).
///
/// # Returns
///
/// * `f64` - Estimated Tool life ($T$) in minutes.
///
/// # Example
///
/// ```
/// use math_explorer::applied::engineering::taylor_tool_life;
///
/// // High Speed Steel (HSS) tool cutting steel
/// let v = 40.0; // 40 m/min
/// let c = 100.0;
/// let n = 0.1;
///
/// #[allow(deprecated)]
/// let life = taylor_tool_life(v, c, n);
/// // Expecting (100/40)^(1/0.1) = 2.5^10 = 9536.7...
/// assert!(life > 9000.0);
/// ```
#[deprecated(
    since = "0.2.0",
    note = "Use `calculate_taylor_tool_life` with semantic types instead to prevent primitive obsession."
)]
pub fn taylor_tool_life(cutting_speed: f64, constant_c: f64, exponent_n: f64) -> f64 {
    if cutting_speed <= 0.0 || exponent_n == 0.0 {
        return 0.0;
    }
    (constant_c / cutting_speed).powf(1.0 / exponent_n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uart() {
        // 9600 baud, 8N1 (1 start + 8 data + 1 stop = 10 bits)
        // 9600 * 0.8 = 7680
        #[allow(deprecated)]
        let eff = uart_effective_throughput(9600, 8, 10);
        assert_eq!(eff, 7680.0);
    }

    #[test]
    fn test_calculate_uart_throughput() {
        let eff =
            calculate_uart_throughput(BaudRate::new(9600), DataBits::new(8), TotalBits::new(10));
        assert_eq!(eff, 7680.0);
    }

    #[test]
    fn test_calculate_taylor_tool_life() {
        let life = calculate_taylor_tool_life(
            CuttingSpeed::new(40.0),
            MaterialConstant::new(100.0),
            ToolExponent::new(0.1),
        );
        assert!(life.value() > 9000.0);
    }
}
