
#[derive(Clone, Debug)]
pub enum Error {
    InvalidBoxType(),
    InvalidBox(String),
    InvalidData(String),
    EOF(),
    FileNotFound(),
    BoxNotFound(String),
    InternalError(),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::InvalidBox(str) => str.clone(),
            Error::InvalidData(str) => str.clone(),
            Error::EOF() => "End of File".to_owned(),
            Error::InvalidBoxType() => "Invalid box type".to_owned(),
            Error::FileNotFound() => "File not found".to_owned(),
            Error::BoxNotFound(str) => str.clone(),
            Error::InternalError() => "Internal error".to_owned(),
        }
    }
}
impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }

}
