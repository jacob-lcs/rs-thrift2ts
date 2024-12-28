use pilota_thrift_parser::Union;

use crate::utils::r#type::type_to_ts;

pub fn gen(content: &Union) -> String {
  let mut ts_code = String::new();
  // 处理 Union 项
  ts_code.push_str(&format!("export type {} =\n", content.name.0));
  for field in &content.fields {
    ts_code.push_str(&format!(
      "  | {{ {}: {} }}\n",
      field.name.0,
      type_to_ts(&field.ty)
    ));
  }
  ts_code.push_str(";\n");
  ts_code
}
