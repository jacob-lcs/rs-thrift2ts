use pilota_thrift_parser::Constant;

pub fn gen(content: &Constant) -> String {
  let mut ts_code = String::new();
  ts_code.push_str(&format!(
    "const {:?}: {:?} = {:?};\n",
    content.name, content.r#type, content.value
  ));
  return ts_code;
}
