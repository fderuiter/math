//! # Engineering Utilities
//!
//! Practical formulas and calculations for hardware engineering and reliability analysis.
//!
//! This module collects utility functions that don't fit into the larger simulation frameworks
//! but are essential for "back-of-the-napkin" estimation in embedded systems and manufacturing.

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
/// let throughput = uart_effective_throughput(9600, 8, 10);
/// assert_eq!(throughput, 7680.0);
/// ```
pub fn uart_effective_throughput(baud_rate: u32, data_bits: u32, total_bits: u32) -> f64 {
    if total_bits == 0 {
        return 0.0;
    }
    (baud_rate as f64 * data_bits as f64) / total_bits as f64
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
        let eff = uart_effective_throughput(9600, 8, 10);
        assert_eq!(eff, 7680.0);
    }
}
