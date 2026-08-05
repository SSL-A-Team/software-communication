#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ErrorTelemetry {
    pub timestamp:      u32,
    pub error_message:  [u8; 60],
}
const _: () = assert!(core::mem::size_of::<ErrorTelemetry>() == 64);

impl Default for ErrorTelemetry {
    fn default() -> Self {
        Self { timestamp: 0, error_message: [0u8; 60] }
    }
}
