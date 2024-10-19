use crate::constants::UNSIGNED_SHORT;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use amqp_error::AppError;
use amqp_utils::sync_util::read_bytes_2;
use std::vec::IntoIter;

impl Encode for u16 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(UNSIGNED_SHORT, self.to_be_bytes().to_vec())
    }
}

impl Decode for u16 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            UNSIGNED_SHORT => Ok(parse_u16(stream)?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

fn parse_u16(iter: &mut IntoIter<u8>) -> Result<u16, AppError> {
    let val_bytes = read_bytes_2(iter)?;
    Ok(u16::from_be_bytes(val_bytes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ushort() {
        let val: u16 = 16;
        assert_eq!(val.encode().constructor(), 0x60);
    }

    #[test]
    fn test_encode_u16() {
        let test_cases = [
            (0_u16, vec![0x60, 0, 0]),          // Test with zero
            (1_u16, vec![0x60, 0, 1]),          // Test with a small positive value
            (u16::MAX, vec![0x60, 0xff, 0xff]), // Test with the maximum u16 value
            (256_u16, vec![0x60, 1, 0]),        // Test with a typical number
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for u16 value: {input}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x10];
        assert_eq!(u16::try_decode(0x60, &mut val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(u16::try_decode(0x56, &mut val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x01];
        assert!(u16::try_decode(0x60, &mut val.into_iter()).is_err());
    }
}
