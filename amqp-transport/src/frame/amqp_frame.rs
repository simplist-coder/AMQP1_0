use crate::constants::AMQP_FRAME;
use crate::frame::performative::Performative;
use amqp_error::AppError;
use amqp_utils::sync_util::read_bytes_2;
use amqp_utils::vec::VecExt;
use std::vec::IntoIter;

#[derive(Debug, Copy, Clone)]
pub struct AmqpFrame {
    channel: u16,
    performative: Performative,
}

impl AmqpFrame {
    pub fn new(channel: u16, performative: Performative) -> Self {
        AmqpFrame {
            channel,
            performative,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut data = self.performative.encode();
        data.prepend(&mut self.channel.to_be_bytes().to_vec());
        data.prepend(&mut vec![AMQP_FRAME]);
        data.prepend(&mut vec![2]);
        // adjust size by + 4 to include the 4 bytes of the size itself
        let mut size = ((data.len() + 4) as u32).to_be_bytes().to_vec();
        data.prepend(&mut size);
        data
    }

    pub fn try_decode(doff: u8, stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let channel = u16::from_be_bytes(read_bytes_2(stream)?);
        skip_extended_header(doff, stream);
        let performative = Performative::try_decode(stream)?;
        Ok(AmqpFrame::new(channel, performative))
    }
}

fn skip_extended_header(doff: u8, stream: &mut IntoIter<u8>) {
    if doff == 2 {
        return;
    }
    for _ in 0..(doff * 4) - 8 {
        stream.next();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode_decode_round_trip_amqp_frame_open() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_begin() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_attach() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_flow() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_transfer() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_disposition() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_detach() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_end() {
        todo!()
    }
    #[test]
    fn test_encode_decode_round_trip_amqp_frame_close() {
        todo!()
    }
}
