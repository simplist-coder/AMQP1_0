use crate::constants::{LONG, SMALL_LONG};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_8;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for i64 {
    fn encode(self) -> Encoded {
        match self {
            x if (-128..=127).contains(&x) => {
                Encoded::new_fixed(SMALL_LONG, (x as i8).to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(LONG, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for i64 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            LONG => Ok(parse_i64(stream)?),
            SMALL_LONG => Ok(parse_small_i64(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_i64(iter: &mut IntoIter<u8>) -> Result<i64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(i64::from_be_bytes(byte_vals))
}

fn parse_small_i64(iter: &mut IntoIter<u8>) -> Result<i64, AppError> {
    if let Some(val) = iter.next() {
        Ok(i64::from(i8::from_be_bytes([val])))
    } else {
        Err(AmqpError::DecodeError)?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_long() {
        let val: i64 = 500;
        assert_eq!(val.encode().constructor(), 0x81);
    }

    #[test]
    fn test_encode_i64() {
        let test_cases = [
            (127_i64, vec![0x55, 127]),   // Test with upper boundary of small long
            (-128_i64, vec![0x55, 0x80]), // Test with lower boundary of small long
            (128_i64, vec![0x81, 0, 0, 0, 0, 0, 0, 0, 128]), // Test just outside upper boundary
            (
                -129_i64,
                vec![0x81, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
            ), // Test just outside lower boundary
            (
                i64::MAX,
                vec![0x81, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            ), // Test with the maximum i64 value
            (i64::MIN, vec![0x81, 0x80, 0, 0, 0, 0, 0, 0, 0]), // Test with the minimum i64 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for i64 value: {input}"
            );
        }
    }

    #[test]
    fn amqp_encodes_longs_between_neg_128_and_127_as_smalllong() {
        let lower: i64 = -128;
        let higher: i64 = 127;
        assert_eq!(lower.encode().constructor(), 0x55);
        assert_eq!(higher.encode().constructor(), 0x55);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10];
        assert_eq!(
            i64::try_decode(0x81, &mut val.into_iter()).unwrap(),
            1_048_592
        );
    }

    #[test]
    fn try_decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i64::try_decode(0x66, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x00, 0x00, 0x01];
        assert!(i64::try_decode(0x81, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_smalli64_values() {
        let positive = vec![100];
        let negative = (-100i8).to_be_bytes().to_vec();
        assert_eq!(
            i64::try_decode(SMALL_LONG, &mut positive.into_iter()).unwrap(),
            100
        );
        assert_eq!(
            i64::try_decode(SMALL_LONG, &mut negative.into_iter()).unwrap(),
            -100
        );
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_i64_and_bytes_are_missing() {
        let val = vec![];
        assert!(i64::try_decode(0x55, &mut val.into_iter()).is_err());
    }
}
