use super::RequestResponseCode;
use crate::utils::{Bytes, Reader, Writer};

pub const MAX_VERSION_NUMBER_ENTRY: usize = 10;

#[derive(Default, Clone, Copy)]
pub struct VersionNumberEntry(u16);

impl VersionNumberEntry {
    pub fn MajorVersion(&self) -> u8 {
        Self::get_bits(self.0, 12, 15)
    }

    pub fn MinorVersion(&self) -> u8 {
        Self::get_bits(self.0, 8, 11)
    }

    pub fn UpdateVersionNumber(&self) -> u8 {
        Self::get_bits(self.0, 4, 7)
    }

    pub fn Alpha(&self) -> u8 {
        Self::get_bits(self.0, 0, 3)
    }

    fn get_bits(value: u16, start: usize, end: usize) -> u8 {
        const LENGTH: usize = 16usize;
        let value = value << LENGTH - (end + 1) >> LENGTH - (end + 1);
        let value = value >> start;
        value as u8
    }
}

pub struct VERSION {
    pub SPDMVersion: u8,
    pub RequestResponseCode: u8,
    pub Param1: u8,
    pub Param2: u8,
    pub Reserved: u8,
    pub VersionNumberEntryCount: u8,
    pub VersionNumberEntry1_n: [VersionNumberEntry; MAX_VERSION_NUMBER_ENTRY],
}

impl Default for VERSION {
    fn default() -> Self {
        Self {
            SPDMVersion: 0x10,
            RequestResponseCode: RequestResponseCode::VERSION as u8,
            Param1: 0,
            Param2: 0,
            Reserved: 0,
            VersionNumberEntryCount: 0,
            VersionNumberEntry1_n: [VersionNumberEntry::default(); MAX_VERSION_NUMBER_ENTRY],
        }
    }
}

impl Bytes for VERSION {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        let mut reader = Reader::init(bytes);
        self.SPDMVersion = reader.read::<u8>().unwrap();
        self.RequestResponseCode = reader.read::<u8>().unwrap();
        self.Param1 = reader.read::<u8>().unwrap();
        self.Param2 = reader.read::<u8>().unwrap();
        self.VersionNumberEntryCount = reader.read::<u8>().unwrap();
        todo!()
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        todo!()
    }
}

#[test]
fn test_spdm_version_entry() {
    let v = VersionNumberEntry(0x1234);
    assert_eq!(v.MajorVersion(), 1);
    assert_eq!(v.MinorVersion(), 2);
    assert_eq!(v.UpdateVersionNumber(), 3);
    assert_eq!(v.Alpha(), 4);
}
