use course::CourseData;

pub mod level;
pub mod objects;
pub mod sound_effects;
pub mod thumbnail;
pub mod course;

#[derive(Debug)]
pub enum Error {
    FileTooLarge,
    InvalidData,
    MissingCourseData(CourseData),
}
