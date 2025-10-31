struct Item<'a> {
    name: &'a str,
    cost: u32,
    damage: u32,
    armor: u32,
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Item; 6] = [
    Item {
        name: "No Armor",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 8] = [
    Item {
        name: "No left Ring",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "No right Ring",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Clone)]
struct Player {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Player {
    pub fn new(hp: u32, damage: u32, armor: u32) -> Player {
        Player { hp, damage, armor }
    }

    pub fn equiped_with(hp: u32, items: Vec<&Item>) -> Player {
        Player {
            hp,
            damage: items.iter().map(|i| i.damage).sum(),
            armor: items.iter().map(|i| i.armor).sum(),
        }
    }

    pub fn wins_fight(&self, boss: &Player) -> bool {
        let mut player = self.clone();
        let mut boss = boss.clone();

        loop {
            boss.hp = boss
                .hp
                .saturating_sub(player.damage.saturating_sub(boss.armor));
            if boss.hp == 0 {
                return true;
            }

            player.hp = player
                .hp
                .saturating_sub(boss.damage.saturating_sub(player.armor));
            if player.hp == 0 {
                return false;
            }
        }
    }
}

pub fn day21_fst() -> u32 {
    let boss = Player::new(100, 8, 2);

    let mut min_gold = u32::MAX;

    for armor in ARMORS {
        for weapon in WEAPONS {
            for left_ring in RINGS {
                for right_ring in RINGS {
                    if left_ring.name == right_ring.name {
                        continue;
                    }

                    let player =
                        Player::equiped_with(100, vec![&armor, &weapon, &left_ring, &right_ring]);

                    if player.wins_fight(&boss) {
                        min_gold = min_gold
                            .min(armor.cost + weapon.cost + left_ring.cost + right_ring.cost);
                    }
                }
            }
        }
    }

    min_gold
}

pub fn day21_snd() -> u32 {
    let boss = Player::new(100, 8, 2);

    let mut max_gold = 0;

    for armor in ARMORS {
        for weapon in WEAPONS {
            for left_ring in RINGS {
                for right_ring in RINGS {
                    if left_ring.name == right_ring.name {
                        continue;
                    }

                    let player =
                        Player::equiped_with(100, vec![&armor, &weapon, &left_ring, &right_ring]);

                    if !player.wins_fight(&boss) {
                        max_gold = max_gold
                            .max(armor.cost + weapon.cost + left_ring.cost + right_ring.cost);
                    }
                }
            }
        }
    }

    max_gold
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day21_fst() {
        assert_eq!(day21_fst(), 91);
    }

    #[test]
    fn test_day21_snd() {
        assert_eq!(day21_snd(), 158);
    }
}
