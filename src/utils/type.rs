use pilota_thrift_parser::descriptor::{Ty, Type};

pub fn type_to_ts(ty: &Type) -> String {
  match &**ty {
    Ty::String => "string".to_string(),
    Ty::Void => "void".to_string(),
    Ty::Byte | Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Double => "number".to_string(),
    Ty::Bool => "boolean".to_string(),
    Ty::Binary => "Uint8Array".to_string(),
    Ty::Uuid => "string".to_string(),
    Ty::List { value, .. } => format!("Array<{}>", type_to_ts(value)),
    Ty::Set { value, .. } => format!("Set<{}>", type_to_ts(value)),
    Ty::Map { key, value, .. } => format!("Map<{}, {}>", type_to_ts(key), type_to_ts(value)),
    Ty::Path(path) => {
      let segments = path
        .segments
        .iter()
        .map(|s| s.0.clone())
        .collect::<Vec<_>>();
      segments.join(".").to_string()
    }
  }
}
