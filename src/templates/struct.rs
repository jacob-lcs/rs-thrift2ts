use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::{Attribute, Struct};
use std::fmt::Write;

pub fn gen(content: &Struct) -> String {
  let fields_count = content.fields.len();
  let estimated_size = content.name.0.len() + fields_count * 20;

  let mut ts_code = String::with_capacity(estimated_size);

  writeln!(&mut ts_code, "export interface {} {{", content.name.0).unwrap();

  for field in &content.fields {
    match field.attribute {
      Attribute::Optional => {
        write!(&mut ts_code, "  {}?: ", field.name.0).unwrap();
      }
      Attribute::Required | Attribute::Default => {
        write!(&mut ts_code, "  {}: ", field.name.0).unwrap();
      }
    }
    ts_code.push_str(&type_to_ts(&field.ty));
    ts_code.push_str(";\n");
  }
  ts_code.push_str("}\n");
  ts_code
}
