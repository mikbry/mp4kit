
#[derive(Clone, Debug)]
pub enum Error {
    InvalidData(String),
    EOF(),
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Error::InvalidData(str) => str.clone(),
            Error::EOF() => "End of File".to_owned(),
        }
    }
}
impl Into<String> for Error {
    fn into(self) -> String {
        self.to_string()
    }

}
