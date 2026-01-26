use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana_refill: i32,
    duration: i32,
}

const SPELLS: [Spell; 5] = [
    Spell {
        name: "Magic Missile",
        cost: 53,
        damage: 4,
        heal: 0,
        armor: 0,
        mana_refill: 0,
        duration: 0,
    },
    Spell {
        name: "Drain",
        cost: 73,
        damage: 2,
        heal: 2,
        armor: 0,
        mana_refill: 0,
        duration: 0,
    },
    Spell {
        name: "Shield",
        cost: 113,
        damage: 0,
        heal: 0,
        armor: 7,
        mana_refill: 0,
        duration: 6,
    },
    Spell {
        name: "Poison",
        cost: 173,
        damage: 3,
        heal: 0,
        armor: 0,
        mana_refill: 0,
        duration: 6,
    },
    Spell {
        name: "Recharge",
        cost: 229,
        damage: 0,
        heal: 0,
        armor: 0,
        mana_refill: 101,
        duration: 5,
    },
];

#[derive(Clone)]
struct Player {
    hp: i32,
    damage: i32,
    mana: i32,
    armor: i32,
}

#[derive(Clone)]
pub struct GameState {
    player: Player,
    boss: Player,
    mana_spent: i32,
    effects: Vec<Spell>,
    mode: GameMode,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct StateKey {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    effects: [(i32, i32); 5], // (duration, effect_id)
    hard_mode: bool,
    player_turn: bool,
}

#[derive(Clone)]
enum GameMode {
    Normal,
    Hard,
}

impl GameState {
    fn easy(player: Player, boss: Player) -> Self {
        Self {
            player,
            boss,
            mana_spent: 0,
            effects: Vec::new(),
            mode: GameMode::Normal,
        }
    }

    fn make_key(&self) -> StateKey {
        let mut effects_key = [(0, 0); 5];
        for (i, spell) in SPELLS.iter().enumerate() {
            if let Some(effect) = self.effects.iter().find(|e| e.name == spell.name) {
                effects_key[i] = (effect.duration, i as i32);
            }
        }

        StateKey {
            player_hp: self.player.hp,
            player_mana: self.player.mana,
            boss_hp: self.boss.hp,
            effects: effects_key,
            hard_mode: matches!(self.mode, GameMode::Hard),
            player_turn: true,
        }
    }

    fn hard(player: Player, boss: Player) -> Self {
        Self {
            player,
            boss,
            mana_spent: 0,
            effects: Vec::new(),
            mode: GameMode::Hard,
        }
    }

    fn apply_effects(&mut self) {
        self.player.armor = 0;
        for effect in self.effects.iter_mut() {
            self.player.armor += effect.armor;
            self.player.mana += effect.mana_refill;
            self.boss.hp -= effect.damage;
            effect.duration -= 1;
        }
        self.effects.retain(|effect| effect.duration > 0);
    }

    fn cast_spell(&mut self, spell: &Spell) {
        if spell.duration > 0 {
            self.effects.push(spell.clone());
        } else {
            self.boss.hp -= spell.damage;
            self.player.hp += spell.heal;
        }

        self.player.mana -= spell.cost;
        self.mana_spent += spell.cost;
    }

    fn can_cast_spell(&self, spell: &Spell) -> bool {
        if self.player.mana < spell.cost {
            return false;
        }

        if spell.duration > 0
            && self
                .effects
                .iter()
                .any(|e| e.name == spell.name && e.duration > 1)
        {
            return false;
        }

        true
    }

    fn player_turn(&mut self, spell: &Spell) {
        if let GameMode::Hard = self.mode {
            self.player.hp -= 1;
            if self.is_game_over() {
                return;
            }
        }
        self.apply_effects();
        if self.is_game_over() {
            return;
        }
        self.cast_spell(spell);
    }

    fn boss_turn(&mut self) {
        self.apply_effects();
        if self.is_game_over() {
            return;
        }
        let damage = (self.boss.damage - self.player.armor).max(1);
        self.player.hp -= damage;
    }

    fn is_game_over(&self) -> bool {
        self.player.hp <= 0 || self.boss.hp <= 0
    }

    fn player_won(&self) -> bool {
        self.boss.hp <= 0
    }

    pub fn cast(&mut self, spell: &Spell) {
        self.player_turn(spell);
        if !self.is_game_over() {
            self.boss_turn();
        }
    }

