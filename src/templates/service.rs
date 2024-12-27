use pilota_thrift_parser::Service;

use crate::utils::r#type::type_to_ts;

pub fn gen(content: &Service) -> String {
  let mut ts_code = String::new();
  // 处理 Service 项
  ts_code.push_str(&format!("export interface {} {{\n", &content.name.0));
  for function in &content.functions {
    ts_code.push_str(&format!(
      "  {}({}): {};\n",
      function.name.0,
      function
        .arguments
        .iter()
        .map(|p| format!("{}: {}", p.name.0, type_to_ts(&p.ty)))
        .collect::<Vec<_>>()
        .join(", "),
      type_to_ts(&function.result_type)
    ));
  }
  ts_code.push_str("}\n");
  ts_code
}
