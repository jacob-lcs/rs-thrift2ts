use crate::templates;
use crate::ThriftCodeGenOptionsRequired;
use pilota_thrift_parser as thrift_parser;
use pilota_thrift_parser::parser::Parser as _;
use pilota_thrift_parser::File;

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
        ts_code.push_str(&templates::include::gen(&include, &options.ext_name));
      }
      thrift_parser::Item::CppInclude(cpp_include) => {
        ts_code.push_str(&templates::cpp_include::gen(&cpp_include));
      }
      thrift_parser::Item::Namespace(namespace) => {
        ts_code.push_str(&templates::namespace::gen(&namespace));
      }
      thrift_parser::Item::Typedef(typedef) => {
        ts_code.push_str(&templates::type_def::gen(&typedef));
      }
      thrift_parser::Item::Constant(constant) => {
        ts_code.push_str(&templates::constant::gen(&constant));
      }
      thrift_parser::Item::Enum(enum_) => {
        ts_code.push_str(&templates::r#enum::gen(&enum_));
      }
      thrift_parser::Item::Struct(struct_) => {
        ts_code.push_str(&templates::r#struct::gen(&struct_));
      }
      thrift_parser::Item::Union(union) => {
        ts_code.push_str(&templates::union::gen(&union));
      }
      thrift_parser::Item::Exception(exception) => {
        ts_code.push_str(&templates::exception::gen(&exception));
      }
      thrift_parser::Item::Service(service) => {
        ts_code.push_str(&templates::service::gen(&service));
      }
    }
  }

  ts_code
}
