#![cfg_attr(target_arch = "wasm32", no_main, no_std)]

use rand::Rng;

use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};

use libpriories::*;

#[cfg(all(not(target_arch = "wasm32"), not(feature = "export-abi")))]
fn main() {
    let mut s = Storage::from(&stylus_sdk::testing::vm::TestVM::new());
    println!("{}", s.generate())
}

#[cfg(feature = "export-abi")]
fn main() {
    libpriories::print_from_args();
}
