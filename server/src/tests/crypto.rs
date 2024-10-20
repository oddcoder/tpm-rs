use rand_core::{
    impls::{next_u32_via_fill, next_u64_via_fill},
    CryptoRng, RngCore, SeedableRng,
};

/// Implements fake cryptographic operations to be used in unit testing.
pub struct FakeCrypto {
    counter: u8,
}

impl FakeCrypto {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl SeedableRng for FakeCrypto {
    type Seed = [u8; 1];

    fn from_seed(seed: Self::Seed) -> Self {
        Self { counter: seed[0] }
    }
}

impl RngCore for FakeCrypto {
    fn fill_bytes(&mut self, buffer: &mut [u8]) {
        for i in buffer {
            self.counter = self.counter.wrapping_add(1);
            *i = self.counter;
        }
    }

    fn next_u32(&mut self) -> u32 {
        next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        next_u64_via_fill(self)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for FakeCrypto {}

#[test]
fn test_get_random_bytes() {
    let mut crypto = FakeCrypto::new();
    let mut buffer = [0; 10];

    crypto.fill_bytes(&mut buffer);
    assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    crypto.fill_bytes(&mut buffer);
    assert_eq!(buffer, [11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
}
