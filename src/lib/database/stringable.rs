use hex::encode;

pub trait Stringable {
    fn stringify(&self) -> String;
}

impl Stringable for Vec<u8> {
    fn stringify(&self) -> String {
        encode(self)
    }
}

impl Stringable for uuid::Uuid {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for std::net::IpAddr {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for u64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for u32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for i64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringable for String {
    fn stringify(&self) -> String {
        self.clone()
    }
}

impl Stringable for bool {
    fn stringify(&self) -> String {
        self.to_string()
    }
}
