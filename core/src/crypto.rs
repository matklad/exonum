#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

pub fn hash(_buff: &[u8]) -> Hash {
    unimplemented!()
}
