use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// A standard deterministic random number generator provider for the `oxidize` framework.
/// This enforces the "No-Entropy" policy by relying exclusively on explicit seeds
/// and preventing direct OS-level random calls via getrandom.
#[derive(Clone, Debug)]
pub struct OxidizeRng {
    inner: ChaCha8Rng,
}

impl OxidizeRng {
    /// Creates a new deterministic RNG from a 64-bit seed.
    /// This ensures identical stochastic behavior across platforms (Native and Web).
    pub fn new(seed: u64) -> Self {
        Self {
            inner: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl Default for OxidizeRng {
    fn default() -> Self {
        Self::new(0x0001_D1CE_CAFE_BABE)
    }
}

impl RngCore for OxidizeRng {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.inner.try_fill_bytes(dest)
    }
}
