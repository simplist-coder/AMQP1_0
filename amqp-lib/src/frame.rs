use crate::{error::AppError, performative::Performative};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, PartialEq)]
pub enum Frame<'a> {
    Empty(Header),
    Content(Content<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Content<'a> {
    header: Header,
    extended_header: Option<ExtendedHeader>,
    body: Option<Body<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    size: u32,
    doff: u8,
    frame_type: FrameType,
    extended_header: Option<ExtendedHeader>,
}

#[derive(Debug, PartialEq)]
struct ExtendedHeader {}

#[derive(Debug, PartialEq)]
struct Body<'a> {
    performative: Performative,
    payload: &'a [u8],
}

#[derive(Debug, PartialEq)]
enum FrameType {
    Amqp,
    Sasl,
}

impl TryFrom<&[u8]> for Frame<'_> {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let header = Header::try_from(&value[0..7])?;
        let body_start: usize = (4 * header.doff).into();
        let extended_header = ExtendedHeader::try_from(&value[8..body_start - 1])?;
        let body = Body::try_from(&value[body_start..])?;
        Ok(Frame::Content(Content {
            header,
            extended_header: Some(extended_header),
            body: Some(body),
        }))
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut c = Cursor::new(value);
        let size: u32 = c.read_u32::<BigEndian>()?;
        let doff: u8 = c.read_u8()?;
        let _frame_type = FrameType::try_from(c.read_u8()?);
        match (size, doff) {
            (size, doff) if size >= 8 && doff >= 2 => Ok(Header {
                size,
                doff,
                frame_type: FrameType::Amqp,
                extended_header: None,
            }),
            (size, _) if size < 8 => Err(AppError::MalformedFrame(
                "Size is smaller than minimum header size of 8.",
            )),
            (_, doff) if doff < 2 => Err(AppError::MalformedFrame(
                "Doff is smaller than minimum doff of 2.",
            )),
            (_, _) => Err(AppError::MalformedFrame(
                "Size and Doff of header are invalid.",
            )),
        }
    }
}

impl TryFrom<&[u8]> for ExtendedHeader {
    type Error = AppError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Body<'_> {
    type Error = AppError;

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<u8> for FrameType {
    type Error = AppError;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match code {
            0x00 => Ok(FrameType::Amqp),
            0x01 => Ok(FrameType::Sasl),
            _ => Err(AppError::MalformedFrame("Invalid Frame Type.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::AppError, frame::FrameType};

    use super::*;

    #[test]
    fn header_should_deserialize_size() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];
        let header = Header::try_from(data);
        assert!(header.is_ok());
        assert_eq!(header.unwrap().size, 8);
    }

    #[test]
    fn header_should_deserialize_doff() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];
        let header = Header::try_from(data);
        assert!(header.is_ok());
        assert_eq!(header.unwrap().doff, 3);
    }

    #[test]
    fn header_should_deserialize_frame_type() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];
        let header = Header::try_from(data);
        assert!(header.is_ok());
        assert_eq!(header.unwrap().frame_type, FrameType::Amqp);
    }

    #[test]
    fn header_deserialization_should_fail_on_malformed_header() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x06, // size: 6
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];

        let header = Header::try_from(data);
        assert!(header.is_err());
    }

    #[test]
    fn frame_is_malformed_if_size_is_smaller_than_required_frame_header_size() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x06, // size: 6
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];

        let header = Header::try_from(data);
        assert!(header.is_err());
    }

    #[test]
    fn frame_is_malformed_if_doff_is_smaller_than_2() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x01, // doff: 1
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];
        let header = Header::try_from(data);
        assert!(header.is_err());
    }
}
