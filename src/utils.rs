pub trait Bytes: Sized {
    /// Read T from bytes
    /// Return Ok(number of bytes read from bytes), Err(required number of bytes to read)
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize>;
    /// Write T to bytes
    /// Return Ok(number of bytes write to bytes)
    /// Return Err(required number of bytes to write)
    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize>;
}

/// Read from a byte slice.
pub struct Reader<'a> {
    buf: &'a [u8],
    offset: usize,
}
impl<'a> Reader<'a> {
    pub fn init(bytes: &[u8]) -> Reader {
        Reader {
            buf: bytes,
            offset: 0,
        }
    }

    pub fn left_slice(&self) -> &[u8] {
        &self.buf[self.offset..]
    }

    pub fn used(&self) -> usize {
        self.offset
    }

    pub fn left(&self) -> usize {
        self.buf.len() - self.offset
    }

    /// Return Ok(T)
    /// Return Err(size) size is required
    pub fn read<T: Bytes + Default>(&mut self) -> Result<T, usize> {
        let mut ret: T = T::default();
        let res = ret.from_bytes(self.left_slice())?;
        self.offset += res;
        Ok(ret)
    }
}

pub struct Writer<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Writer<'a> {
    pub fn init(bytes: &mut [u8]) -> Writer {
        Writer {
            buf: bytes,
            offset: 0,
        }
    }

    pub fn extend_from_slice(&mut self, value: &[u8]) -> Result<usize, usize> {
        if self.left() < value.len() {
            return Err(value.len());
        }
        let added = value.len();
        for (i, v) in value.iter().enumerate().take(added) {
            self.buf[self.offset + i] = *v;
        }
        self.offset += added;
        Ok(added)
    }
    fn mut_left_slice(&mut self) -> &mut [u8] {
        &mut self.buf[self.offset..]
    }
    pub fn write<T: Bytes>(&mut self, value: T) -> Result<usize, usize> {
        value.to_bytes(self.mut_left_slice())
    }
    pub fn left(&self) -> usize {
        self.buf.len() - self.offset
    }

    pub fn used(&self) -> usize {
        self.offset
    }
}

impl Bytes for u8 {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        type T = u8;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            *self = T::from_le_bytes(bytes[0..size].try_into().map_err(|_e| return size)?);
            Ok(size)
        }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        type T = u8;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            bytes[0..size].copy_from_slice(&self.to_le_bytes()[..]);
            Ok(size)
        }
    }
}

impl Bytes for u16 {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        type T = u16;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            *self = T::from_le_bytes(bytes[0..size].try_into().map_err(|_e| return size)?);
            Ok(size)
        }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        type T = u16;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            bytes[0..size].copy_from_slice(&self.to_le_bytes()[..]);
            Ok(size)
        }
    }
}

impl Bytes for u32 {
    fn from_bytes(&mut self, bytes: &[u8]) -> Result<usize, usize> {
        type T = u32;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            *self = T::from_le_bytes(bytes[0..size].try_into().map_err(|_e| return size)?);
            Ok(size)
        }
    }

    fn to_bytes(&self, bytes: &mut [u8]) -> Result<usize, usize> {
        type T = u32;
        let size = size_of::<T>();
        if bytes.len() < size {
            Err(size)
        } else {
            bytes[0..size].copy_from_slice(&self.to_le_bytes()[..]);
            Ok(size)
        }
    }
}
