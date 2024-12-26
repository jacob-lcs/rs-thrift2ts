use pilota_thrift_parser::Include;
use std::path::PathBuf;

pub fn gen(content: &Include, ext_name: &String) -> String {
  let mut path = PathBuf::from(content.path.to_string().as_str());
  path.set_extension(ext_name);

  let file_name = path.file_stem().unwrap();

  return format!(
    "import {} from '{}'; \n",
    file_name.to_string_lossy(),
    path.display()
  );
}
