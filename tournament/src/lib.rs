use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug, Default)]
pub struct TeamInfo {
    pub name: String,
    pub wins: u16,
    pub drawns: u16,
    pub losts: u16,
}

impl TeamInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, score: &str) {
        match score {
            "win" => {
                self.wins += 1;
            }
            "draw" => {
                self.drawns += 1;
            }
            "loss" => {
                self.losts += 1;
            }
            _ => unreachable!(),
        }
    }

    pub fn matches(&self) -> u16 {
        self.drawns + self.losts + self.wins
    }

    pub fn points(&self) -> u16 {
        1 * self.drawns + 3 * self.wins
    }
}

impl Display for TeamInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.matches(),
            self.wins,
            self.drawns,
            self.losts,
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let tally = match_results
        .lines()
        .fold(HashMap::new(), |mut tally, line| {
            let cols: Vec<&str> = line.trim().split(';').collect();
            let (team1, team2, score) = (cols[0], cols[1], cols[2]);
            let team_info1 = tally.entry(team1).or_insert(TeamInfo::new(&team1));
            team_info1.update(score);
            let team_info2 = tally.entry(team2).or_insert(TeamInfo::new(&team2));
            team_info2.update(if score == "win" {
                "loss"
            } else if score == "loss" {
                "win"
            } else {
                score
            });
            tally
        });
    let mut teams: Vec<TeamInfo> = tally.values().cloned().collect();
    teams.sort_by(|t1, t2| {
        t1.points()
            .cmp(&t2.points())
            .then_with(|| t2.name.to_lowercase().cmp(&t1.name.to_lowercase()))
    });
    let mut result = vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    for score in teams.iter().rev() {
        result.push(format!("{}", score));
    }

    let result = result.join("\n");
    result
}
