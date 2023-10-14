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
    #[error("The base value for setting the exponent was not 0x80000000 or 0x00000000.")]
    IllegalBaseValueForExponentSetting,

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

/// the wikipedia article at https://en.wikipedia.org/wiki/Decimal32_floating-point_format
/// describes decoding a decimal32. in this case we are encoding and thus have to think the other way around
/// if the significant's MSB is 0 then left shift significand by 1 (leading zero becomes implicit)
/// and exponent mus start with bits 00, 01 or 10.
/// if significand's 3 MSB are 100, left shift it by 3 to make the 100 implicit
/// and insert 11 after the sign bit and right shift exponent field by 2 to preserve 
/// the two added bits.
fn set_exponent_bits(mut result: u32, exp: i64)-> Result<u32, ConversionError> {
    if result != 0x8000_0000 && result != 0x0000_0000 {
        return Err(Decimal32ConversionError::IllegalBaseValueForExponentSetting);
    }
    match exp {
        _ if exp < EXPONENT_MIN => Err(Decimal32ConversionError::ExponentUnderflow),
        _ if exp > EXPONENT_MAX => Err(Decimal32ConversionError::ExponentOverflow),
        x => {
            let mut unsigned_exponent: u32 = (exp + EXPONENT_BIAS).try_into().unwrap();
            unsigned_exponent <<= 20;
            result = result | unsigned_exponent;
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
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 96).unwrap()),  format!("{:#b}", 0x8C50_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 64).unwrap()),  format!("{:#b}", 0x8A50_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 32).unwrap()),  format!("{:#b}", 0x8850_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 16).unwrap()),  format!("{:#b}", 0x8750_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 8).unwrap()),   format!("{:#b}", 0x86D0_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 2).unwrap()),   format!("{:#b}", 0x8670_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 1).unwrap()),   format!("{:#b}", 0x8660_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 0).unwrap()),   format!("{:#b}", 0x8650_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -1).unwrap()),  format!("{:#b}", 0x8640_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -2).unwrap()),  format!("{:#b}", 0x8630_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -8).unwrap()),  format!("{:#b}", 0x85C0_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -16).unwrap()), format!("{:#b}", 0x8550_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -32).unwrap()), format!("{:#b}", 0x8450_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -64).unwrap()), format!("{:#b}", 0x8250_0000u32));
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -95).unwrap()), format!("{:#b}", 0x8060_0000u32));
    }
    
}
