namespace cpp test.basic
namespace java test.basic
namespace py test.basic

const i16 INT16_CONSTANT = 1234
const i32 INT32_CONSTANT = 123456789
const i64 INT64_CONSTANT = 9223372036854775807
const double DOUBLE_CONSTANT = 3.14159
const string STRING_CONSTANT = "Hello, World!"
const bool BOOL_CONSTANT = true

struct BasicTypes {
  1: bool boolField
  2: byte byteField
  3: i16 i16Field
  4: i32 i32Field
  5: i64 i64Field
  6: double doubleField
  7: string stringField
  8: binary binaryField
}

service BasicTypeService {
  BasicTypes testAllTypes(
    1: bool boolParam,
    2: byte byteParam,
    3: i16 i16Param,
    4: i32 i32Param,
    5: i64 i64Param,
    6: double doubleParam,
    7: string stringParam,
    8: binary binaryParam
  )

  void testVoid()
  bool testBool(1: bool param)
  byte testByte(1: byte param)
  i16 testI16(1: i16 param)
  i32 testI32(1: i32 param)
  i64 testI64(1: i64 param)
  double testDouble(1: double param)
  string testString(1: string param)
  binary testBinary(1: binary param)
}