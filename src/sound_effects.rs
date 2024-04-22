use packed_struct::prelude::*;

#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="msb", size_bytes="8")]
pub struct SoundEffect {
    // pub sound_type: u8,
    // pub variation: u8,
    // pub x_position: u8,
    // pub y_position: u8,
    pub unknown: u32,
}