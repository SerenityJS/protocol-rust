#![allow(dead_code, unused_variables)]
use napi::{Result, Error, Status::GenericFailure};

pub mod prelude {
  pub type VarInt = u32;
  pub type LU16 = u16;
  pub type LI16 = i16;
  pub type LI32 = i32;
  // Napi doesn't support f32 so internally we will convert to f32
  // when serializing and deserializing.
  pub type LF32 = f64;
  pub type U64 = napi::bindgen_prelude::BigInt;

  // Some weird support bullshit mojang has some strings that are
  // not sized with varint for some reason.
  pub type LittleString = String;
}

use prelude::*;

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

// Random methods
impl BinaryStream {
  pub fn empty(&self) -> bool {
    self.cursor == self.len()
  }
  pub fn len(&self) -> usize {
    self.data.len()
  }
}

// Append another binary stream to this one
impl BinaryStream {
  pub fn append(&mut self, other: &mut BinaryStream) {
    self.data.append(&mut other.data);
  }
  pub fn write(&mut self, other: Vec<u8>) -> Result<()> {
    self.data.extend_from_slice(&other);

    Ok(())
  }
  pub fn read(&mut self, len: usize) -> Result<Vec<u8>> {
    // Check if the cursor is out of bounds.
    if self.cursor + len > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read",
      ));
    }

    let mut bytes = vec![0; len];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + len]);
    self.cursor += len;

    Ok(bytes)
  }
}

// Read/Write U8 using Result with NapiError
impl BinaryStream {
  pub fn write_u8(&mut self, value: u8) -> Result<()> {
    self.data.push(value);

    Ok(())
  }

  pub fn read_u8(&mut self) -> Result<u8> {
    // Check if the cursor is out of bounds.
    if self.cursor >= self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_u8",
      ));
    }

    let value = self.data[self.cursor];
    self.cursor += 1;

    Ok(value)
  }
}

// Read/Write Bool using Result with NapiError
impl BinaryStream {
  pub fn write_bool(&mut self, value: bool) -> Result<()> {
    self.data.push(value as u8);

    Ok(())
  }

  pub fn read_bool(&mut self) -> Result<bool> {
    // Check if the cursor is out of bounds.
    if self.cursor >= self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_bool",
      ));
    }

    let value = self.data[self.cursor] != 0;
    self.cursor += 1;

    Ok(value)
  }
}

// Read/Write I32 using Result with NapiError
impl BinaryStream {
  pub fn write_i32(&mut self, value: i32) -> Result<()> {
    self.data.extend_from_slice(&value.to_be_bytes());

    Ok(())
  }

  pub fn read_i32(&mut self) -> Result<i32> {
    // Check if the cursor is out of bounds.
    if self.cursor + 4 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_i32",
      ));
    }

    let mut bytes = [0; 4];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 4]);

    self.cursor += 4;

    Ok(i32::from_be_bytes(bytes))
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

// Read/Write String with Result and NapiError
impl BinaryStream {
  pub fn write_string(&mut self, value: String) -> Result<()> {
    self.write_varint(value.len() as VarInt)?;

    self.data.extend_from_slice(value.as_bytes());

    Ok(())
  }

  pub fn read_string(&mut self) -> Result<String> {
    let length = self.read_varint()? as usize;

    // Check if the cursor is out of bounds.
    if self.cursor + length > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_string",
      ));
    }

    let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + length]).to_string();

    self.cursor += length;

    Ok(value)
  }
}

//Read/Write LittleString with Result and NapiError
impl BinaryStream {
  pub fn write_littlestring(&mut self, value: LittleString) -> Result<()> {
    self.write_li32(value.len() as i32)?;

    self.data.extend_from_slice(value.as_bytes());

    Ok(())
  }

  pub fn read_littlestring(&mut self) -> Result<LittleString> {
    let length = self.read_li32()? as usize;

    // Check if the cursor is out of bounds.
    if self.cursor + length > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_littlestring",
      ));
    }

    let value = String::from_utf8_lossy(&self.data[self.cursor..self.cursor + length]).to_string();

    self.cursor += length;

    Ok(value)
  }
}

// Read/Write LF32 with Result and NapiError
impl BinaryStream {
  pub fn write_lf32(&mut self, value: LF32) -> Result<()> {
    self.data.extend_from_slice(&(value as f32).to_le_bytes());

    Ok(())
  }

  pub fn read_lf32(&mut self) -> Result<LF32> {
    // Check if the cursor is out of bounds.
    if self.cursor + 4 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_lf32",
      ));
    }

    let mut bytes = [0; 4];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 4]);

    self.cursor += 4;

    Ok(f32::from_le_bytes(bytes) as f64)
  }
}

// Read/Write u16 with Result and NapiError
impl BinaryStream {
  pub fn write_u16(&mut self, value: u16) -> Result<()> {
    self.data.extend_from_slice(&value.to_be_bytes());

    Ok(())
  }

  pub fn read_u16(&mut self) -> Result<u16> {
    // Check if the cursor is out of bounds.
    if self.cursor + 2 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_u16",
      ));
    }

    let mut bytes = [0; 2];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 2]);

    self.cursor += 2;

    Ok(u16::from_be_bytes(bytes))
  }
}

// Read/Write LU16 with Result and NapiError
impl BinaryStream {
  pub fn write_lu16(&mut self, value: LU16) -> Result<()> {
    self.data.extend_from_slice(&value.to_le_bytes());

    Ok(())
  }

  pub fn read_lu16(&mut self) -> Result<LU16> {
    // Check if the cursor is out of bounds.
    if self.cursor + 2 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_lu16",
      ));
    }

    let mut bytes = [0; 2];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 2]);

    self.cursor += 2;

    Ok(u16::from_le_bytes(bytes))
  }
}

// Read/Write LU16 with Result and NapiError
impl BinaryStream {
  pub fn write_li16(&mut self, value: LI16) -> Result<()> {
    self.data.extend_from_slice(&value.to_le_bytes());

    Ok(())
  }

  pub fn read_li16(&mut self) -> Result<LI16> {
    // Check if the cursor is out of bounds.
    if self.cursor + 2 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_lu16",
      ));
    }

    let mut bytes = [0; 2];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 2]);

    self.cursor += 2;

    Ok(i16::from_le_bytes(bytes))
  }
}

// Read/Write LI32 with Result and NapiError
impl BinaryStream {
  pub fn write_li32(&mut self, value: LI32) -> Result<()> {
    self.data.extend_from_slice(&value.to_le_bytes());

    Ok(())
  }

  pub fn read_li32(&mut self) -> Result<LI32> {
    // Check if the cursor is out of bounds.
    if self.cursor + 4 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_li32",
      ));
    }

    let mut bytes = [0; 4];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 4]);

    self.cursor += 4;

    Ok(i32::from_le_bytes(bytes))
  }
}

// read/write U64 with Result and NapiError
impl BinaryStream {
  pub fn write_u64(&mut self, value: u64) -> Result<()> {
    self.data.extend_from_slice(&value.to_be_bytes());

    Ok(())
  }

  pub fn read_u64(&mut self) -> Result<u64> {
    // Check if the cursor is out of bounds.
    if self.cursor + 8 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_u64",
      ));
    }

    let mut bytes = [0; 8];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 8]);

    self.cursor += 8;

    

    Ok(u64::from_be_bytes(bytes))
  }
}
