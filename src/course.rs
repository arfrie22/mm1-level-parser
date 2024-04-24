use std::io::Read;
use crate::{level::Level, thumbnail::Thumbnail, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseData {
    CourseData,
    CourseDataSub,
    Thumbnail0,
    Thumbnail1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub level: Level,
    pub sub_level: Level,
    pub level_preview: Thumbnail,
    pub level_thumbnail: Thumbnail,
}

impl Course {
    pub fn new(level: Level, sub_level: Level, level_preview: Thumbnail, level_thumbnail: Thumbnail) -> Course {
        Course {
            level,
            sub_level,
            level_preview,
            level_thumbnail,
        }
    }

    pub fn from_bytes(level: &[u8], sub_level: &[u8], level_preview: &[u8], level_thumbnail: &[u8]) -> Result<Course, Error> {
        Ok(Course {
            level: Level::from_bytes(level)?,
            sub_level: Level::from_bytes(sub_level)?,
            level_preview: Thumbnail::from_bytes(level_preview),
            level_thumbnail: Thumbnail::from_bytes(level_thumbnail),
        })
    }

    pub fn from_tar<T: std::io::Read>(archive: &mut tar::Archive<T>) -> Result<Self, Error> {
        let mut level = None;
        let mut sub_level = None;
        let mut level_preview = None;
        let mut level_thumbnail = None;
    
        for entry in archive.entries().map_err(|_| Error::InvalidData)? {
            let mut entry = entry.map_err(|_| Error::InvalidData)?;
            match entry.path().map_err(|_| Error::InvalidData)?.file_name().unwrap().to_str().unwrap() {
                "course_data.cdt" => {
                    let mut buffer = Vec::new();
                    entry.read_to_end(&mut buffer).map_err(|_| Error::InvalidData)?;
                    level = Some(buffer);
                }
                "course_data_sub.cdt" => {
                    let mut buffer = Vec::new();
                    entry.read_to_end(&mut buffer).map_err(|_| Error::InvalidData)?;
                    sub_level = Some(buffer);
                }
                "thumbnail0.tnl" => {
                    let mut buffer = Vec::new();
                    entry.read_to_end(&mut buffer).map_err(|_| Error::InvalidData)?;
                    level_preview = Some(buffer);
                }
                "thumbnail1.tnl" => {
                    let mut buffer = Vec::new();
                    entry.read_to_end(&mut buffer).map_err(|_| Error::InvalidData)?;
                    level_thumbnail = Some(buffer);
                }
                _ => {}
            }
        }
        
        Course::from_bytes(
            &level.ok_or(Error::MissingCourseData(CourseData::CourseData))?,
            &sub_level.ok_or(Error::MissingCourseData(CourseData::CourseDataSub))?,
            &level_preview.ok_or(Error::MissingCourseData(CourseData::Thumbnail0))?,
            &level_thumbnail.ok_or(Error::MissingCourseData(CourseData::Thumbnail1))?,
        )
        
    }
}