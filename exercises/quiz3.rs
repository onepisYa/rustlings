/*
 * Copyright (c) 2023 by onepisYa , All Rights Reserved.
 * @Date: 2023-02-09 21:17:13
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-09 23:07:31
 * @FilePath: /rustlings/exercises/quiz3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 * 
 */

// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// 另外在写出来之前，走了很多弯路，实际上，它就是要考察 trait bound + generic 泛型。
// 根本没有说要我们实现转换系统，仅仅是说， 它想要能够打印而已
// 是我自己意淫了一些根本不存在的需求，导致说我认为这个题目其实 设计的有问题。
// 本质上是我没有理解到 出题人的 出题意图。

// 现在虽然实现了，并且通过了，但是总感觉怪怪的。
// 实现二是更好的选择，是出题人想要的答案。
// --------------------------------------

// 实现一
// 这种写法也可以的，但是 我个人认为很不简洁。
// ------ my code start ---------
// pub trait DisplayGrade {
//     fn display(&self) -> String;
// }

// impl DisplayGrade for f32 {
//     fn display(&self) -> String {
//         format!("{}", self)
//     }
// }

// impl DisplayGrade for &str {
//     fn display(&self) -> String {
//         format!("{}", self)
//     }
// }

// pub struct ReportCard<T> {
//     pub grade: T,
//     pub student_name: String,
//     pub student_age: u8,
// }

// impl<T: DisplayGrade> ReportCard<T> {
//     pub fn print(&self) -> String {
//         format!(
//             "{} ({}) - achieved a grade of {}",
//             &self.student_name,
//             &self.student_age,
//             &self.grade.display()
//         )
//     }
// }

// ------ my code end ---------

// 实现二
// 很明显这种方式，要优雅的多。
// ------ my code start ---------
// 准备在这里再写一个版本。
use std::fmt;
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

// ------ my code end -----------

// ------ original implementation start ---------
// pub struct ReportCard {
//     pub grade: f32,
//     pub student_name: String,
//     pub student_age: u8,
// }

// impl ReportCard {
//     pub fn print(&self) -> String {
//         format!(
//             "{} ({}) - achieved a grade of {}",
//             &self.student_name, &self.student_age, &self.grade
//         )
//     }
// }
// ------ original implementation end ---------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            // grade: 2.1,
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
