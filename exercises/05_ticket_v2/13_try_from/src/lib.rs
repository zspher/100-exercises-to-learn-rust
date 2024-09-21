// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Status, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "done" => Ok(Status::Done),
            "inprogress" => Ok(Status::InProgress),
            &_ => Err("invalid type"),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Status, Self::Error> {
        let value = value.to_lowercase();

        match value.as_str() {
            "todo" => Ok(Status::ToDo),
            "done" => Ok(Status::Done),
            "inprogress" => Ok(Status::InProgress),
            &_ => Err("invalid type"),
        }
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
