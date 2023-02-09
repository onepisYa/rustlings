/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-06 22:13:29
 * @FilePath: /rustlings/exercises/modules/modules1.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description: 
 */
// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
