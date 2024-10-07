use core::fmt::Display;

#[repr(u8)]
pub enum ErrorCode {
    InvalidRequest = 0x01,
    Busy = 0x03,
    UnexpectedRequest = 0x04,
    Unspecified = 0x05,
    UnsupportedRequest = 0x07,
    MajorVersionMismatch = 0x41,
    ResponseNotReady = 0x42,
    RequestResynch = 0x43,
    VendorStandardsDefined = 0xFF,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum RequestResponseCode {
    // SPDM 1.0 Request code
    GET_DIGESTS = 0x81,
    GET_CERTIFICATE = 0x82,
    CHALLENGE = 0x83,
    GET_VERSION = 0x84,
    GET_MEASUREMENTS = 0xE0,
    GET_CAPABILITIES = 0xE1,
    NEGOTIATE_ALGORITHMS = 0xE3,
    RESPOND_IF_READY = 0xFF,
    VENDOR_DEFINED_REQUEST = 0xFE,
    // SPDM 1.0 Response code
    DIGESTS = 0x01,
    CERTIFICATE = 0x02,
    CHALLENGE_AUTH = 0x03,
    VERSION = 0x04,
    MEASUREMENTS = 0x60,
    CAPABILITIES = 0x61,
    ALGORITHMS = 0x63,
    VENDOR_DEFINED_RESPONSE = 0x7E,
    ERROR = 0x7F,
}

impl TryFrom<u8> for RequestResponseCode {
    type Error = ErrorCode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // SPDM 1.0 RequestResponseCode
            0x81 => Ok(Self::GET_DIGESTS),
            0x82 => Ok(Self::GET_CERTIFICATE),
            0x83 => Ok(Self::CHALLENGE),
            0x84 => Ok(Self::GET_VERSION),
            0xE0 => Ok(Self::GET_MEASUREMENTS),
            0xE1 => Ok(Self::GET_CAPABILITIES),
            0xE3 => Ok(Self::NEGOTIATE_ALGORITHMS),
            0xFF => Ok(Self::RESPOND_IF_READY),
            0xFE => Ok(Self::VENDOR_DEFINED_REQUEST),
            0x01 => Ok(Self::DIGESTS),
            0x02 => Ok(Self::CERTIFICATE),
            0x03 => Ok(Self::CHALLENGE_AUTH),
            0x04 => Ok(Self::VERSION),
            0x60 => Ok(Self::MEASUREMENTS),
            0x61 => Ok(Self::CAPABILITIES),
            0x63 => Ok(Self::ALGORITHMS),
            0x7E => Ok(Self::VENDOR_DEFINED_RESPONSE),
            0x7F => Ok(Self::ERROR),

            _ => Err(ErrorCode::InvalidRequest),
        }
    }
}

impl Display for RequestResponseCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}(0x{:02x})", self, *self as u8)
    }
}

pub(super) mod CAPABILITIES;
pub(super) mod GET_CAPABILITIES;
pub(super) mod GET_VERSION;
pub(super) mod VERSION;

#[test]
fn test_RequestResponseCode_output() {
    let buffer = &mut [0u8; 1024][..];
    assert_eq!(
        "ALGORITHMS(0x63)",
        format_no_std::show(buffer, format_args!("{}", RequestResponseCode::ALGORITHMS)).unwrap()
    );
}
