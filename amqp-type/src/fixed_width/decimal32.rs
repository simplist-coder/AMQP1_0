use crate::serde::encode::{Encode, Encoded};
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal, Signed, Zero, num_traits::ToBytes,
};

const EXPONENT_BIAS: i64 = 127;
const EXPONENT_MAX: i64 = 127;
const EXPONENT_MIN: i64 = 1 - EXPONENT_MAX;
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

impl TryFrom<BigDecimal> for Decimal32 {
    type Error = ConversionError;

    fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
        todo!("implement conversion with error handling to only allow valid values according to IEEE 754")
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
    // start with empty bit array of 32 bits
    let mut result: u32 = 0;

    let (mut coeff, mut exp) = value.as_bigint_and_exponent();

    result = set_sign_bit(result, coeff.sign())?;
    result = set_exponent_bits(result, exp)?;
    result = set_significand_bits(result, coeff)?;

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

fn set_significand_bits(mut result: u32, significand: BigInt) -> Result<u32, ConversionError> {


    Ok(result)
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
        assert_eq!(set_exponent_bits(0x80000000, 128), Err(Decimal32ConversionError::ExponentOverflow));    
        assert_eq!(set_exponent_bits(0x80000000, 139), Err(Decimal32ConversionError::ExponentOverflow));    
    }

    #[test]
    fn set_exponent_bits_if_exponent_too_small_returns_err() {
        assert_eq!(set_exponent_bits(0x80000000, -127), Err(Decimal32ConversionError::ExponentUnderflow));        
        assert_eq!(set_exponent_bits(0x80000000, -300), Err(Decimal32ConversionError::ExponentUnderflow));        
    }
    
    #[test]
    fn set_exponent_bits_works() {
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, 127).unwrap()),  format!("{:#b}", 0x8C50_0000u32));
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
        assert_eq!(format!("{:#b}", set_exponent_bits(0x8000_0000, -126).unwrap()), format!("{:#b}", 0x8060_0000u32));
    }
    
}
