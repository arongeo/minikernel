//
//! error codes definiton module
//  copyright 2022 - arongeo
//  https://arongeo.com
//

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    GPIOStatusUnwriteable,
    GPIOPinOutOfBounds,
    GPIOPinUsedByOtherProcess,
    GPIOPinWrongFunction,
    DRIVERSMiniUARTInaccessible,
    DRIVERSGPIOHandlerInaccessible,
}
