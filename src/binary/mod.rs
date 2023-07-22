use napi::{Result, Error, Status::GenericFailure};

pub type U8 = u8;
pub type I32 = i32;
pub type VarInt = i32;


pub struct BinaryStream {
  pub data: Vec<u8>,
  pub cursor: usize,
}

// Constructors
impl BinaryStream {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      cursor: 0,
    }
  }

  pub fn from(data: Vec<u8>) -> Self {
    Self {
      data,
      cursor: 0,
    }
  }
}

// Read/Write U8 using Result with NapiError
impl BinaryStream {
  pub fn write_u8(&mut self, value: U8) -> Result<()> {
    self.data.push(value);

    Ok(())
  }

  pub fn read_u8(&mut self) -> Result<U8> {
    // Check if the cursor is out of bounds.
    if self.cursor >= self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds",
      ));
    }

    let value = self.data[self.cursor];
    self.cursor += 1;

    Ok(value)
  }
}

// Read/Write I32 using Result with NapiError
impl BinaryStream {
  pub fn write_i32(&mut self, value: I32) -> Result<()> {
    self.data.extend_from_slice(&value.to_be_bytes());

    Ok(())
  }

  pub fn read_i32(&mut self) -> Result<I32> {
    // Check if the cursor is out of bounds.
    if self.cursor + 4 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds",
      ));
    }

    let mut bytes = [0; 4];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 4]);

    self.cursor += 4;

    Ok(I32::from_be_bytes(bytes))
  }
}

// Read/Write VarInt with Result and NapiError
impl BinaryStream {
  pub fn write_varint(&mut self, value: VarInt) -> Result<()> {
    let mut value = value as u32;

    loop {
      let mut temp = (value & 0b01111111) as u8;

      value >>= 7;

      if value != 0 {
        temp |= 0b10000000;
      }

      self.data.push(temp);

      if value == 0 {
        break;
      }
    }

    Ok(())
  }

  // Make this safe like the above read_u8 and read_i32
  pub fn read_varint(&mut self) -> Result<VarInt> {
    let mut num_read = 0;
    let mut result = 0;

    loop {
      let read = self.read_u8()? as u32;
      let value = read & 0b01111111;

      result |= value << (7 * num_read);

      num_read += 1;

      if num_read > 5 {
        return Err(Error::new(
          GenericFailure,
          "VarInt is too big",
        ));
      }

      if (read & 0b10000000) == 0 {
        break;
      }
    }

    Ok(result as VarInt)
  }
}
