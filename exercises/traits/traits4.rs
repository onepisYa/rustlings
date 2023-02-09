/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 20:19:00
 * @FilePath: /rustlings/exercises/traits/traits4.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
// 真这样写？ 感觉有点蠢。
// 都有的共同点是 需要实现了 Licensed，什么类型不管。
// 也就是说 我们 只需要一个泛型，不知道能不能实现。
// 如何将 compare_license_types 泛型减少到一个
// 可能需要使用 我想到的一个办法是 使用 Wrapper 进行包装。或者 放入到 Enum 中
// 这样 编译器会认为他们是一个类型。这样子，我就不需要 写两个泛型了。

fn compare_license_types<T: Licensed, F: Licensed>(software: T, software_two: F) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
