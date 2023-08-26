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

    use super::Header;

    fn create_header_bytes(size: u32, doff: u8, frame_type: u8) -> Vec<u8> {
        let mut res = vec![];
        res
    }

    #[test]
    fn header_should_deserialize_size() {
        let data = create_header_bytes(20, 0, 0);
        let header = Header::try_from(data.as_slice());
        assert!(header.is_ok());
        assert_eq!(header.unwrap().size, 20);
    }

    #[test]
    fn header_should_deserialize_doff() {
        let data = create_header_bytes(20, 5, 0);
        let header = Header::try_from(data.as_slice());
        assert!(header.is_ok());
        assert_eq!(header.unwrap().doff, 5);
    }

    #[test]
    fn header_should_deserialize_frame_type() {
        let data = create_header_bytes(20, 5, 0);
        let header = Header::try_from(data.as_slice());
        assert!(header.is_ok());
        assert_eq!(header.unwrap().frame_type, FrameType::Amqp);
    }

    #[test]
    fn header_deserialization_should_fail_on_malformed_header() {}
}
