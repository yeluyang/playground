pub trait Record {
    fn to_bytes(&self) -> Vec<u8>;
}
