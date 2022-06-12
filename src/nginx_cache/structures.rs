use std::fmt;
use std::fs::File;
use std::io::{
  BufReader, 
  SeekFrom, 
  Seek, 
  Read, 
};
use std::str;
use std::iter::Rev;
use std::slice::Iter;

use crate::nginx_cache::traits::ByteArrayToInt;

type Result<T> = std::result::Result<T, NginxCacheError>;
#[derive(Debug)]
struct NgxInvalidFileType;
#[derive(Debug)]
pub enum NginxCacheError {
  NginxCache(Error),
  Io(std::io::Error)
}
#[derive(Debug)]
pub struct Error {
  kind:NgxInvalidFileType,
  message:String
}
impl core::fmt::Display for NginxCacheError {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    match &*self {
      NginxCacheError::NginxCache(m) => { write!(f, "{:?}: {}", m.kind, m.message.as_str()) },
      NginxCacheError::Io(..) => { write!(f, "Undefined IO error message") }
    }
  }
}
impl From<std::io::Error> for NginxCacheError {
  fn from(err: std::io::Error) -> NginxCacheError {
    NginxCacheError::Io(err)
  }
}

//https://github.com/nginx/nginx/blob/master/src/http/ngx_http_cache.h
pub struct  NgxHttpFileCacheHeader {
  pub version: [u8; 4],
  pub valid_sec: [u8; 8],
  pub updating_sec: [u8; 8],
  pub error_sec: [u8; 8],
  pub last_modified: [u8; 8],
  pub date: [u8; 8],
  pub crc32: [u8; 8],
  pub valid_msec: [u8; 2],
  pub header_start: [u8; 2],
  pub body_start: [u8; 2],
  pub etag_len: [u8; 1],
  pub etag_data: Vec<u8>,
  pub vary_len: [u8; 1],
  pub vary_data: Vec<u8>,
  pub variant_data: Vec<u8>, // with size of NGX_HTTP_CACHE_KEY_LEN

  pub header_data: Vec<u8>,
  pub body_data: Vec<u8>,

  pub key: String
}
impl NgxHttpFileCacheHeader {
  pub fn new() -> Self {
    return Self {
      version: [0; 4],
      valid_sec: [0; 8],
      updating_sec: [0; 8],
      error_sec: [0; 8],
      last_modified: [0; 8],
      date: [0; 8],
      crc32: [0; 8],
      valid_msec: [0; 2],
      header_start: [0; 2],
      body_start: [0; 2],
      etag_len: [0; 1],
      etag_data: Vec::new(),
      vary_len: [0; 1],
      vary_data: Vec::new(),
      variant_data: Vec::new(),

      header_data: Vec::new(),
      body_data: Vec::new(),

      key: String::from("")
    };
  }
  pub fn from_buffer(&mut self, buffer:&mut BufReader<File>) -> Result<()> {
    match Self::to_object(buffer) {
      Ok(val) => {
        *self = val;
        return Ok(());
      },
      Err(err) => { return Err(err); }
    }
  }
  pub fn to_object(buffer:&mut BufReader<File>) -> Result<NgxHttpFileCacheHeader> {
    let mut data = NgxHttpFileCacheHeader::new();

    buffer.read_exact(&mut data.version)?;
    if data.version != [5, 0, 0, 0] {
      return Err(NginxCacheError::NginxCache(Error{ message: String::from("Invalid file format"), kind:NgxInvalidFileType }));
    }
    buffer.read_exact(&mut data.valid_sec)?;
    buffer.read_exact(&mut data.updating_sec)?;
    buffer.read_exact(&mut data.error_sec)?;
    buffer.read_exact(&mut data.last_modified)?;
    buffer.read_exact(&mut data.date)?;
    buffer.read_exact(&mut data.crc32)?;
    buffer.read_exact(&mut data.valid_msec)?;
    buffer.read_exact(&mut data.header_start)?;
    buffer.read_exact(&mut data.body_start)?;

    buffer.read_exact(&mut data.etag_len)?;
    data.etag_data.resize(data.etag_len.as_usize(), 0);
    buffer.read_exact(&mut data.etag_data)?;

    buffer.read_exact(&mut data.etag_len)?;
    data.vary_data.resize(data.vary_len.as_usize(), 0);
    buffer.read_exact(&mut data.vary_data)?;

    // Read HTTP header
    data.header_data.resize(data.body_start.as_usize() - data.header_start.as_usize(), 0);
    buffer.seek(SeekFrom::Start(data.header_start.as_u64()))?;
    buffer.read_exact(&mut data.header_data)?;

    // Read HTTP data
    buffer.seek(SeekFrom::Start(data.body_start.as_u64()))?;
    buffer.read_to_end(&mut data.body_data)?;

    // Get the cache key, this isn't the best way to do it, but it works for now.
    buffer.seek(SeekFrom::Start(0))?;
    let mut raw_header:Vec<u8> = Vec::new();
    let mut key_bytes:Vec<u8> = Vec::new();
    raw_header.resize(data.header_start.as_usize(), 0);
    buffer.read_exact(&mut raw_header)?;
    let mut iter:Rev<Iter<'_, _>> = raw_header.iter().rev();
    iter.next();
    loop {
      match iter.next() {
        Some(val) => {
          if *val == 0x0a {
            break;
          } else {
            key_bytes.insert(0, *val);
          }
        },
        None => { break; }
      }
    }
    match str::from_utf8(&key_bytes) {
      Ok(val) => {
        data.key = val.replace("KEY: ", "").to_string();
      },
      _ => {}
    }

    return Ok(data);
  }
}
