/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-07 22:31:55
 * @FilePath: /rustlings/exercises/options/options1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    // ???
    // 小于等于 10PM   或者说是 22 点 那么 剩余 5 片 icecream

    if time_of_day > 24 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