    /// Computes the minimum mana spent to win from this state
    fn min_mana_to_win_helper(&self, memo: &mut HashMap<StateKey, Option<i32>>) -> Option<i32> {
        if self.is_game_over() {
            return if self.player_won() {
                Some(self.mana_spent)
            } else {
                None
            };
        }

        let key = self.make_key();
        if let Some(cached) = memo.get(&key) {
            return *cached;
        }

        let mut min_mana: Option<i32> = None;
        for spell in SPELLS.iter() {
            if self.can_cast_spell(spell) {
                let mut next_state = self.clone();
                next_state.cast(spell);
                match next_state.min_mana_to_win_helper(memo) {
                    Some(cost) => {
                        min_mana = match min_mana {
                            Some(current_min) => Some(current_min.min(cost)),
                            None => Some(cost),
                        }
                    }
                    None => continue,
                }
            }
        }

        memo.insert(key, min_mana);

        min_mana
    }
    pub fn min_mana_to_win(&self) -> Option<i32> {
        let mut memo: HashMap<StateKey, Option<i32>> = HashMap::new();
        self.min_mana_to_win_helper(&mut memo)
    }
}

pub fn day22_fst() -> i32 {
    let player = Player {
        hp: 50,
        damage: 0,
        mana: 500,
        armor: 0,
    };

    let boss = Player {
        hp: 55,
        damage: 8,
        mana: 0,
        armor: 0,
    };

    GameState::easy(player, boss).min_mana_to_win().unwrap_or(0)
}

pub fn day22_snd() -> i32 {
    let player = Player {
        hp: 50,
        damage: 0,
        mana: 500,
        armor: 0,
    };

    let boss = Player {
        hp: 55,
        damage: 8,
        mana: 0,
        armor: 0,
    };

    GameState::hard(player, boss).min_mana_to_win().unwrap_or(0)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example_fst_one() {
        let player = Player {
            hp: 10,
            damage: 0,
            mana: 250,
            armor: 0,
        };

        let boss = Player {
            hp: 13,
            damage: 8,
            mana: 0,
            armor: 0,
        };

        let mut state = GameState::easy(player, boss);

        state.cast(&SPELLS[3]); // Poison
        assert_eq!(state.player.hp, 2);
        assert_eq!(state.boss.hp, 10);
        assert_eq!(state.player.mana, 77);

        state.cast(&SPELLS[0]); // Magic Missile
        assert_eq!(state.player.hp, 2);
        assert_eq!(state.player.mana, 24);
        assert!(state.player_won());

        assert_eq!(state.mana_spent, 226);
    }

    #[test]
    fn example_fst_two() {
        let player = Player {
            hp: 10,
            damage: 0,
            mana: 250,
            armor: 0,
        };

        let boss = Player {
            hp: 14,
            damage: 8,
            mana: 0,
            armor: 0,
        };

        let mut state = GameState::easy(player, boss);
        state.cast(&SPELLS[4]); // Recharge
        assert_eq!(state.player.hp, 2);
        assert_eq!(state.boss.hp, 14);
        assert_eq!(state.player.mana, 122);
        assert_eq!(
            state.effects,
            vec![Spell {
                name: "Recharge",
                cost: 229,
                damage: 0,
                heal: 0,
                armor: 0,
                mana_refill: 101,
                duration: 4,
            }]
        );

        state.cast(&SPELLS[2]); // Shield
        assert_eq!(state.player.hp, 1);
        assert_eq!(state.boss.hp, 14);
        assert_eq!(state.player.armor, 7);
        assert_eq!(state.player.mana, 211);
        assert_eq!(
            state.effects,
            vec![
                Spell {
                    name: "Recharge",
                    cost: 229,
                    damage: 0,
                    heal: 0,
                    armor: 0,
                    mana_refill: 101,
                    duration: 2,
                },
                Spell {
                    name: "Shield",
                    cost: 113,
                    damage: 0,
                    heal: 0,
                    armor: 7,
                    mana_refill: 0,
                    duration: 5,
                },
            ]
        );

        state.cast(&SPELLS[1]); // Drain
        assert_eq!(state.player.hp, 2);
        assert_eq!(state.player.armor, 7);
        assert_eq!(state.boss.hp, 12);
        assert_eq!(state.player.mana, 340);
        assert_eq!(
            state.effects,
            vec![Spell {
                name: "Shield",
                cost: 113,
                damage: 0,
                heal: 0,
                armor: 7,
                mana_refill: 0,
                duration: 3,
            },]
        );

        state.cast(&SPELLS[3]); // Poison
        assert_eq!(state.player.armor, 7);
        assert_eq!(state.player.hp, 1);
        assert_eq!(state.boss.hp, 9);
        assert_eq!(state.player.mana, 167);

        state.cast(&SPELLS[0]); // Magic Missile
        assert_eq!(state.player.hp, 1);
        assert_eq!(state.player.mana, 114);
        assert!(state.player_won());
    }

    #[test]
    fn solution_fst() {
        assert_eq!(day22_fst(), 953);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day22_snd(), 1289);
    }

    #[test]
    fn hard_mode_reduces_hp() {
        let player = Player {
            hp: 10,
            damage: 0,
            mana: 250,
            armor: 0,
        };

        let boss = Player {
            hp: 13,
            damage: 8,
            mana: 0,
            armor: 0,
        };

        let mut hard_state = GameState::hard(player.clone(), boss.clone());
        let mut easy_state = GameState::easy(player, boss);

        hard_state.cast(&SPELLS[0]); // Magic Missile
        easy_state.cast(&SPELLS[0]); // Magic Missile

        assert_eq!(hard_state.player.hp, easy_state.player.hp - 1);
    }
}
