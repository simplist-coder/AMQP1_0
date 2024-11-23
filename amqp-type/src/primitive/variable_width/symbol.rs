use crate::constants::{SYMBOL, SYMBOL_SHORT};
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use crate::error::AppError;
use crate::utils::sync_util::{read_bytes, read_bytes_4};
use std::vec::IntoIter;
use crate::error::amqp_error::AmqpError;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Symbol(String);

impl Symbol {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Encode for Symbol {
    fn encode(self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(SYMBOL_SHORT, self.0.as_bytes().to_vec()),
            _ => Encoded::new_variable(SYMBOL, self.0.as_bytes().to_vec()),
        }
    }
}

impl Decode for Symbol {
    fn try_decode(constructor: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        match constructor {
            SYMBOL_SHORT => Ok(parse_short_symbol(stream)?),
            SYMBOL => Ok(parse_symbol(stream)?),
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

fn parse_short_symbol(stream: &mut IntoIter<u8>) -> Result<Symbol, AppError> {
    match stream.next() {
        None => Err(AmqpError::DecodeError)?,
        Some(size) => Ok(Symbol::new(String::from_utf8(read_bytes(
            stream,
            size as usize,
        )?)?)?),
    }
}

fn parse_symbol(stream: &mut IntoIter<u8>) -> Result<Symbol, AppError> {
    let size = u32::from_be_bytes(read_bytes_4(stream)?);
    Symbol::new(String::from_utf8(read_bytes(stream, size as usize)?)?)
}

fn verify_ascii_char_set(string: &str) -> Result<(), AppError> {
    let mut chars = string.chars();
    match chars.all(|c| c.is_ascii()) {
        true => Ok(()),
        false => Err(AmqpError::InvalidField)?,
    }
}

impl Symbol {
    pub fn new(string: String) -> Result<Self, AppError> {
        verify_ascii_char_set(&string)?;
        Ok(Symbol(string))
    }

    pub fn with_ascii(string: &str) -> Self {
        verify_ascii_char_set(&string).expect("String contains non ASCII characters");
        Symbol(string.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Symbol {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Symbol::new(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_symbol() {
        let val = Symbol(String::new());
        assert_eq!(val.encode().constructor(), SYMBOL_SHORT);
    }

    #[test]
    fn test_encode_short_symbol_255() {
        let symbol = Symbol::new("a".repeat(255).to_string()).unwrap();
        let encoded = symbol.clone().encode().into_bytes();

        let mut expected = vec![SYMBOL_SHORT];
        let mut bytes = symbol.0.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_large_symbol_256() {
        let large_string = Symbol::new("a".repeat(256)).unwrap();
        let encoded = large_string.clone().encode().into_bytes();

        let mut expected = vec![SYMBOL];
        let mut bytes = large_string.0.into_bytes();
        expected.append(&mut (bytes.len() as u32).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_decode_small_string() {
        let data = vec![5, b'H', b'e', b'l', b'l', b'o'];
        let result = Symbol::try_decode(SYMBOL_SHORT, &mut data.into_iter()).unwrap();
        assert_eq!(result.0, "Hello".to_string());
    }

    #[test]
    fn test_decode_large_string() {
        let size_bytes = 11u32.to_be_bytes();
        let mut data = vec![size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]];
        data.extend_from_slice(b"Hello World");
        let result = Symbol::try_decode(SYMBOL, &mut data.into_iter()).unwrap();
        assert_eq!(result.0, "Hello World".to_string());
    }

    #[test]
    fn test_illegal_constructor() {
        let data = vec![5, b'E', b'r', b'r', b'o', b'r'];
        let result = Symbol::try_decode(0xFF, &mut data.into_iter());
        assert!(matches!(
            result,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn test_iterator_empty_or_too_short() {
        let data = vec![];
        let result = Symbol::try_decode(SYMBOL, &mut data.into_iter());
        assert!(matches!(
            result,
            Err(AppError::Amqp(AmqpError::DecodeError))
        ));
    }

    #[test]
    fn test_ascii_compliance() {
        let data = vec![2, 0xC3, 0xA9]; // 'é' in UTF-8
        let result = Symbol::try_decode(SYMBOL_SHORT, &mut data.into_iter());
        assert!(matches!(
            result,
            Err(AppError::Amqp(AmqpError::InvalidField))
        ));
    }
}
