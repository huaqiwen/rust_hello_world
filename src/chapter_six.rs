
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    pub fn get_str(&self) -> String {
        return match self {
            IpAddr::V4(a, b, c, d) => { format!("{}.{}.{}.{}", a, b, c, d) }
            IpAddr::V6(s) => { String::from(s) }
        }
    }
}
