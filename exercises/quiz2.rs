/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-02 22:09:32
 * @FilePath: /00rust_test/Users/onepisya/Library/Mobile Documents/iCloud~md~obsidian/Documents/obsidianMyNotes/00AllNotesVault/编程语言/Rust/rust权威指南/projects/rustlings/exercises/quiz2.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    // TODO: Complete the function signature!
    // 需要注意的是 签名的类型选择很重要，这里 如果 返回 Vec<&str>
    // 那么就要求 所有 放入数组内的 数据 生命周期都需要在函数结束后还存在
    // 否则 离开后，数据将会被销毁。
    // 当然如果你能做到 传进来给你的是 &str 类型，你还能 在上一面进行 数据追加的话
    // 也是可以的（但是我个人觉得这种情况不太现实）
    // into 进来的时候 可以 转成 input: Vec<(String, Command)>
    // 也是可以实现的，不过在内部处理的时候，还是会 &String 进行转换的。
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            //  put results in  output
            let o = match command {
                Command::Append(amount) => {
                    let append = move || {
                        let x = "bar".repeat(*amount);
                        // repeat 返回的是 String 类型
                        // String 可以 +=  但是 str 和 &str 不能
                        // &str 在 表达式前面是不可能合并字符的
                        // 除非是通过 该字符串 生成新的 String
                        let mut s = String::from(*string);
                        s += x.as_str();
                        s
                    };
                    // 在这里面就直接把他 推进去了，将所有权交给了 push 函数
                    // 也可以在外面去 push 也可以。
                    // output.push(append());
                    append()
                }
                Command::Trim => {
                    // output.push(string.trim().to_string());
                    string.trim().to_string()
                }
                Command::Uppercase => {
                    // output.push(string.to_uppercase());
                    string.to_uppercase()
                }
            };
            // output.
            output.push(o)
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // Question: What do we need to import to have `transformer` in scope?
    // My_Answer: yes, it's required
    use super::Command;
    use my_module::transformer;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);

        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
