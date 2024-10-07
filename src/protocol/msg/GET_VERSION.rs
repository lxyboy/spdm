use super::{ErrorCode, RequestResponseCode};
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
        let mut reader = Reader::init(bytes);
        if reader.left() < 4 {
            return Err(4);
        }
        self.SPDMVersion = reader.read::<u8>().unwrap();
        self.RequestResponseCode = reader.read::<u8>().unwrap();
        self.Param1 = reader.read::<u8>().unwrap();
        self.Param2 = reader.read::<u8>().unwrap();
        Ok(reader.used())
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        let mut writer = Writer::init(bytes);
        if writer.left() < 4 {
            return Err(4);
        }
        writer.write(self.SPDMVersion).unwrap();
        writer.write(self.RequestResponseCode).unwrap();
        writer.write(self.Param1).unwrap();
        writer.write(self.Param2).unwrap();
        Ok(writer.used())
    }
}
