use pilota_thrift_parser::{Ty, Type, Typedef};

pub fn gen(content: &Typedef) -> String {
  let ts_type = match &content.r#type.0 {
    Ty::String => "string".to_string(),
    Ty::Void => "void".to_string(),
    Ty::Byte | Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Double => "number".to_string(),
    Ty::Bool => "boolean".to_string(),
    Ty::Binary => "Uint8Array".to_string(),
    Ty::Uuid => "string".to_string(),
    Ty::List { value, .. } => format!("Array<{}>", gen_type(value)),
    Ty::Set { value, .. } => format!("Set<{}>", gen_type(value)),
    Ty::Map { key, value, .. } => format!("Map<{}, {}>", gen_type(key), gen_type(value)),
    Ty::Path(_) => "any".to_string(),
  };

  format!("type {} = {};", content.alias.as_str(), ts_type)
}

fn gen_type(ty: &Type) -> String {
  match &ty.0 {
    Ty::String => "string".to_string(),
    Ty::Void => "void".to_string(),
    Ty::Byte | Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Double => "number".to_string(),
    Ty::Bool => "boolean".to_string(),
    Ty::Binary => "Uint8Array".to_string(),
    Ty::Uuid => "string".to_string(),
    Ty::List { value, .. } => format!("Array<{}>", gen_type(value)),
    Ty::Set { value, .. } => format!("Set<{}>", gen_type(value)),
    Ty::Map { key, value, .. } => format!("Map<{}, {}>", gen_type(key), gen_type(value)),
    Ty::Path(_) => "any".to_string(),
  }
}
