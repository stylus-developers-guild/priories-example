#![cfg_attr(not(feature = "export-abi"), no_std)]

use rand::{SeedableRng, Rng};

use rand_chacha::ChaCha8Rng;

use stylus_sdk::{alloy_primitives::*, prelude::*, storage::*};

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};

static MAX_ELEMENTS: usize = 10;

static SIZE: usize = 100;

#[derive(Clone, Debug, Copy)]
pub enum ReadingRoom {
    Stocks,
    Bookshelf,
    DeskForReading,
    Bed,
    DeskForWriting,
}

#[derive(Clone, Debug, Copy)]
pub enum FinancialRoom {
    Bed,
    LockedChest,
    DeskForReading,
    DeskForWriting,
    DeskForMeasurement,
    Altar,
    SampleCollection,
}

#[derive(Clone, Debug, Copy)]
pub enum Dormitory {
    WoodenBedstead,
    PrayerStool,
    WashBasin,
    DeskWithBell,
}

#[derive(Clone, Debug, Copy)]
pub enum Kitchen {
    Fireplace,
    Cauldron,
    TableForBread,
    TableForMeat,
    Shelf,
}

pub trait Drawable {
    fn emoji(x: &Self) -> char;
    fn layout(x: &Self) -> &'static str;
}

macro_rules! drawable {
    (
        $(
            $type:ident => {
                $($variant:ident => $emoji:literal, $layout:literal),* $(,)?
            }
        ),* $(,)?
    ) => {
        $(
            impl Drawable for $type {
                fn emoji(x: &Self) -> char {
                    match x {
                        $(
                            $type::$variant => $emoji,
                        )*
                    }
                }

                fn layout(x: &Self) -> &'static str {
                    match x {
                        $(
                            $type::$variant => $layout,
                        )*
                    }
                }
            }
        )*
    };
}

drawable! {
    ReadingRoom => {
        Stocks => 'S', "XX\nXX",
        Bookshelf => 'B', "XXX",
        DeskForReading => 'r', "XXXX",
        Bed => 'b', "XXXX",
        DeskForWriting => 'w', "XXX\nX",
    },
    FinancialRoom => {
        Bed => 'b', "XXXX",
        LockedChest => 'C', "X",
        DeskForReading => 'r', "XXXX",
        DeskForWriting => 'w', "XXX\nX",
        DeskForMeasurement => 'm', "XXXX\nX  X",
        Altar => 'A', "X",
        SampleCollection => 's', "XX",
    },
    Dormitory => {
        WoodenBedstead => 'd', "X",
        PrayerStool => 'p', "XX",
        WashBasin => 'u', "XX",
        DeskWithBell => '!', "X\nXXXX",
    },
    Kitchen => {
        Fireplace => 'f', "X X\n X \nX X",
        Cauldron => 'o', "XXX\nXXX\nXXX",
        TableForBread => 't', "XXX",
        TableForMeat => 'T', "XXXX",
        Shelf => 'h', "XX",
    }
}


struct RandIterator(usize, ChaCha8Rng);

impl Iterator for RandIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.1.random_range(0..self.0))
    }
}

macro_rules! placement {
    ($($name:ident => ($($variant:ident),* $(,)?)),* $(,)?) => {
        $(
            paste::paste! {
                struct [<$name Iterator>] {
                    rand_iter: RandIterator,
                }

                impl [<$name Iterator>] {
                    fn new(rng: ChaCha8Rng) -> Self {
                        let variants = [$($name::$variant,)*];
                        Self {
                            rand_iter: RandIterator(variants.len(), rng),
                        }
                    }
                }

                impl Iterator for [<$name Iterator>] {
                    type Item = $name;

                    fn next(&mut self) -> Option<Self::Item> {
                        let variants = [$($name::$variant,)*];
                        self.rand_iter.next().map(|index| variants[index])
                    }
                }
            }
        )*
    };
}

placement! {
    ReadingRoom => (Stocks, Bookshelf, DeskForReading, Bed, DeskForWriting),
    FinancialRoom => (
        Bed,
        LockedChest,
        DeskForReading,
        DeskForWriting,
        DeskForMeasurement,
        Altar,
        SampleCollection
    ),
    Dormitory => (
        WoodenBedstead,
        PrayerStool,
        WashBasin,
        DeskWithBell,
    ),
    Kitchen => (
        Fireplace,
        Cauldron,
        TableForBread,
        TableForMeat,
        Shelf,
    )
}

fn draw<I, const S: usize>(mut c: ChaCha8Rng, iter: I) -> [char; S]
where
    I: Iterator,
    I::Item: Drawable,
{
    let mut b = [' '; S];
    let w = S.isqrt();
    let p = c.random_range(0..S);
    for piece in iter {
        let layout = <I as Iterator>::Item::layout(&piece);
        let mut lines = 0;
        let mut i = p;
        let can_insert = layout.chars().all(|c| {
            let has_passed_line = i - p + (p % w) - (lines * w) >= w;
            match c {
                '\n' => {
                    lines += 1;
                    i = p + (lines * w);
                    true
                },
                ' ' => {
                    i += 1;
                    true
                },
                'X' => {
                    if i >= S || b[i] != ' ' || has_passed_line {
                        false
                    } else {
                        i += 1;
                        true
                    }
                },
                _ => unreachable!()
            }
        });
        if can_insert {
            let mut i = p;
            for c in layout.chars() {
                match c {
                    '\n' => {
                        i = p + w;
                    },
                    ' ' => {
                        i += 1;
                    },
                    'X' => {
                        b[i] = <I as Iterator>::Item::emoji(&piece);
                        i += 1;
                    },
                    _ => unreachable!()
                }
            }
        }
    }
    b
}

pub fn draw_reading_room<const E: usize, const S: usize>(c: ChaCha8Rng) -> [char; S] {
    draw::<_, S>(c.clone(), ReadingRoomIterator::new(c).take(E))
}

pub fn draw_financial_room<const E: usize, const S: usize>(c: ChaCha8Rng) -> [char; S] {
    draw::<_, S>(c.clone(), FinancialRoomIterator::new(c).take(E))
}

pub fn draw_dormitory<const E: usize, const S: usize>(c: ChaCha8Rng) -> [char; S] {
    draw::<_, S>(c.clone(), DormitoryIterator::new(c).take(E))
}

pub fn draw_kitchen<const E: usize, const S: usize>(c: ChaCha8Rng) -> [char; S] {
    draw::<_, S>(c.clone(), KitchenIterator::new(c).take(E))
}

#[storage]
#[entrypoint]
struct Storage {
    pub seed: StorageFixedBytes<32>,
}

#[panic_handler]
#[cfg(target_arch = "wasm32")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[public]
impl Storage {
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
