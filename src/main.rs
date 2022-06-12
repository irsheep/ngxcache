use std::env;
use std::path::PathBuf;
use core::slice::Iter;

use clap::Parser;

mod nginx_cache;

const ENV_CACHE_PATH:&str = "NGXCACHE_PATH";
const ENV_ORDER_BY:&str = "NGXCACHE_ORDER_BY";
const ENV_ORDER:&str = "NGXCACHE_ORDER";

/// Reads and displays medatada information about Nginx cache files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to Nginx cache directory
    #[clap(short, long, default_value="")]
    path: String,

    /// Ommites file path
    #[clap(short, long, takes_value=false)]
    no_path:bool,

    #[clap(short, long, default_value="none", possible_values=["none", "filename", "cached", "expire", "modified", "key"])]
    order_by:String,

    /// Sort in ascending order
    #[clap(short, long, takes_value=false)]
    ascending:bool,
    /// Sort in descending order
    #[clap(short, long, takes_value=false)]
    descending:bool,

    /// Path to Nginx cache directory
    #[clap(last=false, default_value="")]
    last: String
}

fn main() {
  let path:PathBuf;
  let args:Args;
  let order_by:String;
  let order:i32;
  let filter:nginx_cache::NgxFilter;

  args = Args::parse();
  
  if args.path != "" {
    path = PathBuf::from(args.path);
  } else if args.last != "" {
    path = PathBuf::from(args.last);
  } else {
    match env::var(ENV_CACHE_PATH) {
      Ok(value) => { path = PathBuf::from(value); },
      _error => { 
        println!("No path was specified or enviroment variable {} was not set.", ENV_CACHE_PATH);
        std::process::exit(0x00);
        
      }
    }
  }

  if args.order_by != "none" { 
    order_by = args.order_by;
  } else {
    match env::var(ENV_ORDER_BY) {
      Ok(value) => { order_by = value; },
      _error => { order_by = String::from("none"); }
    }
  }

  if args.ascending {
    order = 1;
  } else if args.descending {
    order = -1;
  } else {
    match env::var(ENV_ORDER) {
      Ok(value) => { order = if value == "descending" { -1 } else { 1 } },
      _error => { order = 0; }
    }
  }

  if path.exists() {

    filter = nginx_cache::NgxFilter {
      order_by: order_by,
      order: order
    };
    
    let file_data = nginx_cache::get_cached_files_info(path, filter);
    let mut iter:Iter<nginx_cache::NgxCacheData> = file_data.iter();
    loop {
      match iter.next() {
        Some(data) => { print_line(data, args.no_path); },
        None => { break; }
      }
    }
  } else {
    println!("Directory not found: {:?}", path);
  }
}

fn print_line(data:&nginx_cache::NgxCacheData, no_path:bool) {
  println!("{} {:?} {:?} {:?} {} {}",
    if no_path { data.file.file_name().unwrap().to_str().unwrap() } else { data.file.to_str().unwrap() },
    data.cached_date,
    data.expire_date,
    data.last_modified_date,
    data.crc32,
    data.key
  );
}