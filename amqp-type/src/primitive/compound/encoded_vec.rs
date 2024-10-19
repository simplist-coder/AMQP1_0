use crate::serde::encode::Encoded;

pub struct EncodedVec(Vec<Encoded>);

impl EncodedVec {
    pub fn new(data: Vec<Encoded>) -> Self {
        EncodedVec(data)
    }

    pub(crate) fn serialize_without_constructors(self) -> Vec<u8> {
        let mut res = Vec::new();
        for val in self.0 {
            let mut enc: Vec<u8> = val.serialize_without_constructors();
            res.append(&mut enc);
        }
        res
    }

    fn serialize(self) -> Vec<u8> {
        let mut res = Vec::new();
        for val in self.0 {
            let mut enc: Vec<u8> = val.into_bytes();
            res.append(&mut enc);
        }
        res
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.serialize()
    }
}

impl From<EncodedVec> for Vec<u8> {
    fn from(value: EncodedVec) -> Self {
        value.into_bytes()
    }
}
