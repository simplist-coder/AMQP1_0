use crate::serde::encode::{Encode, Encoded};
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal, Signed, Zero, num_traits::ToBytes,
};

const EXPONENT_BIAS: i64 = 101;
const EXPONENT_MAX: i64 = 96;
const EXPONENT_MIN: i64 = -95;
const COEFFICIENT_MAX: i64 = 9_999_999; // 7 digits
const DEFAULT_CONSTR: u8 = 0x74;

#[derive(Hash, Eq, PartialEq)]
pub struct Decimal32(BigDecimal);

impl Encode for Decimal32 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(
            DEFAULT_CONSTR,
            encode_to_bytes(&self.0).unwrap(),
        )
    }
}

impl TryFrom<f32> for Decimal32 {
    type Error = Decimal32ConversionError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Ok(Decimal32(BigDecimal::try_from(value)?))
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Decimal32ConversionError {
    #[error("Failed to parse f32 value to Decimal32 value.")]
    ParseDecimal32Error(#[from] bigdecimal::ParseBigDecimalError),
    #[error("Coefficient is too large for Decimal32 representation.")]
    CoefficientTooLarge,
    #[error("Exponent overflowed in Decimal32 representation")]
    ExponentOverflow,
    #[error("Exponent underflowed in Decimal32 representation")]
    ExponentUnderflow,
    #[error("Failed to scale coefficient. Value cannot be fit into 32 bits.")]
    CoefficientScalingFailedError,
    #[error("The base value for setting the sign for converting the Decimal32 into bytes must be zero.")]
    SignSettingValueIsNotZero,
}

type ConversionError = Decimal32ConversionError;

fn encode_to_bytes(value: &BigDecimal) -> Result<Vec<u8>, Decimal32ConversionError> {    
    if value.is_zero() {
        return Ok([0; 4].to_vec());
    }

    // start with empty bit array of 32 bits
    let result: u32 = 0;
    


    let (mut coeff, mut exp) = value.as_bigint_and_exponent();

    Ok(result.to_be_bytes().to_vec())
}

fn set_sign_bit(mut result: u32, sign: Sign) -> Result<u32, ConversionError> {
    if result != 0 {
        return Err(Decimal32ConversionError::SignSettingValueIsNotZero);
    }
    match sign {
        Sign::Minus => {
            result += 1; // set bit as least significant
            result <<= 31; // shift bit to sign bit location
            Ok(result)
        }
        _ => Ok(result)
    }
}

fn set_exponent_bits(mut result: u32, exp: i64)-> Result<u32, ConversionError> {
    match exp {
        _ if exp < EXPONENT_MIN => Err(Decimal32ConversionError::ExponentUnderflow),
        _ if exp > EXPONENT_MAX => Err(Decimal32ConversionError::ExponentOverflow),
        x => {
            Ok(result)
        }
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn construct_decimal_32() {
        let val: Decimal32 = 32.0.try_into().unwrap();
        assert_eq!(val.encode().constructor(), 0x74);
    }

    #[test]
    fn set_sign_bit_works_for_positive_sign() {
        assert_eq!(set_sign_bit(0, Sign::Plus).unwrap().to_be_bytes(), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn set_sign_bit_works_for_negative_sign() {
        assert_eq!(set_sign_bit(0, Sign::Minus).unwrap().to_be_bytes(), [0x80, 0x00, 0x00, 0x00]);
    }
    
    #[test]
    fn set_sign_bit_resturns_error_on_non_zero_base_number() {
        assert!(set_sign_bit(4, Sign::Minus).is_err());
    }

    #[test]
    fn set_exponent_bits_if_exponent_too_large_returns_err() {
        assert_eq!(set_exponent_bits(0x80000000, 100), Err(Decimal32ConversionError::ExponentOverflow));    
        assert_eq!(set_exponent_bits(0x80000000, 97), Err(Decimal32ConversionError::ExponentOverflow));    
    }

    #[test]
    fn set_exponent_bits_if_exponent_too_small_returns_err() {
        assert_eq!(set_exponent_bits(0x80000000, -100), Err(Decimal32ConversionError::ExponentUnderflow));        
        assert_eq!(set_exponent_bits(0x80000000, -96), Err(Decimal32ConversionError::ExponentUnderflow));        
    }
    
    #[test]
    fn set_exponent_bits_works() {
        assert_eq!(set_exponent_bits(0x80000000, 1).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, 2).unwrap(), 0x86700000);
        assert_eq!(set_exponent_bits(0x80000000, 8).unwrap(), 0x86D00000);
        assert_eq!(set_exponent_bits(0x80000000, 16).unwrap(), 0x87500000);
        assert_eq!(set_exponent_bits(0x80000000, 32).unwrap(), 0x88500000);
        assert_eq!(set_exponent_bits(0x80000000, 64).unwrap(), 0x8A500000);
        assert_eq!(set_exponent_bits(0x80000000, 96).unwrap(), 0x8C500000);
        assert_eq!(set_exponent_bits(0x80000000, 0).unwrap(), 0x86500000);
        assert_eq!(set_exponent_bits(0x80000000, -1).unwrap(), 0x86400000);
        // TODO continue here
        assert_eq!(set_exponent_bits(0x80000000, -2).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, -8).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, -16).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, -32).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, -64).unwrap(), 0x86600000);
        assert_eq!(set_exponent_bits(0x80000000, -95).unwrap(), 0x86600000);
    }
    
}
