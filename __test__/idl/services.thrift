namespace cpp test.services
namespace java test.services
namespace py test.services

include "structs.thrift"
include "exceptions.thrift"

struct SearchCriteria {
  1: optional string keyword,
  2: optional i32 limit,
  3: optional i32 offset,
  4: optional map<string, string> filters
}

struct SearchResult {
  1: required list<structs.Person> items,
  2: required i32 total,
  3: required i32 page,
  4: required i32 pageSize
}

service SearchService {
  SearchResult search(1: SearchCriteria criteria)
    throws (1: exceptions.ValidationException validError),
  
  oneway void updateSearchIndex(1: string entity, 2: string id)
}

service UserManagementService {
  structs.Person createUser(1: structs.Person person)
    throws (
      1: exceptions.ValidationException validError,
      2: exceptions.DatabaseException dbError
    ),
  
  structs.Person updateUser(1: string id, 2: structs.Person person)
    throws (
      1: exceptions.ValidationException validError,
      2: exceptions.DatabaseException dbError
    ),
  
  void deleteUser(1: string id)
    throws (1: exceptions.DatabaseException dbError),
  
  structs.Person getUser(1: string id)
    throws (1: exceptions.DatabaseException dbError),
  
  list<structs.Person> listUsers(1: i32 limit, 2: i32 offset)
    throws (1: exceptions.DatabaseException dbError)
}