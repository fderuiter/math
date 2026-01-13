//! Communications and Data Transfer Utilities.

/// Calculates the Effective Throughput of a UART connection.
///
/// $$ \text{Eff. Speed} = \frac{\text{Baud Rate} \times \text{Data Bits}}{\text{Total Bits per Frame}} $$
///
/// Typical frame: 1 Start bit + 8 Data bits + 0 Parity + 1 Stop bit = 10 bits.
///
/// # Arguments
/// * `baud_rate` - Transmission speed in symbols/second.
/// * `data_bits` - Number of payload bits (usually 8).
/// * `total_bits_per_frame` - Total bits including overhead (usually 10).
pub fn uart_effective_throughput(baud_rate: u32, data_bits: u32, total_bits_per_frame: u32) -> f64 {
    (baud_rate as f64 * data_bits as f64) / total_bits_per_frame as f64
}
