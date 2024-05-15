mod abi;

pub use abi::*;
use bytes::{Bytes, BytesMut};
use prost::{DecodeError, EncodeError, Message};

impl MessageBo {
    pub fn new(msg: String) -> Self {
        Self { msg, ..Self::default() }
    }
}

// impl From<Bytes> for MessageBo {
//     fn from(value: Bytes) -> Self {
//         MessageBo::decode(value).unwrap()
//     }
// }
//
// impl From<BytesMut> for MessageBo {
//     fn from(value: BytesMut) -> Self {
//         let bytes = value.freeze();
//         bytes.into()
//     }
// }
//
// impl From<MessageBo> for Bytes {
//     fn from(value: MessageBo) -> Self {
//         let mut buf = Vec::new();
//         MessageBo::encode(&value, &mut buf).unwrap();
//         buf.into()
//     }
// }

impl TryFrom<Bytes> for MessageBo {
    type Error = DecodeError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        MessageBo::decode(value)
    }
}

impl TryFrom<BytesMut> for MessageBo {
    type Error = DecodeError;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        let bytes = value.freeze();
        bytes.try_into()
    }
}

impl TryFrom<MessageBo> for Bytes {
    type Error = EncodeError;

    fn try_from(value: MessageBo) -> Result<Self, Self::Error> {
        let mut buf = Vec::new();
        value.encode(&mut buf)?;
        Ok(buf.into())
    }
}