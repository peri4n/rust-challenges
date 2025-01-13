#[derive(Clone)]
struct Spell {
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
struct GameState {
    player: Player,
    boss: Player,
    mana_spent: i32,
    effects: Vec<Spell>,
}

impl GameState {
    fn new(player: Player, boss: Player) -> Self {
        Self {
            player,
            boss,
            mana_spent: 0,
            effects: Vec::new(),
        }
    }

    fn apply_effects(&mut self) {
        for effect in self.effects.iter_mut() {
            self.player.armor += effect.armor;
            self.player.mana += effect.mana_refill;
            self.boss.hp -= effect.damage;
            effect.duration -= 1;
        }
        self.effects.retain(|effect| effect.duration > 0);
    }

    fn cast_spell(&mut self, spell: &Spell) {
        self.player.mana -= spell.cost;
        self.mana_spent += spell.cost;
        self.player.armor += spell.armor;
        self.player.mana += spell.mana_refill;
        self.boss.hp -= spell.damage;
        self.player.hp += spell.heal;
        if spell.duration > 0 {
            self.effects.push(spell.clone());
        }
    }

    fn player_turn(&mut self, spell: &Spell) {
        self.apply_effects();
        self.cast_spell(spell);
    }

    fn boss_turn(&mut self) {
        self.apply_effects();
        let damage = self.boss.damage - self.player.armor;
        self.player.hp -= damage;
    }

    fn is_game_over(&self) -> bool {
        self.player.hp <= 0 || self.boss.hp <= 0
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
        hp: 71,
        damage: 10,
        mana: 0,
        armor: 0,
    };

    let state = GameState::new(player, boss);

    // backtrack through all possible spell combinations
    0
}

pub fn day22_snd() -> i32 {
    todo!()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn name() {
        todo!();
    }
}
