use super::RequestResponseCode;
use crate::utils::{Bytes, Reader, Writer};

pub const MAX_VERSION_NUMBER_ENTRY: usize = 10;
pub mod OFFSET {
    pub use crate::protocol::msg::OFFSET::*;
    pub const Reserved: usize = 4;
    pub const VersionNumberEntryCount: usize = 5;
    pub const VersionNumberEntry1_N: usize = 6;
}

#[derive(Default, Clone, Copy)]
#[repr(C)]
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

impl Bytes for VersionNumberEntry {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        self.0.from_bytes(bytes)
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        self.0.to_bytes(bytes)
    }
}

pub struct VERSION {
    pub SPDMVersion: u8,
    pub RequestResponseCode: u8,
    pub Param1: u8,
    pub Param2: u8,
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
            VersionNumberEntryCount: 0,
            VersionNumberEntry1_n: [VersionNumberEntry::default(); MAX_VERSION_NUMBER_ENTRY],
        }
    }
}

impl Bytes for VERSION {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        let bytes_len = bytes.len();
        if bytes_len < OFFSET::VersionNumberEntryCount {
            return Err(OFFSET::VersionNumberEntryCount + 1);
        }
        let mut VersionNumberEntry1_n_size = 0usize;
        let reader = Reader::init(bytes);
        (self.SPDMVersion, _) = reader.read::<u8>(OFFSET::SPDMVersion).unwrap();
        (self.RequestResponseCode, _) = reader.read::<u8>(OFFSET::RequestResponseCode).unwrap();
        (self.Param1, _) = reader.read::<u8>(OFFSET::Param1).unwrap();
        (self.Param2, _) = reader.read::<u8>(OFFSET::Param2).unwrap();
        (self.VersionNumberEntryCount, _) =
            reader.read::<u8>(OFFSET::VersionNumberEntryCount).unwrap();
        (self.VersionNumberEntry1_n, VersionNumberEntry1_n_size) = reader
            .read_n::<VersionNumberEntry, MAX_VERSION_NUMBER_ENTRY>(
            OFFSET::VersionNumberEntry1_N,
            self.VersionNumberEntryCount as usize,
        )?;
        Ok(OFFSET::VersionNumberEntry1_N + VersionNumberEntry1_n_size)
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        let bytes_len = bytes.len();
        if bytes_len < OFFSET::VersionNumberEntryCount {
            return Err(OFFSET::VersionNumberEntryCount + 1);
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
        writer.write(&0u8, OFFSET::Reserved).unwrap();
        writer
            .write(
                &self.VersionNumberEntryCount,
                OFFSET::VersionNumberEntryCount,
            )
            .unwrap();
        let result = writer.write_n(
            &self.VersionNumberEntry1_n[0..(self.VersionNumberEntryCount as usize)],
            OFFSET::VersionNumberEntry1_N,
        )?;
        Ok(result + OFFSET::VersionNumberEntry1_N)
    }
}

#[test]
fn test_spdm_version_entry() {
    let v = VersionNumberEntry(0x1234);
    assert_eq!(v.MajorVersion(), 1);
    assert_eq!(v.MinorVersion(), 2);
    assert_eq!(v.UpdateVersionNumber(), 3);
    assert_eq!(v.Alpha(), 4);

    assert_eq!(size_of::<VersionNumberEntry>(), 2);
}
