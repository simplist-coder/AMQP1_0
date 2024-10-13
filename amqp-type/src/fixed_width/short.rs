use crate::common::read_bytes_2;
use crate::constants::constructors::SHORT;
use crate::error::AppError;
use crate::serde::decode::Decode;
use crate::serde::encode::{Encode, Encoded};
use std::pin::Pin;
use tokio_stream::Stream;

impl Encode for i16 {
    fn encode(&self) -> Encoded {
        Encoded::new_fixed(SHORT, self.to_be_bytes().to_vec())
    }
}

impl Decode for i16 {
    async fn try_decode(
        constructor: u8,
        stream: &mut Pin<Box<impl Stream<Item = u8>>>,
    ) -> Result<Self, crate::error::AppError>
    where
        Self: Sized,
    {
        match constructor {
            SHORT => Ok(parse_i16(stream).await?),
            c => Err(AppError::DeserializationIllegalConstructorError(c)),
        }
    }
}

async fn parse_i16(iter: &mut Pin<Box<impl Stream<Item = u8>>>) -> Result<i16, AppError> {
    let val_bytes = read_bytes_2(iter).await?;
    Ok(i16::from_be_bytes(val_bytes))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::tests::ByteVecExt;

    #[test]
    fn construct_ushort() {
        let val: i16 = 16;
        assert_eq!(val.encode().constructor(), 0x61);
    }

    #[test]
    fn test_encode_i16() {
        let test_cases = [
            (0_i16, vec![0x61, 0, 0]),          // Test with zero
            (1_i16, vec![0x61, 0, 1]),          // Test with a positive value
            (-1_i16, vec![0x61, 0xff, 0xff]),   // Test with a negative value
            (i16::MAX, vec![0x61, 0x7f, 0xff]), // Test with the maximum i16 value
            (i16::MIN, vec![0x61, 0x80, 0x00]), // Test with the minimum i16 value
        ];

        for (input, expected) in test_cases {
            let encoded = input.encode();
            assert_eq!(
                encoded.to_bytes(),
                expected,
                "Failed encoding for i16 value: {}",
                input
            );
        }
    }

    #[tokio::test]
    async fn try_decode_returns_correct_value() {
        let val = vec![0x00, 0x10];
        assert_eq!(
            i16::try_decode(0x61, &mut val.into_pinned_stream())
                .await
                .unwrap(),
            16
        )
    }

    #[tokio::test]
    async fn decode_returns_error_when_value_bytes_are_invalid() {
        let val = vec![0x44];
        assert!(i16::try_decode(0x56, &mut val.into_pinned_stream())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn decode_returns_error_when_bytes_are_missing() {
        let val = vec![0x01];
        assert!(i16::try_decode(0x61, &mut val.into_pinned_stream())
            .await
            .is_err());
    }
}
