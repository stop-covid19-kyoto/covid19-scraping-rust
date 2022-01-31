#[derive(Debug)]
pub struct IncorrectFormatError {}

impl fmt::Display for IncorrectFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect format")
    }
}

impl error::Error for IncorrectFormatError {
    fn description(&self) -> &str {
        "Incorrect format"
    }
}
