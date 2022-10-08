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

/* TODO

Handle team name without clone()?
- I was able to do this, but, man, it got gnarly. Arguably, I should push the changes up into the Team struct. Anyway, I was able to some fun stuff with lifetimes. The borrow checker is my friend.

*/

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table<'a>(results: &'a str) -> HashMap<&'a str, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<&str, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0];
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1];
        let team_2_score: u8 = v[3].parse().unwrap();

        update_team_scores(&mut scores, team_1_name, team_1_score, team_2_score);
        update_team_scores(&mut scores, team_2_name, team_2_score, team_1_score);
    }

    scores
}

fn update_team_scores<'a>(
    scores: &mut HashMap<&'a str, Team>,
    name: &'a str,
    goals_scored: u8,
    goals_conceded: u8,
) {
    scores
        .entry(name)
        .and_modify(|team| {
            team.goals_scored += goals_scored;
            team.goals_conceded += goals_conceded
        })
        .or_insert(Team {
            name: String::from(name),
            goals_scored,
            goals_conceded,
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = String::from("")
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let results = get_results();
        let scores = build_scores_table(&results);

        let mut keys: Vec<&&str> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![&"England", &"France", &"Germany", &"Italy", &"Poland", &"Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let results = get_results();
        let scores = build_scores_table(&results);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let results = get_results();
        let scores = build_scores_table(&results);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
