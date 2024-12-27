namespace cpp test.containers
namespace java test.containers
namespace py test.containers

typedef list<string> StringList
typedef set<i32> IntSet
typedef map<string, double> StringDoubleMap

struct ComplexContainers {
  1: list<string> stringList
  2: list<i32> numberList
  3: set<string> stringSet
  4: set<i64> numberSet
  5: map<string, string> stringMap
  6: map<i32, double> numberMap
  7: list<list<string>> nestedList
  8: map<string, map<string, i32>> nestedMap
  9: set<list<string>> mixedContainer1
  10: map<string, set<i32>> mixedContainer2
}

const list<string> CONST_LIST = ["a", "b", "c"]
const set<i32> CONST_SET = [1, 2, 3]
const map<string, string> CONST_MAP = {"key1": "value1", "key2": "value2"}

service ContainerService {
  list<string> getStringList(1: i32 count)
  set<i32> getNumberSet(1: i32 max)
  map<string, double> getStringDoubleMap(1: list<string> keys)
  
  ComplexContainers testAllContainers(
    1: list<string> listParam,
    2: set<i32> setParam,
    3: map<string, double> mapParam
  )
}