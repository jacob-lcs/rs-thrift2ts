include "types.thrift"

struct User {
  1: i32 id,
  2: string name,
  3: optional string email,
  4: optional i32 age
}
