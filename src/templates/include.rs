use pilota_thrift_parser::Include;
use std::path::{Path, PathBuf};

fn ensure_dot_slash(path: &Path) -> PathBuf {
  let path_without_extension = path.file_stem().unwrap().to_string_lossy().to_string();
  if path.is_relative() && !path.starts_with("./") {
    // 如果路径是相对路径且不以 "./" 开头，则添加 "./"
    PathBuf::from("./").join(path_without_extension)
  } else {
    // 否则直接返回原路径
    path.to_path_buf()
  }
}

pub fn gen(content: &Include) -> String {
  let path = Path::new(content.path.0.as_str());
  let new_path = ensure_dot_slash(path);

  let file_name = path.file_stem().unwrap();

  format!(
    "import * as {} from '{}'; \n",
    file_name.to_string_lossy(),
    match new_path.to_str() {
      Some(p) => p,
      None => panic!("Invalid path: {:?}", new_path),
    }
  )
}
