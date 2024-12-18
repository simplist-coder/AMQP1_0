use crate::constants::DECIMAL_32;
use crate::error::amqp_error::AmqpError;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::utils::sync_util::read_bytes_4;
use std::hash::{Hash, Hasher};
use std::vec::IntoIter;

#[derive(Debug, Clone, Copy)]
pub struct Decimal32(f32);

impl Encode for Decimal32 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(DECIMAL_32, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Decimal32 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            DECIMAL_32 => Ok(parse_decimal32(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_decimal32(iter: &mut IntoIter<u8>) -> Result<Decimal32, AppError> {
    let byte_vals = read_bytes_4(iter)?;
    Ok(Decimal32(f32::from_be_bytes(byte_vals)))
}

impl Hash for Decimal32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialEq for Decimal32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for Decimal32 {}

impl From<f32> for Decimal32 {
    fn from(value: f32) -> Self {
        Decimal32(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_32() {
        let val: Decimal32 = 32f32.into();
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn test_encode_decimal32() {
        let test_cases = [
            (Decimal32(0.0), vec![DECIMAL_32, 0, 0, 0, 0]), // Test with zero
            (Decimal32(1.0), vec![DECIMAL_32, 63, 128, 0, 0]), // Test with a positive value
            (Decimal32(-1.0), vec![DECIMAL_32, 191, 128, 0, 0]), // Test with a negative value
            (Decimal32(f32::INFINITY), vec![DECIMAL_32, 127, 128, 0, 0]), // Test with positive infinity
            (
                Decimal32(f32::NEG_INFINITY),
                vec![DECIMAL_32, 255, 128, 0, 0],
            ), // Test with negative infinity
            (Decimal32(f32::NAN), vec![DECIMAL_32, 127, 192, 0, 0]),      // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for Decimal32 value: {input:?}"
            );
        }
    }

    #[test]
    fn test_successful_deserialization() {
        let value = 1.2345f32;
        let data = value.to_be_bytes().to_vec();

        match Decimal32::try_decode(DECIMAL_32, &mut data.into_iter()) {
            Ok(decimal) => assert_eq!(value, decimal.0),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF;
        let bytes = vec![ /* other bytes */];

        assert!(matches!(
            Decimal32::try_decode(illegal_constructor, &mut bytes.into_iter()),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector

        assert!(matches!(
            Decimal32::try_decode(DECIMAL_32, &mut bytes.into_iter()),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }
}
