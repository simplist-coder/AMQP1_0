use crate::constants::{SMALL_UNSIGNED_LONG, UNSIGNED_LONG, UNSIGNED_LONG_ZERO};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_8;
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

impl Encode for u64 {
    fn encode(self) -> Encoded {
        match self {
            0 => Encoded::new_empty(UNSIGNED_LONG_ZERO),
            x if x > 0 && x <= 255 => {
                Encoded::new_fixed(SMALL_UNSIGNED_LONG, (x as u8).to_be_bytes().to_vec())
            }
            _ => Encoded::new_fixed(UNSIGNED_LONG, self.to_be_bytes().to_vec()),
        }
    }
}

impl Decode for u64 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            UNSIGNED_LONG => Ok(parse_ulong(stream)?),
            SMALL_UNSIGNED_LONG => Ok(parse_small_ulong(stream)?),
            UNSIGNED_LONG_ZERO => Ok(0),
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

fn parse_ulong(iter: &mut IntoIter<u8>) -> Result<u64, AppError> {
    let _abs = Box::pin(vec!["Hello"]);

    let byte_vals = read_bytes_8(iter)?;
    Ok(u64::from_be_bytes(byte_vals))
}

fn parse_small_ulong(iter: &mut IntoIter<u8>) -> Result<u64, AppError> {
    if let Some(val) = iter.next() {
        Ok(u64::from(val))
    } else {
        Err(AmqpError::FrameSizeTooSmall)?
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_ulong() {
        let val: u64 = 500;
        assert_eq!(val.encode().constructor(), 0x80);
    }

    #[test]
    fn test_encode_u64() {
        let test_cases = [
            (0_u64, vec![0x44]),                           // Test with zero
            (1_u64, vec![0x53, 1]),                        // Test with a small positive value
            (255_u64, vec![0x53, 255]), // Test with upper boundary of small ulong
            (256_u64, vec![0x80, 0, 0, 0, 0, 0, 0, 1, 0]), // Test just outside upper boundary
            (
                u64::MAX,
                vec![0x80, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
            ), // Test with the maximum u64 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for u64 value: {input}"
            );
        }
    }

    #[test]
    fn amqp_type_encodes_ulong_smaller_than_256_as_smallulong() {
        let val: u64 = 255;
        assert_eq!(val.encode().constructor(), 0x53);
    }

    #[test]
    fn amqp_type_encodes_ulong_value_0_as_zero_length() {
        let val: u64 = 0;
        assert_eq!(val.encode().constructor(), 0x44);
    }

    #[test]
    fn try_decode_returns_correct_value() {
        let val = vec![0x01, 0x01, 0x11, 0x10, 0x10, 0x00, 0x00, 0x10];
        assert_eq!(
            u64::try_decode(0x80, &mut val.into_iter()).unwrap(),
            72_357_829_700_222_992
        );
    }

    #[test]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(u64::try_decode(0x66, &mut val.into_iter()).is_err());
    }

    #[test]
    fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x00, 0x00, 0x01];
        assert!(u64::try_decode(0x70, &mut val.into_iter()).is_err());
    }

    #[test]
    fn try_decode_can_decode_zero_length_value_zero() {
        let val = vec![];
        assert_eq!(u64::try_decode(0x44, &mut val.into_iter()).unwrap(), 0);
    }

    #[test]
    fn try_decode_can_decode_smallulong_values() {
        let val = vec![0xff];
        assert_eq!(u64::try_decode(0x53, &mut val.into_iter()).unwrap(), 255);
    }

    #[test]
    fn try_decode_returns_error_when_parsing_small_ulong_and_bytes_are_missing() {
        let val = vec![];
        assert!(u64::try_decode(0x53, &mut val.into_iter()).is_err());
    }
}
