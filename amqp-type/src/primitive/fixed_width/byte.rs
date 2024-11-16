use crate::constants::BYTE;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for i8 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(BYTE, self.to_be_bytes().to_vec())
    }
}

impl Decode for i8 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            BYTE => Ok(parse_i8(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_i8(iter: &mut IntoIter<u8>) -> Result<i8, AppError> {
    if let Some(val) = iter.next() {
        Ok(i8::from_be_bytes([val]))
    } else {
        Err(AmqpError::FrameSizeTooSmall)?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::constants::BYTE;

    #[test]
    fn construct_byte() {
        let val: i8 = 8;
        assert_eq!(val.encode().constructor(), 0x51);
    }

    #[test]
    fn test_encode_i8() {
        let test_cases = [
            (0_i8, vec![BYTE, 0]),       // Test with zero
            (1_i8, vec![BYTE, 1]),       // Test with a positive value
            (-1_i8, vec![BYTE, 0xff]),   // Test with a negative value
            (i8::MAX, vec![BYTE, 0x7f]), // Test with the maximum i8 value
            (i8::MIN, vec![BYTE, 0x80]), // Test with the minimum i8 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for i8 value: {input}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x10];
        assert_eq!(i8::try_decode(0x51, &mut val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i8::try_decode(0x66, &mut val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![];
        assert!(i8::try_decode(0x51, &mut val.into_iter()).is_err());
    }
}
