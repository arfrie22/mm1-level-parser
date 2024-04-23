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

#[cfg(test)]
mod tests {
    use crate::course::Course;
    use crate::level::Level;

    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_course() {
        let mut file = File::open("test/course_data.cdt").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let level = Level::from_bytes(&buffer).unwrap();
        println!("{:?}", level);
    }
}