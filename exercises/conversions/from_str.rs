// from_str.rs
// This is similar to from_into.rs, but this time we'll implement `FromStr`
// and return errors instead of falling back to a default value.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
    //
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if you want to return a
// string error message, you can do so via just using return `Err("my error message".into())`.


// --------------- 为本练习实现的一些工具 start ---------------

macro_rules! try {
    ($expr:expr) => {
        match $expr {
            Ok(val) =>  val,
            Err(e) => return Err(Self::Err::ParseInt(e)),
        }
    };
}

impl From<ParseIntError> for ParsePersonError{
    fn from(s: ParseIntError) -> ParsePersonError{
        ParsePersonError::ParseInt(s)
    }
}


// --------------- 为本练习实现的一些工具 end ---------------

// 名字和年龄 任何一个无效或者缺失 都 return Err(Self::Err::ParseInt(ParseIntError));
// 空字符串则 return Err(Self::Err::Empty);
// length!=2 尾随逗号 或者 逗号缺失，或者 年龄缺失 return Err(Self::Err::BadLen);

impl FromStr for Person {
    type Err = ParsePersonError;
    // fn from_str(s: &str) -> Result<Person, Self::Err> {
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if s.is_empty() {
            return Err(Self::Err::Empty);
        } else if parts.len() != 2 {
            return Err(Self::Err::BadLen);
        } else if parts[0].is_empty() {
            return Err(Self::Err::NoName);
        } else {
            let name = parts[0].to_string();
            // NOTE: 对于 age 的处理方式一
            // match parts[1].parse::<usize>() {
            //     Ok(age) => Ok(Person { name, age }),
            //     Err(e) => return Err(Self::Err::ParseInt(e)),
            // }

            // NOTE: 对于 age 的处理方式二
            // 定义一个宏来处理， 不过没有太大的意义，搞着玩
            // let age = try!(parts[1].parse::<usize>());
            // Ok(Person { name, age })

            // NOTE: 对于 age 的处理三
            // DONE: 自己布置的小作业，为 该类型实现 ? 操作符，
            // DONE: 自动的 返回 对应的类型
            // DONE: 如果出错则 返回 错误。
            // 很好的学习和巩固了 From trait
            let age = parts[1].parse::<usize>()?;
            Ok(Person { name, age })
        }

    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
