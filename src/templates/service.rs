use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::Service;
use std::fmt::Write;

pub fn gen(content: &Service) -> String {
  let capacity = content.functions.len() * 64 + 32;
  let mut ts_code = String::with_capacity(capacity);

  writeln!(&mut ts_code, "export interface {} {{", &content.name.0).unwrap();
  for function in &content.functions {
    writeln!(
      &mut ts_code,
      "  {}({}): {};",
      function.name.0,
      function
        .arguments
        .iter()
        .map(|p| format!("{}: {}", p.name.0, type_to_ts(&p.ty)))
        .collect::<Vec<_>>()
        .join(", "),
      format!("Promise<{}>", type_to_ts(&function.result_type)),
    )
    .unwrap();
  }
  ts_code.push_str("}\n");
  ts_code
}
