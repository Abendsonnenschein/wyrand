#![no_std]

use core::default::Default;
use core::ffi::c_void;
use core::mem::size_of;

#[link(name = "Security", kind = "framework")]
extern "C" {
    fn SecRandomCopyBytes(rnd: *const c_void, count: usize, bytes: *mut u8) -> u32;
}

fn get_entropy(out: &mut [u8]) -> bool {
    unsafe { SecRandomCopyBytes(core::ptr::null(), out.len(), out.as_mut_ptr()) == 0 }
}

pub struct WyRand {
    seed: u64,
}

impl WyRand {
    pub fn new() -> Self {
        let mut entropy: [u8; size_of::<u64>()] = Default::default();
        get_entropy(&mut entropy);

        Self {
            seed: u64::from_ne_bytes(entropy),
        }
    }

    pub fn rand(&mut self) -> [u8; 16] {
        self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);

        let a: u128 = (self.seed ^ 0xe7037ed1a0b428db) as u128;
        let b: u128 = (self.seed as u128).wrapping_mul(a);
        let c: [u8; 16] = b.to_ne_bytes();

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_duplicated() {
        let mut rng: WyRand = WyRand::new();

        let rs0: [u8; 16] = rng.rand();
        let rs1: [u8; 16] = rng.rand();

        assert_ne!(rs0, rs1);
    }
}
