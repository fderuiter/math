//! Engineering Formulas and Utilities.
//!
//! Miscellaneous engineering calculations for embedded systems and reliability.

/// Calculates the Effective UART Throughput.
///
/// $$ \text{Eff. Speed} = \frac{\text{Baud Rate} \times \text{Data Bits}}{\text{Total Bits per Frame}} $$
///
/// # Arguments
///
/// * `baud_rate` - Symbols per second.
/// * `data_bits` - Number of payload bits (usually 8).
/// * `total_bits` - Total bits including start, stop, and parity (usually 10: 1 start + 8 data + 1 stop).
///
/// # Returns
///
/// * `f64` - Effective bits per second.
pub fn uart_effective_throughput(baud_rate: u32, data_bits: u32, total_bits: u32) -> f64 {
    if total_bits == 0 {
        return 0.0;
    }
    (baud_rate as f64 * data_bits as f64) / total_bits as f64
}

/// Estimates Fault Lifetime using a Modified Taylor's Equation.
///
/// Used in CNC machine fault prognostics.
///
/// Note: The prompt simply says "Taylor's Equation (Modified)" and "Fault lifetime: inversely proportional to tool life".
/// Standard Taylor's Tool Life Equation is $V T^n = C$.
/// Modified usually adds feed/depth: $T = C / (V^p f^q d^r)$.
///
/// Without explicit formula details beyond "inversely proportional", we implement the standard form $T = (C/V)^{1/n}$.
///
/// $$ T = \left( \frac{C}{V} \right)^{1/n} $$
///
/// # Arguments
///
/// * `cutting_speed` ($V$) - The cutting speed.
/// * `constant_c` ($C$) - Material/tool constant.
/// * `exponent_n` ($n$) - Exponent depending on tool material.
///
/// # Returns
///
/// * `f64` - Tool life ($T$).
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
