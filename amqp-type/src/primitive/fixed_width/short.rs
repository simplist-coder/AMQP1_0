use crate::constants::SHORT;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_2;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for i16 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(SHORT, self.to_be_bytes().to_vec())
    }
}

impl Decode for i16 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            SHORT => Ok(parse_i16(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_i16(iter: &mut IntoIter<u8>) -> Result<i16, AppError> {
    let val_bytes = read_bytes_2(iter)?;
    Ok(i16::from_be_bytes(val_bytes))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ushort() {
        let val: i16 = 16;
        assert_eq!(val.encode().constructor(), 0x61);
    }

    #[test]
    fn test_encode_i16() {
        let test_cases = [
            (0_i16, vec![0x61, 0, 0]),          // Test with zero
            (1_i16, vec![0x61, 0, 1]),          // Test with a positive value
            (-1_i16, vec![0x61, 0xff, 0xff]),   // Test with a negative value
            (i16::MAX, vec![0x61, 0x7f, 0xff]), // Test with the maximum i16 value
            (i16::MIN, vec![0x61, 0x80, 0x00]), // Test with the minimum i16 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for i16 value: {input}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x10];
        assert_eq!(i16::try_decode(0x61, &mut val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i16::try_decode(0x56, &mut val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x01];
        assert!(i16::try_decode(0x61, &mut val.into_iter()).is_err());
    }
}
