use crate::constants::{INTEGER, SMALL_INTEGER};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_4;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for i32 {
    fn encode(self) -> Encoded {
        match self {
            x if (-128..=127).contains(&x) => {
                Encoded::new_fixed(SMALL_INTEGER, (x as i8).to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(INTEGER, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i32 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            INTEGER => Ok(parse_i32(stream)?),
            SMALL_INTEGER => Ok(parse_small_i32(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_i32(iter: &mut IntoIter<u8>) -> Result<i32, AppError> {
    let val_bytes = read_bytes_4(iter)?;
    Ok(i32::from_be_bytes(val_bytes))
}

fn parse_small_i32(iter: &mut IntoIter<u8>) -> Result<i32, AppError> {
    if let Some(val) = iter.next() {
        Ok(i32::from(i8::from_be_bytes([val])))
    } else {
        Err(AmqpError::DecodeError)?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::primitive::Primitive;

    #[test]
    fn test_encode_i32() {
        let test_cases = [
            (127_i32, vec![0x54, 127]),   // Test with upper boundary of small int
            (-128_i32, vec![0x54, 0x80]), // Test with lower boundary of small int
            (128_i32, vec![0x71, 0, 0, 0, 128]), // Test just outside upper boundary
            (-129_i32, vec![0x71, 0xff, 0xff, 0xff, 0x7f]), // Test just outside lower boundary
            (i32::MAX, vec![0x71, 0x7f, 0xff, 0xff, 0xff]), // Test with the maximum i32 value
            (i32::MIN, vec![0x71, 0x80, 0, 0, 0]), // Test with the minimum i32 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for i32 value: {input}"
            );
        }
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x00, 0x00, 0x10];
        assert_eq!(i32::try_decode(0x71, &mut val.into_iter()).unwrap(), 16);
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i32::try_decode(0x56, &mut val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x01];
        assert!(i32::try_decode(0x71, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_smallulong_values() {
        let val = vec![100];
        assert_eq!(
            i32::try_decode(SMALL_INTEGER, &mut val.into_iter()).unwrap(),
            100
        );
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![];
        assert!(i32::try_decode(0x54, &mut val.into_iter()).is_err());
    }

    #[test]
    fn construct_int() {
        let val = 500;
        assert_eq!(val.encode().constructor(), 0x71);
    }

    #[test]
    fn amqp_encodes_ints_between_neg_128_and_127_as_smallint() {
        let lower = -128;
        let higher = 127;
        assert_eq!(lower.encode().constructor(), 0x54);
        assert_eq!(higher.encode().constructor(), 0x54);
    }

    #[test]
    fn test_encode_decode_negative_small_int() {
        let original = Primitive::Int(-100);
        let encoded = original.encode().into_bytes();
        let stream = &mut encoded.into_iter();
        stream.next();
        let decoded = i32::try_decode(SMALL_INTEGER, stream).unwrap();
        assert_eq!(decoded, -100);
    }

    #[test]
    fn test_encode_decode_negative_int() {
        let original = Primitive::Int(-1000);
        let encoded = original.encode().into_bytes();
        let stream = &mut encoded.into_iter();
        stream.next();
        let decoded = i32::try_decode(INTEGER, stream).unwrap();
        assert_eq!(decoded, -1000);
    }
}
