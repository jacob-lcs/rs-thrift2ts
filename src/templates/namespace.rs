use log::info;
use pilota_thrift_parser::Namespace;

pub fn gen(content: &Namespace) -> String {
  info!("gen Namespace: {:?}", content);

  String::from("")
}
