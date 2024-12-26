use pilota_thrift_parser::Exception;

pub fn gen(content: &Exception) -> String {
  let mut ts_code = String::new();
  // 处理 Exception 项
  ts_code.push_str(&format!("interface {:?} extends Error {{\n", content.name));
  for field in &content.fields {
    ts_code.push_str(&format!("  {:?}: {:?},\n", field.name, field.ty));
  }
  ts_code.push_str("}\n");
  ts_code
}
