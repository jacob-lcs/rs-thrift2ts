#![deny(clippy::all)]

mod generator;

use napi_derive::napi;
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

#[napi(object)]
pub struct ThriftCodeGenOptions {
  ///  Generated file type name, the default value is .ts.
  pub ext_name: Option<String>,
  /// Whether to convert the data type of map in thrift to Object in js instead of Map when parsing data, the default value is false.
  pub map_as_object: Option<bool>,
  /// Whether to convert the data type of set in thrift to Array in js instead of Set when parsing data, the default value is false.
  pub set_as_array: Option<bool>,
  ///  Whether to convert the data type of i64 in thrift to string in js instead of bigint when parsing data, the default value is false.
  pub int64_as_string: Option<bool>,
  /// Whether to convert the data type of i64 in thrift to string in js instead of bigint when parsing data, the default value is false.
  pub int64_as_number: Option<bool>,
  /// The path of the thrift file to be parsed.
  pub path: String,
}

#[napi]
pub fn gen(options: ThriftCodeGenOptions) {
  let ext_name = options.ext_name.unwrap_or_else(|| ".ts".to_string());
  let map_as_object = options.map_as_object.unwrap_or(false);
  let set_as_array = options.set_as_array.unwrap_or(false);
  let int64_as_string = options.int64_as_string.unwrap_or(false);
  let int64_as_number = options.int64_as_number.unwrap_or(false);
  let path = options.path;

  let thrift_files: Vec<_> = WalkDir::new(path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().map_or(false, |ext| ext == "thrift"))
    .map(|e| e.path().to_owned())
    .collect();

  // 并行处理所有文件
  thrift_files
    .par_iter()
    .try_for_each(|path| process_thrift_file(path).map_err(|e| eprintln!("Error processing file: {}", e)));
}

fn process_thrift_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
  // 1. 读取 thrift 文件内容
  let content = std::fs::read_to_string(path)?;

  // 2. 转换为 TypeScript
  let ts_content = generator::ts_generator::convert_thrift_to_ts(&content);
  let ts_path = path.with_extension("ts");
  std::fs::write(ts_path, ts_content)?;

  Ok(())
}
