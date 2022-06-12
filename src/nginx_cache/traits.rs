pub trait ByteArrayToInt {
  fn as_i64(&self) -> i64;
  fn as_u64(&self) -> u64;
  fn as_usize(&self) -> usize;
}
impl ByteArrayToInt for [u8; 8] {
  fn as_i64(&self) -> i64 {
    let (_, rest) = self.split_at(std::mem::size_of::<i32>());
    return u32::from_le_bytes(rest.try_into().unwrap()) as i64
  }
  fn as_u64(&self) -> u64 {
    let (_, rest) = self.split_at(std::mem::size_of::<i32>());
    return u32::from_le_bytes(rest.try_into().unwrap()) as u64
  }
  fn as_usize(&self) -> usize {
    let (_, rest) = self.split_at(std::mem::size_of::<i32>());
    return u32::from_le_bytes(rest.try_into().unwrap()) as usize
  }
}
impl ByteArrayToInt for [u8; 4] {
  fn as_i64(&self) -> i64{
    return u32::from_le_bytes(*self) as i64;
  }
  fn as_u64(&self) -> u64{
    return u32::from_le_bytes(*self) as u64;
  }
  fn as_usize(&self) -> usize{
    return u32::from_le_bytes(*self) as usize;
  }
}
impl ByteArrayToInt for [u8; 2] {
  fn as_i64(&self) -> i64{
    return u16::from_le_bytes(*self) as i64;
  }
  fn as_u64(&self) -> u64{
    return u16::from_le_bytes(*self) as u64;
  }
  fn as_usize(&self) -> usize {
    return u16::from_le_bytes(*self) as usize;
  }
}
impl ByteArrayToInt for [u8; 1] {
  fn as_i64(&self) -> i64{
    return i64::from_le(self[0].into());
  }
  fn as_u64(&self) -> u64{
    return u64::from_le(self[0].into());
  }

  fn as_usize(&self) -> usize {
    return i64::from_le(self[0].into()) as usize;
  }
}
