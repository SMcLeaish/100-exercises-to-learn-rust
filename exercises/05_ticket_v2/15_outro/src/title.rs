use std::collections::hash_map::Values;

// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.
use thiserror::Error;
#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);
#[derive(Debug, Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    EmptyTitle,
    #[error("The title cannot be longer than 50 bytes")]
    TooLongTitle,
}
fn validate_title(title: &str) -> Result<(), TicketTitleError> {
    if title.is_empty() {
        Err(TicketTitleError::EmptyTitle)
    } else if title.len() > 50 {
        Err(TicketTitleError::TooLongTitle)
    } else {
        Ok(())
    }
}
impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(title: String) -> Result<Self, Self::Error> {
        validate_title(&title)?;
        Ok(Self(title))
    }
}
impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;
    fn try_from(title: &str) -> Result<Self, Self::Error> {
        validate_title(title)?;
        Ok(Self(title.to_string()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
