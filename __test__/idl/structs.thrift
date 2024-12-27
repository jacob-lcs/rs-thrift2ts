namespace cpp test.structs
namespace java test.structs
namespace py test.structs

struct Address {
  1: required string street,
  2: required string city,
  3: required string country,
  4: optional string postalCode,
  5: optional string state
}

struct Contact {
  1: required string email,
  2: optional string phone,
  3: optional string mobile,
  4: optional string fax
}

struct Person {
  1: required string firstName,
  2: required string lastName,
  3: required i32 age,
  4: optional string middleName,
  5: required Address address,
  6: optional Contact contact,
  7: optional list<string> nicknames,
  8: optional map<string, string> properties
}

struct Company {
  1: required string name,
  2: required Address headquarters,
  3: required list<Person> employees,
  4: optional list<Address> branches,
  5: optional map<string, Contact> departmentContacts
}

struct Department {
  1: required string name,
  2: required Person manager,
  3: required list<Person> staff,
  4: optional double budget,
  5: optional map<string, double> expenditure
}

service StructService {
  Person createPerson(1: string firstName, 2: string lastName, 3: i32 age),
  Company createCompany(1: string name, 2: Address headquarters),
  Department createDepartment(1: string name, 2: Person manager),
  
  list<Person> searchPersons(1: map<string, string> criteria),
  list<Company> searchCompanies(1: map<string, string> criteria)
}