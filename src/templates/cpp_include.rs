use log::info;
use pilota_thrift_parser::CppInclude;

pub fn gen(content: &CppInclude) -> String {
  info!("gen CppInclude: {:?}", content.0);

  return String::from("\n");
}
