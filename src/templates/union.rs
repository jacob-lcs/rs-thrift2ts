use pilota_thrift_parser::Union;

pub fn gen(content: &Union) -> String {
  let mut ts_code = String::new();
  // 处理 Union 项
  ts_code.push_str(&format!("export type {:?} =\n", content.name));
  for field in &content.fields {
    ts_code.push_str(&format!("  | {{ {:?}: {:?} }}\n", field.name, field.ty));
  }
  ts_code.push_str(";\n");
  ts_code
}
