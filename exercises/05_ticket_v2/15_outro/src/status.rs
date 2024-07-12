// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;
    fn try_from(s: String) -> Result<Status, String> {
        if s.to_lowercase() == "todo" {
            return Ok(Status::ToDo)
        }
        if s.to_lowercase() == "inprogress" {
            return Ok(Status::InProgress)
        }
        if s.to_lowercase() == "done" {
            return Ok(Status::Done)
        }
        Err("Err".into())
    }
}

impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(s: &str) -> Result<Status, String> {
        if s.to_lowercase() == "todo" {
            return Ok(Status::ToDo)
        }
        if s.to_lowercase() == "inprogress" {
            return Ok(Status::InProgress)
        }
        if s.to_lowercase() == "done" {
            return Ok(Status::Done)
        }
        Err("Err".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
