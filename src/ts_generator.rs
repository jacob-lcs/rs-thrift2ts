use pilota_thrift_parser as thrift_parser;
use pilota_thrift_parser::parser::Parser as _;
use pilota_thrift_parser::File;
use crate::ThriftCodeGenOptionsRequired;
use crate::templates;

pub fn convert_thrift_to_ts(content: &str, options: &ThriftCodeGenOptionsRequired) -> String {
  // 将 Thrift 内容解析为 AST
  let ast = thrift_parser::File::parse(&content).unwrap().1;

  // 将 AST 转换为 TypeScript
  generate_ts_from_ast(&ast, options)
}

fn generate_ts_from_ast(ast: &File, options: &ThriftCodeGenOptionsRequired) -> String {
  let mut ts_code = String::new();

  for item in &ast.items {
    match item {
      thrift_parser::Item::Include(include) => {
        // 处理 Include 项
        ts_code.push_str(&templates::include::gen(&include, &options.ext_name));
      }
      thrift_parser::Item::CppInclude(cpp_include) => {
        // 处理 CppInclude 项
        ts_code.push_str(&format!("// CppInclude: {:?}\n", cpp_include.0));
      }
      thrift_parser::Item::Namespace(namespace) => {
        // 处理 Namespace 项
        ts_code.push_str(&format!("// Namespace: {:?}\n", namespace.name));
      }
      thrift_parser::Item::Typedef(typedef) => {
        // 处理 Typedef 项
        // ts_code.push_str(&format!("type {:?} = {:?};\n", typedef.name, typedef.ty));
      }
      thrift_parser::Item::Constant(constant) => {
        // 处理 Constant 项
        ts_code.push_str(&format!(
          "const {:?}: {:?} = {:?};\n",
          constant.name, constant.r#type, constant.value
        ));
      }
      thrift_parser::Item::Enum(enum_) => {
        // 处理 Enum 项
        // ts_code.push_str(&format!("enum {:?} {{\n", enum_.name));
        // for value in &enum_.values {
        //   ts_code.push_str(&format!("  {:?} = {:?},\n", value.name, value.value.as_ref().unwrap_or(&0)));
        // }
        // ts_code.push_str("}\n");
      }
      thrift_parser::Item::Struct(struct_) => {
        // 处理 Struct 项
        ts_code.push_str(&format!("interface {} {{\n", struct_.name.to_string()));
        for field in &struct_.fields {
          // ts_code.push_str(&format!("  {}: {:?},\n", field.name.to_string(), field.ty));
          ts_code.push_str(&format!("  {}: ", field.name.to_string()));
          match &field.ty.0 {
            pilota_thrift_parser::Ty::String => ts_code.push_str("string"),
            pilota_thrift_parser::Ty::Void => ts_code.push_str("void"),
            pilota_thrift_parser::Ty::Byte => ts_code.push_str("unknown"),
            pilota_thrift_parser::Ty::Bool => ts_code.push_str("boolean"),
            pilota_thrift_parser::Ty::Binary => ts_code.push_str("unknown"),
            pilota_thrift_parser::Ty::I8 => ts_code.push_str("number"),
            pilota_thrift_parser::Ty::I16 => ts_code.push_str("number"),
            pilota_thrift_parser::Ty::I32 => ts_code.push_str("number"),
            pilota_thrift_parser::Ty::I64 => ts_code.push_str("string"),
            pilota_thrift_parser::Ty::Double => ts_code.push_str("number"),
            pilota_thrift_parser::Ty::Uuid => ts_code.push_str("string"),
            pilota_thrift_parser::Ty::List { value, cpp_type } => ts_code.push_str("any[]"),
            pilota_thrift_parser::Ty::Set { value, cpp_type } => ts_code.push_str("Map"),
            pilota_thrift_parser::Ty::Map {
              key,
              value,
              cpp_type,
            } => ts_code.push_str("Map"),
            pilota_thrift_parser::Ty::Path(path) => ts_code.push_str("unknown"),
          }
          ts_code.push_str(";\n");
        }
        ts_code.push_str("}\n");
      }
      thrift_parser::Item::Union(union) => {
        // 处理 Union 项
        ts_code.push_str(&format!("type {:?} =\n", union.name));
        for field in &union.fields {
          ts_code.push_str(&format!("  | {{ {:?}: {:?} }}\n", field.name, field.ty));
        }
        ts_code.push_str(";\n");
      }
      thrift_parser::Item::Exception(exception) => {
        // 处理 Exception 项
        ts_code.push_str(&format!(
          "interface {:?} extends Error {{\n",
          exception.name
        ));
        for field in &exception.fields {
          ts_code.push_str(&format!("  {:?}: {:?},\n", field.name, field.ty));
        }
        ts_code.push_str("}\n");
      }
      thrift_parser::Item::Service(service) => {
        // 处理 Service 项
        ts_code.push_str(&format!("interface {:?} {{\n", service.name));
        for function in &service.functions {
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
      }
    }
  }

  ts_code
}
