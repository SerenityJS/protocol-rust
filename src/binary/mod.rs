// Binary utilities for reading and writing binary data
// for Minecraft Bedrock Edition packets;

// Used for reading and writing binary data

#![allow(dead_code, unused_variables)]
pub enum Endianess {
  Big,
  Little,
}

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

// Read/Write signed integers
impl BinaryStream {
  pub fn write_i8(&mut self, value: i8) {
    self.data.push(value as u8);
  }

  pub fn write_i16(&mut self, value: i16, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_i32(&mut self, value: i32, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_i64(&mut self, value: i64, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_i128(&mut self, value: i128, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn read_i8(&mut self) -> i8 {
    let value = self.data[self.cursor];
    self.cursor += 1;
    value as i8
  }

  pub fn read_i16(&mut self, endianess: Endianess) -> i16 {
    let mut bytes = [0; 2];
    for i in 0..2 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => i16::from_be_bytes(bytes),
      Endianess::Little => i16::from_le_bytes(bytes),
    }
  }

  pub fn read_i32(&mut self, endianess: Endianess) -> i32 {
    let mut bytes = [0; 4];
    for i in 0..4 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => i32::from_be_bytes(bytes),
      Endianess::Little => i32::from_le_bytes(bytes),
    }
  }

  pub fn read_i64(&mut self, endianess: Endianess) -> i64 {
    let mut bytes = [0; 8];
    for i in 0..8 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => i64::from_be_bytes(bytes),
      Endianess::Little => i64::from_le_bytes(bytes),
    }
  }

  pub fn read_i128(&mut self, endianess: Endianess) -> i128 {
    let mut bytes = [0; 16];
    for i in 0..16 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => i128::from_be_bytes(bytes),
      Endianess::Little => i128::from_le_bytes(bytes),
    }
  }
}

// Read/Write unsigned integers
impl BinaryStream {
  pub fn write_u8(&mut self, value: u8) {
    self.data.push(value);
  }

  pub fn write_u16(&mut self, value: u16, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_u32(&mut self, value: u32, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_u64(&mut self, value: u64, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_u128(&mut self, value: u128, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn read_u8(&mut self) -> u8 {
    let value = self.data[self.cursor];
    self.cursor += 1;
    value
  }

  pub fn read_u16(&mut self, endianess: Endianess) -> u16 {
    let mut bytes = [0; 2];
    for i in 0..2 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => u16::from_be_bytes(bytes),
      Endianess::Little => u16::from_le_bytes(bytes),
    }
  }

  pub fn read_u32(&mut self, endianess: Endianess) -> u32 {
    let mut bytes = [0; 4];
    for i in 0..4 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => u32::from_be_bytes(bytes),
      Endianess::Little => u32::from_le_bytes(bytes),
    }
  }

  pub fn read_u64(&mut self, endianess: Endianess) -> u64 {
    let mut bytes = [0; 8];
    for i in 0..8 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => u64::from_be_bytes(bytes),
      Endianess::Little => u64::from_le_bytes(bytes),
    }
  }

  pub fn read_u128(&mut self, endianess: Endianess) -> u128 {
    let mut bytes = [0; 16];
    for i in 0..16 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => u128::from_be_bytes(bytes),
      Endianess::Little => u128::from_le_bytes(bytes),
    }
  }
}

// Read/Write floats
impl BinaryStream {
  pub fn write_f32(&mut self, value: f32, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn write_f64(&mut self, value: f64, endianess: Endianess) {
    match endianess {
      Endianess::Big => self.data.extend(value.to_be_bytes().iter()),
      Endianess::Little => self.data.extend(value.to_le_bytes().iter()),
    }
  }

  pub fn read_f32(&mut self, endianess: Endianess) -> f32 {
    let mut bytes = [0; 4];
    for i in 0..4 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => f32::from_be_bytes(bytes),
      Endianess::Little => f32::from_le_bytes(bytes),
    }
  }

  pub fn read_f64(&mut self, endianess: Endianess) -> f64 {
    let mut bytes = [0; 8];
    for i in 0..8 {
      bytes[i] = self.data[self.cursor];
      self.cursor += 1;
    }
    match endianess {
      Endianess::Big => f64::from_be_bytes(bytes),
      Endianess::Little => f64::from_le_bytes(bytes),
    }
  }
}

// Read/Write varint varlong
impl BinaryStream {
  pub fn read_varint(&mut self) -> i32 {
    let mut num_read = 0;
    let mut result = 0;
    let mut read: u8;
    loop {
      read = self.read_u8();
      let value = (read & 0b01111111) as i32;
      result |= value << (7 * num_read);
      num_read += 1;
      if num_read > 5 {
        panic!("VarInt is too big");
      }
      if (read & 0b10000000) == 0 {
        break;
      }
    }
    result
  }

  pub fn read_varlong(&mut self) -> i64 {
    let mut num_read = 0;
    let mut result = 0;
    let mut read: u8;
    loop {
      read = self.read_u8();
      let value = (read & 0b01111111) as i64;
      result |= value << (7 * num_read);
      num_read += 1;
      if num_read > 10 {
        panic!("VarLong is too big");
      }
      if (read & 0b10000000) == 0 {
        break;
      }
    }
    result
  }

  pub fn write_varint(&mut self, mut value: i32) {
    loop {
      let mut temp = (value & 0b01111111) as u8;
      value = value >> 7;
      if value != 0 {
        temp |= 0b10000000;
      }
      self.write_u8(temp);
      if value == 0 {
        break;
      }
    }
  }

  pub fn write_varlong(&mut self, mut value: i64) {
    loop {
      let mut temp = (value & 0b01111111) as u8;
      value = value >> 7;
      if value != 0 {
        temp |= 0b10000000;
      }
      self.write_u8(temp);
      if value == 0 {
        break;
      }
    }
  }
}

// Read/Write strings
impl BinaryStream {
  pub fn write_string(&mut self, value: &str) {
    let bytes = value.as_bytes();
    self.write_varint(bytes.len() as i32);
    self.data.extend(bytes.iter());
  }

  pub fn read_string(&mut self) -> String {
    let length = self.read_varint();
    let mut bytes = vec![0; length as usize];
    for i in 0..length {
      bytes[i as usize] = self.read_u8();
    }
    String::from_utf8(bytes).unwrap()
  }
}

// Read/Write bools
impl BinaryStream {
  pub fn write_bool(&mut self, value: bool) {
    self.write_u8(if value { 1 } else { 0 });
  }

  pub fn read_bool(&mut self) -> bool {
    self.read_u8() == 1
  }
}

// Read/Write LittleStrings which appears to be a u32 little endian
// followed by a string of that length
impl BinaryStream {
  pub fn write_little_string(&mut self, value: &str) {
    let bytes = value.as_bytes();
    self.write_u32(bytes.len() as u32, Endianess::Little);
    self.data.extend(bytes.iter());
  }

  pub fn read_little_string(&mut self) -> String {
    let length = self.read_u32(Endianess::Little);
    let mut bytes = vec![0; length as usize];
    for i in 0..length {
      bytes[i as usize] = self.read_u8();
    }
    String::from_utf8(bytes).unwrap()
  }
}
