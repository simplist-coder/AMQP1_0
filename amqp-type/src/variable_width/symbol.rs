use crate::constants::constructors::{SYMBOL, SYMBOL_SHORT};
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Symbol(String);

impl Encode for Symbol {
    fn encode(&self) -> Encoded {
        match self.0.len() {
            x if x <= 255 => Encoded::new_variable(SYMBOL_SHORT, self.0.as_bytes().to_vec()),
            _ => Encoded::new_variable(SYMBOL, self.0.as_bytes().to_vec()),
        }
    }
}

impl Decode for Symbol {
    async fn try_decode(
        _constructor: u8,
        _stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}

fn verify_ascii_char_set(string: &String) -> Result<(), AppError> {
    let mut chars = string.chars();
    match chars.all(|c| c.is_ascii()) {
        true => Ok(()),
        false => Err(AppError::IllegalNonASCIICharacterInSymbol),
    }
}

#[allow(dead_code)]
impl Symbol {
    pub fn new(string: String) -> Result<Self, AppError> {
        verify_ascii_char_set(&string)?;
        Ok(Symbol(string))
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
        let val = Symbol("".to_string());
        assert_eq!(val.encode().constructor(), SYMBOL_SHORT);
    }

    #[test]
    fn test_encode_short_symbol_255() {
        let symbol = Symbol::new("a".repeat(255).to_string()).unwrap();
        let encoded = symbol.encode().to_bytes();

        let mut expected = vec![SYMBOL_SHORT];
        let mut bytes = symbol.0.into_bytes();
        expected.append(&mut (bytes.len() as u8).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_large_symbol_256() {
        let large_string = Symbol::new("a".repeat(256)).unwrap();
        let encoded = large_string.encode().to_bytes();

        let mut expected = vec![SYMBOL];
        let mut bytes = large_string.0.into_bytes();
        expected.append(&mut (bytes.len() as u32).to_be_bytes().to_vec());
        expected.append(&mut bytes);

        assert_eq!(encoded, expected);
    }
}
