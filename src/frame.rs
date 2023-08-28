use crate::{error::AppError, performative::Performative};

#[derive(Debug, PartialEq)]
pub struct Frame<'a> {
    header: Header,
    extended_header: ExtendedHeader,
    body: Body<'a>,
}

#[derive(Debug, PartialEq)]
struct Header {
    size: u32,
    doff: u8,
    frame_type: FrameType,
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
}

impl TryFrom<&[u8]> for Frame<'_> {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let header = Header::try_from(&value[0..7])?;
        let extended_header = ExtendedHeader::try_from(&value[8..12])?;
        let body = Body::try_from(&value[13..])?;
        Ok(Frame {
            header,
            extended_header,
            body,
        })
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for ExtendedHeader {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Body<'_> {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for FrameType {
    type Error = AppError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
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
        assert_eq!(header.unwrap().size, 20);
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
        assert_eq!(
            header.err().unwrap(),
            AppError::MalformedFrame("".to_string())
        );
    }

    #[test]
    fn frame_is_malformed_if_size_is_smaller_than_required_frame_header_size() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x06, // size: 8
            0x03, // doff: 3
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];

        let frame = Frame::try_from(data);
        assert!(frame.is_err());
        assert_eq!(
            frame.err().unwrap(),
            AppError::MalformedFrame("".to_string())
        );
    }

    #[test]
    fn frame_is_malformed_if_doff_is_smaller_than_2() {
        let data: &[u8] = &[
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x03, // doff: 1
            0x00, // type: 0
            0x00, // type-specific: 0
            0x00, // type-specific: 0
        ];
        let frame = Frame::try_from(data);
        assert!(frame.is_err());
        assert_eq!(
            frame.err().unwrap(),
            AppError::MalformedFrame("".to_string())
        );
    }
}
