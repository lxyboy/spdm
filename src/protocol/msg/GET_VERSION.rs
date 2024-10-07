use super::{ErrorCode, RequestResponseCode, OFFSET};
use crate::utils::{Bytes, Reader, Writer};

pub struct GET_VERSION {
    SPDMVersion: u8,
    RequestResponseCode: u8,
    Param1: u8,
    Param2: u8,
}

impl Default for GET_VERSION {
    fn default() -> Self {
        Self {
            SPDMVersion: 0x10,
            RequestResponseCode: RequestResponseCode::GET_VERSION as u8,
            Param1: 0,
            Param2: 0,
        }
    }
}

impl GET_VERSION {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validation() -> Result<(), ErrorCode> {
        // TODO
        Ok(())
    }
}

impl Bytes for GET_VERSION {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        let length = OFFSET::Param2 + 1;
        if bytes.len() < length {
            return Err(length);
        }
        let reader = Reader::init(bytes);
        (self.SPDMVersion, _) = reader.read::<u8>(OFFSET::SPDMVersion).unwrap();
        (self.RequestResponseCode, _) = reader.read::<u8>(OFFSET::RequestResponseCode).unwrap();
        (self.Param1, _) = reader.read::<u8>(OFFSET::Param1).unwrap();
        (self.Param2, _) = reader.read::<u8>(OFFSET::Param2).unwrap();
        Ok(length)
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        let length = OFFSET::Param2 + 1;
        if bytes.len() < length {
            return Err(length);
        }
        let mut writer = Writer::init(bytes);
        writer
            .write(&self.SPDMVersion, OFFSET::SPDMVersion)
            .unwrap();
        writer
            .write(&self.RequestResponseCode, OFFSET::RequestResponseCode)
            .unwrap();
        writer.write(&self.Param1, OFFSET::Param1).unwrap();
        writer.write(&self.Param2, OFFSET::Param2).unwrap();
        Ok(length)
    }
}
