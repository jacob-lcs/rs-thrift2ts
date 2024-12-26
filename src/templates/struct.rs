use pilota_thrift_parser::Struct;

pub fn gen(content: &Struct) -> String {
  let mut ts_code = String::new();
  ts_code.push_str(&format!("interface {} {{\n", content.name.to_string()));
  for field in &content.fields {
    // ts_code.push_str(&format!("  {}: {:?},\n", field.name.to_string(), field.ty));
    ts_code.push_str(&format!("  {}: ", field.name.to_string()));
    match &field.ty.0 {
      pilota_thrift_parser::Ty::String => ts_code.push_str("string"),
      pilota_thrift_parser::Ty::Void => ts_code.push_str("void"),
      pilota_thrift_parser::Ty::Byte => ts_code.push_str("unknown"),
      pilota_thrift_parser::Ty::Bool => ts_code.push_str("boolean"),
      pilota_thrift_parser::Ty::Binary => ts_code.push_str("unknown"),
      pilota_thrift_parser::Ty::I8 => ts_code.push_str("number"),
      pilota_thrift_parser::Ty::I16 => ts_code.push_str("number"),
      pilota_thrift_parser::Ty::I32 => ts_code.push_str("number"),
      pilota_thrift_parser::Ty::I64 => ts_code.push_str("string"),
      pilota_thrift_parser::Ty::Double => ts_code.push_str("number"),
      pilota_thrift_parser::Ty::Uuid => ts_code.push_str("string"),
      pilota_thrift_parser::Ty::List { value, cpp_type } => ts_code.push_str("any[]"),
      pilota_thrift_parser::Ty::Set { value, cpp_type } => ts_code.push_str("Map"),
      pilota_thrift_parser::Ty::Map {
        key,
        value,
        cpp_type,
      } => ts_code.push_str("Map"),
      pilota_thrift_parser::Ty::Path(path) => ts_code.push_str("unknown"),
    }
    ts_code.push_str(";\n");
  }
  ts_code.push_str("}\n");
  return ts_code;
}
