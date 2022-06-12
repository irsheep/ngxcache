use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use walkdir::WalkDir;

mod structures;
use structures::*;
mod traits;
use traits::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NgxCacheData {
  pub file:PathBuf,
  pub cached_date:NaiveDateTime,
  pub expire_date:NaiveDateTime,
  pub last_modified_date:NaiveDateTime,
  pub crc32:u64,
  pub key:String
}

pub struct NgxFilter {
  pub order_by:String,
  pub order:i32
}

pub fn get_cached_files_info(path:PathBuf, filter:NgxFilter) -> Vec<NgxCacheData> {
  let mut data:Vec<NgxCacheData> = Vec::new();
  for entry in WalkDir::new(path).into_iter() {
    if entry.as_ref().unwrap().file_type().is_file() {
      match parse_file(entry.unwrap().into_path()) {
        Some(v) => data.push(v),
        None => {}
      }
    }
  }

  if filter.order_by != "none" && filter.order != 0 {
    let ascending:bool = if filter.order == 1 { true } else { false };
    match filter.order_by.as_str() {
      "filename" => { data.sort_by(|a, b| if ascending { a.file.cmp(&b.file) } else { b.file.cmp(&a.file) } )},
      "cached" => { data.sort_by(|a, b| if ascending { a.cached_date.cmp(&b.cached_date) } else { b.cached_date.cmp(&a.cached_date) } )},
      "expired" => { data.sort_by(|a, b| if ascending { a.expire_date.cmp(&b.expire_date) } else { b.expire_date.cmp(&a.expire_date) } )},
      "modified" => { data.sort_by(|a, b| if ascending { a.last_modified_date.cmp(&b.last_modified_date) } else { b.last_modified_date.cmp(&a.last_modified_date) } )},
      "key" => { data.sort_by(|a, b| if ascending { a.key.cmp(&b.key) } else { b.key.cmp(&a.key) } )},
      _ => {}
    }
  }

  return data;
}

fn parse_file(file_name:PathBuf) -> Option<NgxCacheData> {
  let mut buf_reader:BufReader<File>;
  let mut nginx_header:NgxHttpFileCacheHeader = NgxHttpFileCacheHeader::new();

  match File::open(&file_name) {
    Ok(file_handle) => {
      buf_reader = BufReader::new(file_handle);
      match nginx_header.from_buffer(&mut buf_reader) {
        Ok(_) => {
          return Some(NgxCacheData{
            file: file_name,
            cached_date: NaiveDateTime::from_timestamp(nginx_header.date.as_i64(), 0),
            expire_date: NaiveDateTime::from_timestamp(nginx_header.valid_sec.as_i64(), 0),
            last_modified_date: NaiveDateTime::from_timestamp(nginx_header.last_modified.as_i64(), 0),
            crc32: nginx_header.crc32.as_u64(),
            key: nginx_header.key
          });
        },
        Err(err) => { println!("{} {}", err, file_name.to_str().unwrap()); return None; }
      }
    },
    Err(err) => { println!("{:?}", err); return None; }
  }
}
