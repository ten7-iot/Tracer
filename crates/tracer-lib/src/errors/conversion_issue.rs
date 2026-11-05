#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConvertError {
    OutOfRange = 0x01,
    OutOfBounds = 0x02,
    FailedToConvert = 0x03,
}