#[derive(Clone)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut new_self = self.clone();
            new_self.health = 100;

            if self.level >= 10 {
                new_self.mana = Some(100);
            } else {
                new_self.mana = None;
            }

            Some(new_self)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana >= mana_cost {
                self.mana = Some(mana - mana_cost);

                mana_cost * 2
            } else {
                0
            }
        } else {
            if self.health >= mana_cost {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }

            0
        }
    }
}
