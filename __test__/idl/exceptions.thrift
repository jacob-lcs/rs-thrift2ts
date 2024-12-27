namespace cpp test.exceptions
namespace java test.exceptions
namespace py test.exceptions

exception ValidationException {
    1: string message
    2: list<string> fields
    3: map<string, string> details
}

exception AuthenticationException {
    1: string message
    2: string username
    3: i32 errorCode
}

exception DatabaseException {
    1: string message
    2: i32 errorCode
    3: string query
    4: optional map<string, string> details
}

exception BusinessException {
    1: string message
    2: i32 errorCode
    3: string domain
    4: optional map<string, string> context
}

struct User {
    1: required string username
    2: required string password
    3: required string email
}

service SecureService {
    User authenticate(1: string username, 2: string password)
        throws (
            1: AuthenticationException authError,
            2: ValidationException validError
        )
    
    User createUser(1: User user)
        throws (
            1: ValidationException validError,
            2: DatabaseException dbError
        )
    
    void deleteUser(1: string username)
        throws (
            1: AuthenticationException authError,
            2: DatabaseException dbError,
            3: BusinessException bizError
        )
}