namespace cpp test.enums
namespace java test.enums
namespace py test.enums

enum DayOfWeek {
  MONDAY = 1,
  TUESDAY = 2,
  WEDNESDAY = 3,
  THURSDAY = 4,
  FRIDAY = 5,
  SATURDAY = 6,
  SUNDAY = 7
}

enum Month {
  JANUARY = 1,
  FEBRUARY = 2,
  MARCH = 3,
  APRIL = 4,
  MAY = 5,
  JUNE = 6,
  JULY = 7,
  AUGUST = 8,
  SEPTEMBER = 9,
  OCTOBER = 10,
  NOVEMBER = 11,
  DECEMBER = 12
}

enum Status {
  ACTIVE = 1,
  INACTIVE = 2,
  PENDING = 3,
  DELETED = 4
}

struct Calendar {
  1: required DayOfWeek dayOfWeek,
  2: required Month month,
  3: required i32 year,
  4: required i32 day
}

struct Task {
  1: required string title,
  2: required Status status,
  3: optional string description,
  4: optional Calendar dueDate
}

service CalendarService {
  string getDayName(1: DayOfWeek day),
  string getMonthName(1: Month month),
  list<Task> getTasksByStatus(1: Status status),
  Calendar createCalendar(1: DayOfWeek day, 2: Month month, 3: i32 year, 4: i32 dayOfMonth)
}