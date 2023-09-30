use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;

#[cfg(not(feature = "zero-length-bools"))]
impl Encode for bool {
    fn encode(&self) -> Encoded {
        match self {
            true => Encoded::new_fixed(0x56, vec![0x01]),
            false => Encoded::new_fixed(0x56, vec![0x00]),
        }
    }
}


#[cfg(feature = "zero-length-bools")]
impl Encode for bool {
    
    fn encode(&self) -> Encoded {
        match self {
            true => 0x41.into(),
            false => 0x42.into(),
        }
    }
}



#[cfg(not(feature = "zero-length-bools"))]
impl Decode for bool {

    fn can_decode(data: impl Iterator<Item = u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(0x56) => true,
            _ => false
        }
    }

    fn try_decode(mut iter: impl Iterator<Item = u8>) -> Result<Self, AppError> where Self: Sized {
        let con = iter.next();
        let val = iter.next();
        match (con, val) {
            (Some(c), Some(v)) if c == 0x56 && v == 0x00 => Ok(false),
            (Some(c), Some(v)) if c == 0x56 && v == 0x01 => Ok(true),
            (Some(c), _) => Err(AppError::DeserializationError("bool".to_string(),format!("bool cannot be constructed from value {:#04x}", c))),
            (Some(c), None) => Err(AppError::DeserializationError("bool".to_string(), "Iterator was empty".to_string())),
            (None, _) => Err(AppError::DeserializationError("bool".to_string(), "Iterator was empty".to_string())),
        }

    }
}



#[cfg(feature = "zero-length-bools")]
impl Decode for bool {

    fn can_decode(data: Iterator<Item = u8>) -> bool {
        let mut iter = data.into_iter().peekable();
        match iter.peek() {
            Some(0x41) => true,
            Some(0x42) => true,
            _ => false
        }
    }

    fn try_decode(data: Iterator<Item = u8>) -> Result<Self, AppError> where Self: Sized {                
        if let Some(val) = iter.next() {
            return match val {
                0x41 => Ok(true),
                0x42 => Ok(false),
                _ => Err(AppError::DeserializationError("bool".to_string(), format!("bool cannot be constructed from value {:#04x}", val)))
            }
        }
        Err(AppError::DeserializationError("bool".to_string(), "Iterator was empty".to_string()))        
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn construct_bool() {
        assert_eq!(true.encode().constructor(), 0x56);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn bool_gets_encoded_correctly() {
        assert_eq!(true.encode().to_bytes(), vec![0x56, 0x01]);
        assert_eq!(false.encode().to_bytes(), vec![0x56, 0x00]);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn can_decode_returns_true_if_constructor_is_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::can_decode(val_true.into_iter()), true);
        assert_eq!(bool::can_decode(val_false.into_iter()), true);
    }

    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn can_decode_returns_false_if_constructor_invalid() {
        let val_true = vec![0x88, 0x01];
        let val_false = vec![0x97, 0x00];
        assert_eq!(bool::can_decode(val_true.into_iter()), false);
        assert_eq!(bool::can_decode(val_false.into_iter()), false);
    }

    
    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn decode_returns_error_when_value_bytes_are_invalid() {
        let val_true = vec![0x56, 0x34];
        let val_false = vec![0x56, 0x44];
        assert!(bool::try_decode(val_true.into_iter()).is_err());
        assert!(bool::try_decode(val_false.into_iter()).is_err());
    }


    #[test]
    #[cfg(not(feature = "zero-length-bools"))]
    fn try_decode_returns_correct_value_if_bytes_are_valid() {
        let val_true = vec![0x56, 0x01];
        let val_false = vec![0x56, 0x00];
        assert_eq!(bool::try_decode(val_true.into_iter()).unwrap(), true);
        assert_eq!(bool::try_decode(val_false.into_iter()).unwrap(), false);
    }
    

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_false_as_zero_length() {
        assert_eq!(false.encode().constructor(), 0x42);
    }

    #[test]
    #[cfg(feature = "zero-length-bools")]
    fn amqp_type_constructs_bool_true_as_zero_length() {
        assert_eq!(true.encode().constructor(), 0x41)
    }
}
