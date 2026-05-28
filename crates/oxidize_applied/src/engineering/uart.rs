use super::error::EngineeringError;

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
    /// # Errors
    /// Returns `EngineeringError` if `value` is 0, since a frame cannot have 0 bits.
    pub fn new(value: u32) -> Result<Self, EngineeringError> {
        if value == 0 {
            return Err(EngineeringError::InvalidParameter {
                name: "TotalBits".to_string(),
                value: 0.0,
            });
        }
        Ok(Self(value))
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
/// use oxidize_applied::engineering::{calculate_uart_throughput, BaudRate, DataBits, TotalBits};
///
/// // Standard 9600 8N1
/// let throughput = calculate_uart_throughput(
///     BaudRate::new(9600),
///     DataBits::new(8),
///     TotalBits::new(10).unwrap(),
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
/// use oxidize_applied::engineering::uart_effective_throughput;
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
        let eff = calculate_uart_throughput(
            BaudRate::new(9600),
            DataBits::new(8),
            TotalBits::new(10).unwrap(),
        );
        assert_eq!(eff, 7680.0);
    }
}
