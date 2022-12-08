
#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    GPIOStatusUnwriteable,
    GPIOPinOutOfBounds,
    GPIOPinUsedByOtherProcess,
    GPIOPinWrongFunction,
}
