use crate::serde::encode::Encoded;

pub struct EncodedVec(Vec<Encoded>);

impl EncodedVec {
    pub fn new(data: Vec<Encoded>) -> Self {
        EncodedVec(data)
    }
}

impl From<EncodedVec> for Vec<u8> {
    fn from(value: EncodedVec) -> Self {
        let mut res = Vec::new();
        for val in value.0 {
            let mut enc: Vec<u8> = val.into();
            res.append(&mut enc);
        }
        res
    }
}