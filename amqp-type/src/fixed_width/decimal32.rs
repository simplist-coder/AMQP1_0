use crate::serde::encode::{Encode, Encoded};
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal, Signed, Zero,
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

#[derive(thiserror::Error, Debug)]
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
    verify_coefficient_not_too_large(&coeff)?;
    verify_exponent_below_max(&exp)?;
    let sign_bit = match coeff.sign() {
        Sign::Minus => 0x80,
        _ => 0,
    };

    let biased_exp = (exp + EXPONENT_BIAS) as u8;
    let coeff_bytes = coeff.abs().to_bytes_be().1;

    verify_coefficient_scaling(&coeff_bytes)?;

    let mut result: [u8; 4] = [sign_bit | biased_exp, 0, 0, 0];
    let offset = 4 - coeff_bytes.len();
    for (i, &byte) in coeff_bytes.iter().enumerate() {
        result[offset + i] = byte;
    }

    Ok(result.to_vec())
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

fn verify_coefficient_not_too_large(coeff: &BigInt) -> Result<(), ConversionError> {
    if coeff.abs() > COEFFICIENT_MAX.into() {
        return Err(Decimal32ConversionError::CoefficientTooLarge);
    }
    Ok(())
}

fn verify_exponent_below_max(exp: &i64) -> Result<(), Decimal32ConversionError> {
    if exp > &EXPONENT_MAX {
        return Err(Decimal32ConversionError::ExponentOverflow);
    }
    Ok(())
}

fn verify_no_exponent_underflow(exp: &i64) -> Result<(), Decimal32ConversionError> {
    if exp < &EXPONENT_MIN {
        return Err(Decimal32ConversionError::ExponentUnderflow);
    }
    Ok(())
}

fn verify_coefficient_scaling(coeff_bytes: &Vec<u8>) -> Result<(), Decimal32ConversionError> {
    if coeff_bytes.len() > 3 {
        return Err(Decimal32ConversionError::CoefficientScalingFailedError);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use bigdecimal::num_traits::ToBytes;

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
}
