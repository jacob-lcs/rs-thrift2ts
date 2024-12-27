use pilota_thrift_parser::Enum;

pub fn gen(content: &Enum) -> String {
  let mut ts_code = String::with_capacity(128);
  ts_code.push_str("export enum ");
  ts_code.push_str(content.name.as_str());
  ts_code.push_str(" {\n");

  for value in &content.values {
    ts_code.push_str("    ");
    ts_code.push_str(value.name.as_str());
    ts_code.push_str(" = ");
    match &value.value {
      Some(v) => ts_code.push_str(&v.to_string()),
      None => ts_code.push_str("undefined"),
    }
    ts_code.push_str(",\n");
  }

  ts_code.push_str("}\n");
  ts_code
}
