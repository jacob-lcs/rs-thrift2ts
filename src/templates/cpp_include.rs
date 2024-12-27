use pilota_thrift_parser::CppInclude;

pub fn gen(content: &CppInclude) -> String {
  format!("// CppInclude: {}\n", content.0 .0)
}
