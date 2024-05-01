use std::io::{Cursor, Seek, Write};

use chrono::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use packed_struct::prelude::*;

use crate::{objects::Object, sound_effects::SoundEffect, Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum GameMode {
    #[default]
    SuperMarioBros,
    Mario3,
    MarioWorld,
    NewSuperMarioBrosU,
}

impl PackedStruct for GameMode {
    type ByteArray = [u8; 2];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        Ok(match self {
            GameMode::SuperMarioBros => [b'M', b'1'],
            GameMode::Mario3 => [b'M', b'3'],
            GameMode::MarioWorld => [b'M', b'W'],
            GameMode::NewSuperMarioBrosU => [b'W', b'U'],
        })
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        Ok(match src {
            [b'M', b'1'] => GameMode::SuperMarioBros,
            [b'M', b'3'] => GameMode::Mario3,
            [b'M', b'W'] => GameMode::MarioWorld,
            [b'W', b'U'] => GameMode::NewSuperMarioBrosU,
            _ => return Err(packed_struct::PackingError::InvalidValue),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum CourseTheme {
    #[default]
    Overworld = 0,
    Underground = 1,
    Castle = 2,
    Airship = 3,
    Water = 4,
    GhostHouse = 5,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum AutoScroll {
    #[default]
    None = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
}

// 00 	u64 	Assumed to be a version number. Always 0xB (11) right now
// 08 	u32 	Checksum. Standard CRC32 of the entire file from offset 0x10 onwards.
// 0C 	padding 	4 unused bytes
// 10 	u16 	Creation year
// 12 	u8 	Creation month
// 13 	u8 	Creation day
// 14 	u8 	Creation hour
// 15 	u8 	Creation minute
// 16 	u8 	Unknown
// 17 	u8 	Unknown
// 18 	u64 	Unknown
// 20 	u8 	Unknown
// 21 	padding 	7 unused bytes
// 28 	u16[0x21] 	UCS-2 course name - 32 characters long plus zero terminator
// 6A 	char[2] 	Game mode ('M1', 'M3', 'MW', 'WU')
// 6C 	u8 	Unknown
// 6D 	u8 	Course theme (0 = overworld, 1 = underground, 2 = castle, 3 = airship, 4 = water, 5 = ghost house)
// 6E 	u8 	Unknown
// 6F 	u8 	Unknown
// 70 	u16 	Time limit
// 72 	u8 	Autoscroll (0 = none, 1 = slow, 2 = medium, 3 = fast)
// 73 	u8 	Flags
// 74 	u32 	Width
// 78 	u8[0x60] 	Mii data
// D8 	u32 	Unknown
// DC 	u32 	Unknown
// E0 	padding 	0xC unused bytes
// EC 	u32 	Object count
// F0 	obj_t[2600] 	Objects (note that the full size is reserved even if the course has less than 2600 objects)
// 145F0 	effect_t[300] 	Sound effects
// 14F50 	padding 	0xB0 unused bytes

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Level {
    pub version: u64,
    pub creation_time: chrono::NaiveDateTime,
    pub level_name: String,
    pub game_mode: GameMode,
    pub course_theme: CourseTheme,
    pub time_limit: u16,
    pub auto_scroll: AutoScroll,
    pub flags: u8,
    pub width: u32,
    pub mii_data: [u8; 0x60],
    pub objects: Vec<Object>,
    pub sound_effects: Vec<SoundEffect>,
}

impl PackedStruct for Level {
    type ByteArray = [u8; 0x15000];

    fn pack(&self) -> packed_struct::PackingResult<Self::ByteArray> {
        let mut bytes = [0; 0x15000];
        let mut cursor = Cursor::new(&mut bytes[..]);

        // 00 	u64 	Assumed to be a version number. Always 0xB (11) right now
        cursor
            .write_all(&self.version.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 08 	u32 	Checksum. Standard CRC32 of the entire file from offset 0x10 onwards. (Will write this later)
        // 0C 	padding 	4 unused bytes

        cursor
            .seek(std::io::SeekFrom::Start(0x10))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 10 	u16 	Creation year
        let year = self.creation_time.year() as u16;
        cursor
            .write_all(&year.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 12 	u8 	Creation month
        let month = self.creation_time.month() as u8;
        cursor
            .write_all(&[month])
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 13 	u8 	Creation day
        let day = self.creation_time.day() as u8;
        cursor
            .write_all(&[day])
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 14 	u8 	Creation hour
        let hour = self.creation_time.hour() as u8;
        cursor
            .write_all(&[hour])
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 15 	u8 	Creation minute
        let minute = self.creation_time.minute() as u8;
        cursor
            .write_all(&[minute])
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 16 	u8 	Unknown
        // 17 	u8 	Unknown
        // 18 	u64 Unknown
        // 20 	u8 	Unknown
        // 21 	padding 	7 unused bytes

        cursor
            .seek(std::io::SeekFrom::Start(0x28))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 28 	u16[0x21] 	UCS-2 course name - 32 characters long plus zero terminator
        let mut name = [0; 0x21];
        ucs2::encode(&self.level_name, &mut name)
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        let mut name_bytes = [0; 0x42];
        for i in 0..0x21 {
            name_bytes[i * 2] = name[i] as u8;
            name_bytes[i * 2 + 1] = 0;
        }
        cursor
            .write_all(&name_bytes)
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 6A 	char[2] 	Game mode ('M1', 'M3', 'MW', 'WU')
        cursor
            .write_all(&self.game_mode.pack()?)
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 6C 	u8 	Unknown

        cursor
            .seek(std::io::SeekFrom::Start(0x6D))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 6D 	u8 	Course theme (0 = overworld, 1 = underground, 2 = castle, 3 = airship, 4 = water, 5 = ghost house)
        cursor
            .write_all(&[self.course_theme as u8])
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 6E 	u8 	Unknown
        // 6F 	u8 	Unknown

        cursor
            .seek(std::io::SeekFrom::Start(0x70))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 70 	u16 	Time limit
        cursor
            .write_all(&self.time_limit.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 72 	u8 	Autoscroll (0 = none, 1 = slow, 2 = medium, 3 = fast)
        cursor
            .write_all(&[self.auto_scroll as u8])
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // 73 	u8 	Flags
        cursor
            .write_all(&[self.flags])
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 74 	u32 	    Width
        cursor
            .write_all(&self.width.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 78 	u8[0x60] 	    Mii data
        cursor
            .write_all(&self.mii_data)
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // D8 	u32 	Unknown
        // DC 	u32 	Unknown
        // E0 	padding 	0xC unused bytes

        cursor
            .seek(std::io::SeekFrom::Start(0xEC))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // EC 	u32 	Object count
        let object_count = self.objects.len() as u32;
        cursor
            .write_all(&object_count.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        // F0 	obj_t[2600] 	Objects (note that the full size is reserved even if the course has less than 2600 objects)
        for object in &self.objects {
            cursor
                .write_all(&object.pack()?)
                .map_err(|_| packed_struct::PackingError::InternalError)?;
        }

        cursor
            .seek(std::io::SeekFrom::Start(0x145F0))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 145F0 	effect_t[300] 	Sound effects
        for effect in &self.sound_effects {
            cursor
                .write_all(&effect.pack()?)
                .map_err(|_| packed_struct::PackingError::InternalError)?;
        }

        // 14F50 	padding 	0xB0 unused bytes

        // Pack the checksum
        cursor
            .seek(std::io::SeekFrom::Start(0x8))
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        // 08 	u32 	Checksum. Standard CRC32 of the entire file from offset 0x10 onwards.


        let checksum = crc32fast::hash(&bytes[0x10..]);

        let mut cursor = Cursor::new(&mut bytes[..]);
        cursor
            .write_all(&checksum.to_be_bytes())
            .map_err(|_| packed_struct::PackingError::InternalError)?;

        Ok(bytes)
    }

    fn unpack(src: &Self::ByteArray) -> packed_struct::PackingResult<Self> {
        // 00 	u64 	Assumed to be a version number. Always 0xB (11) right now
        let version = u64::from_be_bytes(
            src[0..8]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        );

        // 08 	u32 	Checksum. Standard CRC32 of the entire file from offset 0x10 onwards. (Will write this later)
        // 0C 	padding 	4 unused bytes

        // 10 	u16 	Creation year
        let year = u16::from_be_bytes(
            src[0x10..0x12]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        );
        // 12 	u8 	Creation month
        let month = src[0x12];
        // 13 	u8 	Creation day
        let day = src[0x13];
        // 14 	u8 	Creation hour
        let hour = src[0x14];
        // 15 	u8 	Creation minute
        let minute = src[0x15];

        let creation_time = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                .ok_or(packed_struct::PackingError::InvalidValue)?,
            chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, 0)
                .ok_or(packed_struct::PackingError::InvalidValue)?,
        );

        // 16 	u8 	Unknown
        // 17 	u8 	Unknown
        // 18 	u64 Unknown
        // 20 	u8 	Unknown
        // 21 	padding 	7 unused bytes

        // 28 	u16[0x21] 	UCS-2 course name - 32 characters long plus zero terminator
        let mut name_chars = [0u16; 0x21];

        for i in 0..0x21 {
            name_chars[i] = u16::from_be_bytes(
                src[(0x28 + i * 2)..(0x28 + i * 2 + 2)]
                    .try_into()
                    .map_err(|_| packed_struct::PackingError::InvalidValue)?,
            );
        }

        let mut name_bytes = [0u8; 0x21 * 4];
        ucs2::decode(&name_chars, &mut name_bytes)
            .map_err(|_| packed_struct::PackingError::InternalError)?;
        
        let name = String::from_utf8(name_bytes.to_vec()).map_err(|_| packed_struct::PackingError::InternalError)?;

        // 6A 	char[2] 	Game mode ('M1', 'M3', 'MW', 'WU')
        let game_mode = GameMode::unpack(
            &src[0x6A..0x6C]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        )?;

        // 6C 	u8 	Unknown

        // 6D 	u8 	Course theme (0 = overworld, 1 = underground, 2 = castle, 3 = airship, 4 = water, 5 = ghost house)
        let course_theme = CourseTheme::try_from_primitive(src[0x6D]).map_err(|_| packed_struct::PackingError::InvalidValue)?;

        // 6E 	u8 	Unknown
        // 6F 	u8 	Unknown

        // 70 	u16 	Time limit
        let time_limit = u16::from_be_bytes(
            src[0x70..0x72]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        );
        // 72 	u8 	Autoscroll (0 = none, 1 = slow, 2 = medium, 3 = fast)
        let auto_scroll = AutoScroll::try_from_primitive(src[0x72]).map_err(|_| packed_struct::PackingError::InvalidValue)?;

        // 73 	u8 	Flags
        let flags = src[0x73];
        // 74 	u32 	    Width
        let width = u32::from_be_bytes(
            src[0x74..0x78]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        );
        // 78 	u8[0x60] 	    Mii data
        let mut mii_data = [0; 0x60];
        mii_data.copy_from_slice(&src[0x78..0xD8]);

        // D8 	u32 	Unknown
        // DC 	u32 	Unknown
        // E0 	padding 	0xC unused bytes

        // EC 	u32 	Object count
        let object_count = u32::from_be_bytes(
            src[0xEC..0xF0]
                .try_into()
                .map_err(|_| packed_struct::PackingError::InvalidValue)?,
        );

        // F0 	obj_t[2600] 	Objects (note that the full size is reserved even if the course has less than 2600 objects)
        let mut objects = Vec::new();
        for i in 0..object_count {
            let object = Object::unpack(
                &src[(0xF0 + i as usize * 0x20)..(0xF0 + (i + 1) as usize * 0x20)]
                    .try_into()
                    .map_err(|_| packed_struct::PackingError::InvalidValue)?,
            )?;
            objects.push(object);
        }

        // 145F0 	effect_t[300] 	Sound effects
        let mut sound_effects = Vec::new();
        for i in 0..300 {
            let effect = SoundEffect::unpack(
                &src[(0x145F0 + i as usize * 0x8)..(0x145F0 + (i + 1) as usize * 0x8)]
                    .try_into()
                    .map_err(|_| packed_struct::PackingError::InvalidValue)?,
            )?;
            sound_effects.push(effect);
        }
        

        // 14F50 	padding 	0xB0 unused bytes

        Ok(Level {
            version,
            creation_time,
            level_name: name,
            game_mode,
            course_theme,
            time_limit,
            auto_scroll,
            flags,
            width,
            mii_data,
            objects,
            sound_effects,
        })
    }
}

impl Level {
    pub fn new(
        version: u64,
        creation_time: chrono::NaiveDateTime,
        level_name: String,
        game_mode: GameMode,
        course_theme: CourseTheme,
        time_limit: u16,
        auto_scroll: AutoScroll,
        flags: u8,
        width: u32,
        mii_data: [u8; 0x60],
        objects: Vec<Object>,
        sound_effects: Vec<SoundEffect>,
    ) -> Self {
        Level {
            version,
            creation_time,
            level_name,
            game_mode,
            course_theme,
            time_limit,
            auto_scroll,
            flags,
            width,
            mii_data,
            objects,
            sound_effects,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Level, Error> {
        Level::unpack(
            &bytes
                .try_into()
                .map_err(|_| Error::InvalidData)?,
        ).map_err(|_| Error::InvalidData)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let packed = self.pack().map_err(|_| Error::InvalidData)?;
        Ok(packed.to_vec())
    }

    // Width in file / 16, in range of [0, 240]
    pub fn block_width(&self) -> u32 {
        self.width / 16
    }

    // Always 27 in Mario Maker 1
    pub fn block_height(&self) -> u32 {
        27
    }
}