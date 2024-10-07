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
}
impl<'a> Reader<'a> {
    pub fn init(bytes: &[u8]) -> Reader {
        Reader { buf: bytes }
    }

    /// Return Ok((T, sizeof<T>))
    /// Return Err(offset + sizeof<T>) overflow
    pub fn read<T: Bytes + Default>(&self, offset: usize) -> Result<(T, usize), usize> {
        if offset > self.buf.len() {
            return Err(offset);
        }
        let mut ret: T = T::default();
        let length = ret
            .from_bytes(&self.buf[offset..])
            .map_err(|required_size| required_size + offset)?;
        Ok((ret, length))
    }

    /// Return Ok(T)
    /// Return Err(offset + n * sizeof<T>) overflow
    pub fn read_n<T: Bytes + Copy + Default, const N: usize>(
        &self,
        offset: usize,
        count: usize,
    ) -> Result<([T; N], usize), usize> {
        let mut ret = [T::default(); N];
        let mut size = 0;
        for index in 0..count {
            size += ret[index]
                .from_bytes(&self.buf[(offset + size)..])
                .map_err(|required_size| required_size + offset + size)?;
        }
        Ok((ret, size))
    }
}

pub struct Writer<'a> {
    buf: &'a mut [u8],
}

impl<'a> Writer<'a> {
    pub fn init(bytes: &mut [u8]) -> Writer {
        Writer { buf: bytes }
    }

    pub fn write<T: Bytes>(&mut self, value: &T, offset: usize) -> Result<usize, usize> {
        if offset > self.buf.len() {
            return Err(offset);
        }
        value
            .to_bytes(&mut self.buf[offset..])
            .map_err(|required_size| required_size + offset)
    }

    pub fn write_n<T: Bytes>(&mut self, value: &[T], offset: usize) -> Result<usize, usize> {
        let mut nwrite = 0;
        for v in value.iter() {
            nwrite += self.write(v, offset + nwrite)?;
        }
        return Ok(nwrite);
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
