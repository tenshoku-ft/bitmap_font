#![no_std]

/// raw binary of font
pub const FONT:[u8;2048]=*include_bytes!(concat!(env!("OUT_DIR"),"/ascii.bin"));
