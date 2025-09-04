#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

use rand::Rng;

use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

use libpriories::*;

extern crate alloc;

#[cfg(target_arch = "wasm32")]
use alloc::{string::String, vec, vec::Vec};

static MAX_ELEMENTS: usize = 10;

static SIZE: usize = 100;

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[storage]
#[entrypoint]
struct Storage {
    pub seed: StorageFixedBytes<32>,
}

#[public]
impl Storage {
    #[constructor]
    pub fn init(&mut self, initial: FixedBytes<32>) {
        self.seed.set(initial);
    }

    pub fn generate(&mut self) -> String {
        let mut c = ChaCha8Rng::from_seed(*self.seed.get());
        // This randomness strategy isn't perfect!
        let s = match c.random_range(0..5) {
            0 => draw_reading_room::<MAX_ELEMENTS, SIZE>(c),
            1 => draw_financial_room::<MAX_ELEMENTS, SIZE>(c),
            2 => draw_dormitory::<MAX_ELEMENTS, SIZE>(c),
            3 => draw_kitchen::<MAX_ELEMENTS, SIZE>(c),
            _ => unreachable!(),
        };
        s.iter().collect()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut s = Storage::from(&stylus_sdk::testing::vm::TestVM::new());
    println!("{}", s.generate())
}
