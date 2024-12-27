use pilota_thrift_parser::Namespace;

pub fn gen(content: &Namespace) -> String {
  format!("// Namespace: {:?}, scope: {:?}\n", content.name.segments.iter(), content.scope)
}
