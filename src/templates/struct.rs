use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::Struct;

pub fn gen(content: &Struct) -> String {
  let mut ts_code = String::new();
  ts_code.push_str(&format!("export interface {} {{\n", content.name.0));
  for field in &content.fields {
    ts_code.push_str(&format!("  {}: ", field.name.0));
    ts_code.push_str(&type_to_ts(&field.ty));
    ts_code.push_str(";\n");
  }
  ts_code.push_str("}\n");
  ts_code
}
