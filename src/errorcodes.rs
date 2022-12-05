
#[derive(Debug)]
pub enum ErrorCode {
    GPIOStatusUnwriteable,
    GPIOPinOutOfBounds,
    GPIOPinUsedByOtherProcess,
}
