#![allow(dead_code, unused_variables)]
use napi::{Result, Error, Status::GenericFailure};
use uuid::Uuid;

pub mod prelude {
  pub type VarInt = u32;
  // u64 is used for varlongs but we must use bigint
  pub type VarLong = napi::bindgen_prelude::BigInt;
  pub type ZigZag = i32;
  // i64 is used for zigzaglongs but we must use bigint
  pub type ZigZong = napi::bindgen_prelude::BigInt;
  pub type LU16 = u16;
  pub type LI16 = i16;
  pub type LI32 = i32;
  // Napi doesn't support f32 so internally we will convert to f32
  // when serializing and deserializing.
  pub type LF32 = f64;
  pub type LU64 = napi::bindgen_prelude::BigInt;
  pub type LI64 = napi::bindgen_prelude::BigInt;

  // Some weird support bullshit mojang has some strings that are
  // not sized with varint for some reason.
  pub type LittleString = String;

  pub type UUID = String;
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
    let mut value = value;

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

// Read/Write ZigZag with Result and NapiError
impl BinaryStream {
  pub fn write_zigzag(&mut self, value: ZigZag) -> Result<()> {
    self.write_varint(((value << 1) ^ (value >> 31)) as VarInt)?;

    Ok(())
  }
  pub fn read_zigzag(&mut self) -> Result<ZigZag> {
    let value = self.read_varint()?;

    Ok(((value >> 1) as ZigZag) ^ (-((value & 1) as ZigZag)))
  }
}

// Read/Write varlong with Result and NapiError
impl BinaryStream {
  pub fn write_varlong(&mut self, value: VarLong) -> Result<()> {
    let mut value = value.get_u64().1;

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
  pub fn read_varlong(&mut self) -> Result<VarLong> {
    let mut num_read = 0;
    let mut result = 0;

    loop {
      let read = self.read_u8()? as u64;
      let value = read & 0b01111111;

      result |= value << (7 * num_read);

      num_read += 1;

      if num_read > 10 {
        return Err(Error::new(
          GenericFailure,
          "VarLong is too big",
        ));
      }

      if (read & 0b10000000) == 0 {
        break;
      }
    }

    Ok(napi::bindgen_prelude::BigInt::from(result))
  }
}

// TODO: this is very unoptimized because not only is it reading from a bigint then reconverting to a bigint
// but napi has a bug so we also have to manually handle the sign before converting it to and from bigint
// Read/Write ZigZong with Result and NapiError
impl BinaryStream {
  pub fn write_zigzong(&mut self, value: ZigZong) -> Result<()> {
    // Bug in napi-rs bigints will not read as negative so we have to read it as unsigned
    // and apply the sign based of the sign boolean in the tuple
    let converted = value.get_u64();
    let value = match converted.0 {
      true => -(converted.1 as i64),
      false => converted.1 as i64,
    };    

    let value = napi::bindgen_prelude::BigInt::from(
      ((value << 1) ^ (value >> 63)) as u64
    );
    self.write_varlong(value)?;

    Ok(())
  }
  pub fn read_zigzong(&mut self) -> Result<ZigZong> {
    let value = self.read_varlong()?;
    let value = value.get_u64().1;

    let value = ((value >> 1) as i64) ^ (-((value & 1) as i64));
    let signed = value < 0;

    let value = match signed {
      true => (-value) as u64,
      false => value as u64,
    };
    
    let value = napi::bindgen_prelude::BigInt {
      sign_bit: signed,
      words: vec![value],
    };

    Ok(value)
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

// Read/Write UUID with Result and NapiError
impl BinaryStream {
  pub fn write_uuid(&mut self, value: UUID) -> Result<()> {
    // if empty string then write 16 0 bytes
    if value == "" {
      self.data.extend_from_slice(&[0u8; 16]);
      return Ok(());
    }

    let uid = match Uuid::parse_str(&value) {
      Ok(uuid) => uuid,
      Err(_) => return Err(Error::new(
        GenericFailure,
        "Invalid UUID at write_uuid",
      )),
    };

    // no need to verify it tbh
    // if uid.get_version() != Some(uuid::Version::Random) {
    //   return Err(Error::new(
    //     GenericFailure,
    //     "Invalid uuid not v4 at write_uuid",
    //   ));
    // }

    self.data.extend_from_slice(uid.as_bytes());

    Ok(())
  }

  pub fn read_uuid(&mut self) -> Result<UUID> {
    let mut bytes = [0u8; 16];

    // Check if the cursor is out of bounds.
    if self.cursor + 16 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_uuid",
      ));
    }

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 16]);

    self.cursor += 16;
 
    // if all bytes are 0 then return empty string
    if bytes.iter().all(|&x| x == 0) {
      return Ok(String::new());
    }

    // No validation occuring here could be bad...?  
    Ok(Uuid::from_bytes(bytes).to_string())
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

// read/write LI64 with Result and NapiError
impl BinaryStream {
  pub fn write_li64(&mut self, value: LI64) -> Result<()> {
    // Bug in napi-rs bigints will not read as negative so we have to read it as unsigned
    // and apply the sign based of the sign boolean in the tuple
    let converted = value.get_u64();
    let value = match converted.0 {
      true => -(converted.1 as i64),
      false => converted.1 as i64,
    };    

    self.data.extend_from_slice(&value.to_le_bytes());

    Ok(())
  }
  pub fn read_li64(&mut self) -> Result<ZigZong> {
    // Check if the cursor is out of bounds.
    if self.cursor + 8 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_li64",
      ));
    }

    let mut bytes = [0; 8];
    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 8]);
    self.cursor += 8;

    let value = i64::from_le_bytes(bytes);
    let signed = value < 0;

    let value = match signed {
      true => (-value) as u64,
      false => value as u64,
    };
    
    let value = napi::bindgen_prelude::BigInt {
      sign_bit: signed,
      words: vec![value],
    };

    Ok(value)
  }
}

// read/write U64 with Result and NapiError
impl BinaryStream {
  pub fn write_lu64(&mut self, value: LU64) -> Result<()> {
    self.data.extend_from_slice(&value.get_u64().1.to_le_bytes());

    Ok(())
  }

  pub fn read_lu64(&mut self) -> Result<LU64> {
    // Check if the cursor is out of bounds.
    if self.cursor + 8 > self.data.len() {
      return Err(Error::new(
          GenericFailure,
          "Cursor out of bounds at read_lu64",
      ));
    }

    let mut bytes = [0; 8];

    bytes.copy_from_slice(&self.data[self.cursor..self.cursor + 8]);

    self.cursor += 8;

    

    Ok(napi::bindgen_prelude::BigInt::from(u64::from_le_bytes(bytes)))
  }
}
