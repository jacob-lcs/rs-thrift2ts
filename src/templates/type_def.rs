use crate::utils::r#type::type_to_ts;
use pilota_thrift_parser::Typedef;

pub fn gen(content: &Typedef) -> String {
  format!(
    "type {} = {};",
    content.alias.as_str(),
    type_to_ts(&content.r#type)
  )
}
