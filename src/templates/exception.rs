use pilota_thrift_parser::Exception;

use crate::utils::r#type::type_to_ts;

pub fn gen(content: &Exception) -> String {
  let mut ts_code = String::new();
  // 处理 Exception 项
  ts_code.push_str(&format!("export interface {} extends Error {{\n", content.name.0));
  for field in &content.fields {
    ts_code.push_str(&format!("  {}: {},\n", field.name.0, type_to_ts(&field.ty)));
  }
  ts_code.push_str("}\n");
  ts_code
}
