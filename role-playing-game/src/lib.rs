// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health >= 1 {
            return None
        }
        let mut  p = Self{
            health: self.health,
            mana:  self.mana,
            level:  self.level
        };
        if self.health == 0 {
            p.health = 100;
        }
        p.mana = if self.level >= 10 {
            Some(100)
        } else {
            None
        };
        Some(p)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
            Some(x) if x < mana_cost => 0,
            Some(x) => {
                self.mana = Some(x - mana_cost);
                2 * mana_cost
            }
        }
    }
}
