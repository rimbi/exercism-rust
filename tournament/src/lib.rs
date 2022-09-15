use std::{collections::BTreeMap, iter::once};

#[derive(PartialEq, Eq, Debug)]
struct Team {
    won: u8,
    loss: u8,
    draw: u8,
    name: String,
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            won: 0,
            loss: 0,
            draw: 0,
            name: name.to_string(),
        }
    }

    pub fn points(&self) -> u8 {
        self.won * 3 + self.draw
    }

    pub fn matches(&self) -> u8 {
        self.won + self.draw + self.loss
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.points().cmp(&other.points()) {
            core::cmp::Ordering::Equal => self.name.cmp(&other.name),
            ord => ord.reverse(),
        }
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.points().partial_cmp(&other.points()) {
            Some(core::cmp::Ordering::Equal) => self.name.partial_cmp(&other.name),
            ord => ord.map(|ord| ord.reverse()),
        }
    }
}

impl ToString for Team {
    fn to_string(&self) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.matches(),
            self.won,
            self.draw,
            self.loss,
            self.points()
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let res = match_results.lines().fold(BTreeMap::new(), |mut map, res| {
        let mut split = res.split(';');
        let t1 = split.next().unwrap();
        let t2 = split.next().unwrap();
        let res = split.next().unwrap();
        match res {
            "win" => {
                map.entry(t1).or_insert_with(|| Team::new(t1)).won += 1;
                map.entry(t2).or_insert_with(|| Team::new(t2)).loss += 1
            }
            "loss" => {
                map.entry(t2).or_insert_with(|| Team::new(t2)).won += 1;
                map.entry(t1).or_insert_with(|| Team::new(t1)).loss += 1
            }
            "draw" => {
                map.entry(t1).or_insert_with(|| Team::new(t1)).draw += 1;
                map.entry(t2).or_insert_with(|| Team::new(t2)).draw += 1
            }
            _ => panic!("invalid match result!"),
        }
        // v.entry(key)
        map
    });

    let mut res = res.values().collect::<Vec<_>>();
    res.sort();
    // println!("{:?}", res);
    once("Team                           | MP |  W |  D |  L |  P".to_string())
        .chain(res.iter().map(|t| t.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}
