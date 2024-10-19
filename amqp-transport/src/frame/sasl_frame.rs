use amqp_error::AppError;
use std::vec::IntoIter;

pub struct SaslFrame {}

impl SaslFrame {
    pub(crate) fn encode(self) -> Vec<u8> {
        todo!()
    }
}

impl SaslFrame {
    pub fn try_decode(_doff: u8, _stream: &mut IntoIter<u8>) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    //TODO: write tests
}
