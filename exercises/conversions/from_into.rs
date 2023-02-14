// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a hint.

// From trait 用于 value 到 value 的转换。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    // 回退机制，默认值。
    // 当 提供的 字符串不能转换到 Person 对象的时候。
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return the default of Person
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person object with the results

// name 为 空 和 字符串 length 为 0 返回 默认值
// 切割出来 数组长度不等于 2 的也直接 返回默认值
// age usize 类型
// 逗号 分隔
// 也就是说  我们只需要 字符长度不为 0，长度为 0 应该也可以切割。是不是就可以合并了？


// 现在已经完成了这道题目
// 我在想 优化下逻辑。
// 嵌套 match 还是很丑陋
// if 的写法好像更清晰？
// NOTE: if 和 match 的使用也是需要根据实际场景来选择，
// NOTE: 虽然结果一样，但是 不同的场景下，使用正确的方式，代码可读性会更高。

// NOTE: 方法一
// impl From<&str> for Person {
//     fn from(s: &str) -> Person {
//         let s_length = s.len();
//         let splitted_s: Vec<&str> = s.split(",").collect();
//         match splitted_s.len() {
//             2 => {
//                 let maybe_name = splitted_s[0];
//                 let maybe_age = splitted_s[1];
//                 println!("name: {maybe_name}, age: {maybe_age}");
//                 match maybe_name {
//                     "" => return Person::default(),
//                     _ => {
//                         return match maybe_age.parse::<usize>() {
//                             Ok(age) => Person {
//                                 name: maybe_name.to_string(),
//                                 age,
//                             },
//                             Err(_) => Person::default(),
//                         };
//                     }
//                 }
//             }
//             _ => {
//                 return Person::default();
//             }
//         };
//     }
// }

// 方法 二
// NOTE: 学习到了一个 is_empty 的用法。 is_empty 配合 if 在当前这个场景还是比较好用的。
// impl From<&str> for Person {
//     fn from(s: &str) -> Person {
//         if s.is_empty() {
//             return Person::default();
//         }

//         let parts = s.split(',').collect::<Vec<&str>>();
//         if parts.len() != 2 {
//             return Person::default();
//         }

//         let name = parts[0].to_string();
//         if name.is_empty() {
//             return Person::default();
//         }

//         let age = match parts[1].parse::<usize>() {
//             Ok(age) => age,
//             Err(_) => return Person::default(),
//         };

//         Person { name, age }
//     }
// }

// NOTE: 方法三
// 这段代码可以的，我觉得可读性是比较高的
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 2
            || s.is_empty()
            || parts[0].is_empty()
            || parts[1].parse::<usize>().ok().is_none()
        {
            return Person::default();
        };

        let name = parts[0].to_string();
        let age = parts[1].parse::<usize>().unwrap();
        // 上面已经判断过了，如果是 parse 肯定是成功的

        Person { name, age }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
