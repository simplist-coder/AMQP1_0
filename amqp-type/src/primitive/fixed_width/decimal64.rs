use crate::constants::DECIMAL_64;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::read_bytes_8;
use std::hash::{Hash, Hasher};
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

#[derive(Debug, Copy, Clone)]
pub struct Decimal64(f64);

impl Encode for Decimal64 {
    fn encode(self) -> Encoded {
        Encoded::new_fixed(DECIMAL_64, self.0.to_be_bytes().to_vec())
    }
}

impl Decode for Decimal64 {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            DECIMAL_64 => Ok(parse_decimal64(stream)?),
            _ => Err(AmqpError::DecodeError)?,
        }
    }
}

fn parse_decimal64(iter: &mut IntoIter<u8>) -> Result<Decimal64, AppError> {
    let byte_vals = read_bytes_8(iter)?;
    Ok(Decimal64(f64::from_be_bytes(byte_vals)))
}

impl From<f64> for Decimal64 {
    fn from(value: f64) -> Self {
        Decimal64(value)
    }
}

impl PartialEq for Decimal64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits().eq(&other.0.to_bits())
    }
}

impl Eq for Decimal64 {}

impl Hash for Decimal64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_decimal_64() {
        let val: Decimal64 = 64.0.into();
        assert_eq!(val.encode().constructor(), 0x84);
    }

    #[test]
    fn test_encode_decimal64() {
        let test_cases = [
            (Decimal64(0.0), [0x84, 0, 0, 0, 0, 0, 0, 0, 0]), // Test with zero
            (Decimal64(1.0), [0x84, 63, 240, 0, 0, 0, 0, 0, 0]), // Test with a positive value
            (Decimal64(-1.0), [0x84, 191, 240, 0, 0, 0, 0, 0, 0]), // Test with a negative value
            (Decimal64(f64::INFINITY), [0x84, 127, 240, 0, 0, 0, 0, 0, 0]), // Test with positive infinity
            (
                Decimal64(f64::NEG_INFINITY),
                [0x84, 255, 240, 0, 0, 0, 0, 0, 0],
            ), // Test with negative infinity
            (Decimal64(f64::NAN), [0x84, 127, 248, 0, 0, 0, 0, 0, 0]),      // Test with NaN
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.into_bytes(),
                expected,
                "Failed encoding for Decimal64 value: {input:?}"
            );
        }
    }

    #[test]
    fn test_successful_deserialization() {
        let value = 1.2345f64;
        let mut data = vec![];
        data.append(&mut value.to_be_bytes().to_vec()); // Put an f64 into the buffer

        match Decimal64::try_decode(DECIMAL_64, &mut data.into_iter()) {
            Ok(decimal) => assert_eq!(value, decimal.0),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    #[test]
    fn test_illegal_constructor_deserialization() {
        let illegal_constructor = 0xFF; // Assuming this is not Decimal64
        let bytes = vec![ /* other bytes */];

        assert!(matches!(
            Decimal64::try_decode(illegal_constructor, &mut bytes.into_iter()),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ))
    }

    #[test]
    fn test_empty_iterator_deserialization() {
        let bytes = vec![]; // Empty vector

        assert!(matches!(
            Decimal64::try_decode(DECIMAL_64, &mut bytes.into_iter()),
            Err(AppError::Amqp(AmqpError::DecodeError))
        ))
    }
}
