use crate::constants::UNSIGNED_BYTE;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for u8 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(UNSIGNED_BYTE, self.to_be_bytes().to_vec())
    }
}

impl Decode for u8 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let val = stream.next();
        match (constructor, val) {
            (UNSIGNED_BYTE, Some(x)) => Ok(x),
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ubyte() {
        let val: u8 = 8;
        assert_eq!(val.encode().constructor(), 0x50);
    }

    #[test]
    fn test_encode_u8() {
        let test_cases = [
            (0_u8, vec![0x50, 0]),       // Test with zero
            (1_u8, vec![0x50, 1]),       // Test with a small positive value
            (u8::MAX, vec![0x50, 0xff]), // Test with the maximum u8 value
            (100_u8, vec![0x50, 100]),   // Test with a typical number
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for u8 value: {input}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x10];
        assert_eq!(u8::try_decode(0x50, &mut val.into_iter()).unwrap(), 16);
    }
}
