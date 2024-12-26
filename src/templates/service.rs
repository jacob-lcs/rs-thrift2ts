use pilota_thrift_parser::Service;

pub fn gen(content: &Service) -> String {
  let mut ts_code = String::new();
  // 处理 Service 项
  ts_code.push_str(&format!("interface {:?} {{\n", &content.name));
  for function in &content.functions {
    ts_code.push_str(&format!(
      "  {:?}({:?}): {:?};\n",
      function.name,
      function
        .arguments
        .iter()
        .map(|p| format!("{:?}: {:?}", p.name, p.ty))
        .collect::<Vec<_>>()
        .join(", "),
      function.result_type
    ));
  }
  ts_code.push_str("}\n");
  return ts_code;
}
