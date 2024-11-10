// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.
use thiserror::Error;
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
#[derive(Debug, Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    invalid_status: String,
}
fn parse_status(status: &str) -> Result<Status, ParseStatusError> {
    match status.to_lowercase().as_str() {
        "todo" => Ok(Status::ToDo),
        "inprogress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        _ => Err(ParseStatusError {
            invalid_status: status.to_string(),
        }),
    }
}
impl TryFrom<String> for Status {
    type Error = ParseStatusError;
    fn try_from(status: String) -> Result<Self, Self::Error> {
        parse_status(&status)
    }
}
impl TryFrom<&str> for Status {
    type Error = ParseStatusError;
    fn try_from(status: &str) -> Result<Self, Self::Error> {
        parse_status(status)
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
