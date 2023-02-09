/*
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved.
 * @Date: 2022-12-28 21:00:00
 * @LastEditors: onepisYa pis1@qq.com
 * @LastEditTime: 2023-02-07 21:32:21
 * @FilePath: /rustlings/exercises/hashmaps/hashmaps3.rs
 * 路漫漫其修远兮，吾将上下而求索。
 * @Description:
 */
// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::hash_map::Entry;
use std::collections::HashMap;

// A structure to store team name and its goal details.
#[derive(Debug)]
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

/// 用于更新 判断 team 是否存在，存在则更新 Team,
/// 不存在则 插入新 Team
fn update_team_goals(match_entry: Entry<String, Team>, team: Team) -> &mut Team {
    match match_entry {
        // 已有值，则更新
        Entry::Occupied(entry) => {
            let temp_team = entry.into_mut();
            // 更新值
            temp_team.goals_conceded += team.goals_conceded;
            temp_team.goals_scored += team.goals_scored;
            temp_team
        }
        // 空， 则插入
        Entry::Vacant(entry) => entry.insert(team),
    }
    // 也可以直接使用 or_insert 如果是计次， 那么可以 直接 插入 0 ，
    // 累乘 则可以 or_insert(1)
    // 获取到返回的引用之后  直接进行 += 或者 *= 一举两得。
    // 参考文档
    // https://rustwiki.org/zh-CN/book/ch08-03-hash-maps.html#%E6%A0%B9%E6%8D%AE%E6%97%A7%E5%80%BC%E6%9B%B4%E6%96%B0%E4%B8%80%E4%B8%AA%E5%80%BC
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.

        let team_1_name_copy = team_1_name.clone();
        let team1 = Team {
            name: team_1_name,
            goals_scored: team_1_score.clone(),
            goals_conceded: team_2_score.clone(),
        };
        let team1_entry = scores.entry(team_1_name_copy.clone());
        update_team_goals(team1_entry, team1);

        let team_2_name_copy = team_2_name.clone();
        let team2 = Team {
            name: team_2_name,
            goals_scored: team_2_score,
            goals_conceded: team_1_score,
        };
        let team2_entry = scores.entry(team_2_name_copy.clone());
        update_team_goals(team2_entry, team2);
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();

        println!("{:#?}", scores);
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
