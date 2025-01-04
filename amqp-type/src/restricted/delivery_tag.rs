use crate::error::amqp_error::AmqpError;
use crate::primitive::variable_width::binary::Binary;
use crate::primitive::Primitive;
use crate::error::AppError;

/// # Delivery Tag
///
/// ##### AMQP Specification
/// ```xml
/// <type name="delivery-tag" class="restricted" source="binary"/>
/// ```
/// A delivery-tag may be up to 32 octets of binary data.

#[derive(Debug, Clone, PartialEq)]
pub struct DeliveryTag(Binary);

impl DeliveryTag {
    pub fn new(bytes: Vec<u8>) -> Result<Self, AppError> {
        match bytes.len() {
            0..=32 => Ok(Self(Binary::from(bytes))),
            _ => Err(AmqpError::InvalidField)?,
        }
    }
}

impl TryFrom<Primitive> for DeliveryTag {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match value {
            Primitive::Binary(b) => Ok(DeliveryTag::new(b.into())?),
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

impl TryFrom<Primitive> for Option<DeliveryTag> {
    type Error = AppError;

    fn try_from(value: Primitive) -> Result<Self, Self::Error> {
        match &value {
            Primitive::Null => Ok(None),
            Primitive::Binary(_) => Ok(Some(value.try_into()?)),
            _ => Err(AmqpError::DecodeError)?
        }
    }
}

impl From<DeliveryTag> for Binary {
    fn from(value: DeliveryTag) -> Self {
        value.0
    }
}

impl From<DeliveryTag> for Primitive {
    fn from(value: DeliveryTag) -> Self {
        Primitive::Binary(Binary::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Primitive;

    #[test]
    fn test_delivery_tag() {
        let data = [1].repeat(32);
        for i in 0..=32 {
            DeliveryTag::new(data[..i].to_vec()).unwrap();
        }
    }

    #[test]
    fn test_delivery_tag_error() {
        let data = [1].repeat(33);
        assert!(matches!(
            DeliveryTag::new(data),
            Err(AppError::Amqp(AmqpError::InvalidField))
        ));
    }

    #[test]
    fn test_delivery_tag_into_primitive() {
        let data = [1].repeat(32);
        assert_eq!(
            Primitive::from(DeliveryTag::new(data.clone()).unwrap()),
            Primitive::Binary(Binary::from(data))
        );
    }
}
