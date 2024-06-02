use nih_plug::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EartrainerSysExMessage {
    SetTonic(u8),
}
impl SysExMessage for EartrainerSysExMessage {
    type Buffer = [u8; 6];

    fn from_buffer(buffer: &[u8]) -> Option<Self> {
        // `buffer` contains the entire buffer, including headers and the 0xf7 End Of system
        // eXclusive byte
        match buffer {
            [0xf0, 0x69, 0x01, tonic, 0xf7] => Some(EartrainerSysExMessage::SetTonic(*tonic)),
            _ => None,
        }
    }

    fn to_buffer(self) -> (Self::Buffer, usize) {
        // `Self::Buffer` needs to have a fixed size, so the result needs to be padded, and we
        // return the message's actual length in bytes alongside it so the caller can trim the
        // excess padding
        match self {
            EartrainerSysExMessage::SetTonic(tonic) => ([0xf0, 0x69, 0x01, tonic, 0xf7, 0], 5),
        }
    }
}

