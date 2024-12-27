use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::{ConstValue, Constant};

fn const_value_to_ts(value: &ConstValue) -> String {
  match value {
    ConstValue::Bool(b) => b.to_string(),
    ConstValue::Path(p) => format!("{:?}", p),
    ConstValue::String(s) => format!("{:?}", s.0),
    ConstValue::Int(i) => i.to_string(),
    ConstValue::Double(d) => format!("{}", d.0),
    ConstValue::List(l) => {
      let elements: Vec<String> = l.iter().map(const_value_to_ts).collect();
      format!("[{}]", elements.join(", "))
    }
    ConstValue::Map(m) => {
      let entries: Vec<String> = m
        .iter()
        .map(|(k, v)| format!("{}: {}", const_value_to_ts(k), const_value_to_ts(v)))
        .collect();
      format!("{{{}}}", entries.join(", "))
    }
  }
}

pub fn gen(content: &Constant) -> String {
  let mut ts_code = String::new();
  ts_code.push_str(&format!(
    "export const {}: {} = {};\n",
    content.name.0,
    type_to_ts(&content.r#type),
    const_value_to_ts(&content.value)
  ));
  ts_code
}
