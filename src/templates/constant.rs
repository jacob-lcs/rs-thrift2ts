use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::{ConstValue, Constant};

fn const_value_to_ts(value: &ConstValue) -> String {
  match value {
    ConstValue::Bool(b) => b.to_string(),
    ConstValue::Int(i) => i.to_string(),
    ConstValue::Double(d) => d.0.to_string(),
    ConstValue::String(s) => format!("\"{}\"", s.0.replace('\"', "\\\"")),
    ConstValue::Path(p) => p
      .segments
      .iter()
      .map(|s| s.0.clone())
      .collect::<Vec<_>>()
      .join("."),
    ConstValue::List(l) => format!(
      "[{}]",
      l.iter()
        .map(const_value_to_ts)
        .collect::<Vec<_>>()
        .join(", ")
    ),
    ConstValue::Map(m) => format!(
      "{{{}}}",
      m.iter()
        .map(|(k, v)| format!("{}: {}", const_value_to_ts(k), const_value_to_ts(v)))
        .collect::<Vec<_>>()
        .join(", ")
    ),
  }
}

pub fn gen(content: &Constant) -> String {
  format!(
    "export const {}: {} = {};\n",
    content.name.0,
    type_to_ts(&content.r#type),
    const_value_to_ts(&content.value)
  )
}
