use pilota_thrift_parser::CppInclude;
use log::info;

pub fn gen(content: &CppInclude) -> String {
  info!("gen CppInclude: {:?}", content.0);

  return String::from("\n");
}
