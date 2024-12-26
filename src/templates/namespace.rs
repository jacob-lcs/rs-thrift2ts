use pilota_thrift_parser::Namespace;
use log::info;

pub fn gen(content: &Namespace) -> String {
  info!("gen Namespace: {:?}", content);

  return String::from("\n");
}
