use hex::encode;

pub trait Stringable {
    fn stringify(&self) -> Result<String, StringifyError>;
}

impl Stringable for Vec<u8> {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(encode(self))
    }
}

impl Stringable for uuid::Uuid {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for std::net::IpAddr {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for u64 {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for u32 {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for i64 {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for i32 {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for &str {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for str {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

impl Stringable for String {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.clone())
    }
}

impl Stringable for bool {
    fn stringify(&self) -> Result<String, StringifyError> {
        Ok(self.to_string())
    }
}

#[derive(Debug)]
pub enum StringifyError {
    Message(&'static str),
}

impl std::fmt::Display for StringifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(error_msg) => write!(f, "{:?}", error_msg),
        }
    }
}

impl std::error::Error for StringifyError {}

impl From<&'static str> for StringifyError {
    fn from(e: &'static str) -> Self {
        Self::Message(e)
    }
}
