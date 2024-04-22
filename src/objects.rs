use packed_struct::prelude::*;

// 00 	u32 	X position (* 10)
// 04 	u32 	Z position (* 10)
// 08 	s16 	Y position (* 10)
// 0A 	s8 	Width (in blocks)
// 0B 	s8 	Height (in blocks)
// 0C 	u32 	Object flags
// 10 	u32 	Child object flags
// 14 	u32 	Extended object data (used by Firebar and perhaps others)
// 18 	s8 	Object type
// 19 	s8 	Child object type
// 1A 	s16 	Link ID (assigned to pipes and rails)
// 1C 	s16 	Effect Index (-1 if none)
// 1E 	s8 	Unknown (Always -1 in sample courses - could be object's transformation ID?)
// 1F 	s8 	Child object's transformation ID (used by EditKinokoFunny)
#[derive(Debug, PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct Object {
    #[packed_field(bytes="0x00..=0x03")]
    x_position: u32,
    #[packed_field(bytes="0x04..=0x07")]
    z_position: u32,
    #[packed_field(bytes="0x08..=0x09")]
    y_position: i16,   
    #[packed_field(bytes="0x0A")]
    width: i8,
    #[packed_field(bytes="0x0B")]
    height: i8,
    #[packed_field(bytes="0x0C..=0x0F")]
    object_flags: u32,
    #[packed_field(bytes="0x10..=0x13")]
    child_object_flags: u32,
    #[packed_field(bytes="0x14..=0x17")]
    extended_object_data: u32,
    #[packed_field(bytes="0x18")]
    object_type: i8,
    #[packed_field(bytes="0x19")]
    child_object_type: i8,
    #[packed_field(bytes="0x1A..=0x1B")]
    link_id: i16,
    #[packed_field(bytes="0x1C..=0x1D")]
    effect_index: i16,
    // Could be incorrect
    #[packed_field(bytes="0x1E")]
    transformation_id: i8,
    #[packed_field(bytes="0x1F")]
    child_object_transformation_id: i8,
}