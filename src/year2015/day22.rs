use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Game {
    hp: i32,
    mp: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    mana_spent: i32,
    boss_hp: i32,
    boss_dmg: i32,
}

pub fn parse_input(input: &str) -> Result<Game> {
    let (hp, dmg) = scan_fmt::scan_fmt!(
        input,
        "Hit Points: {}
Damage: {}",
        i32,
        i32
    )
    .map_err(|_| task_error!("Wrong input format"))?;
    Ok(Game::new(hp, dmg))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

static PRICES: Lazy<HashMap<Spell, i32>> = Lazy::new(|| {
    HashMap::from([
        (Spell::MagicMissile, 53),
        (Spell::Drain, 73),
        (Spell::Shield, 113),
        (Spell::Poison, 173),
        (Spell::Recharge, 229),
    ])
});

impl Game {
    fn new(hp: i32, dmg: i32) -> Game {
        Game {
            hp: 50,
            mp: 500,
            shield: 0,
            poison: 0,
            recharge: 0,
            mana_spent: 0,
            boss_hp: hp,
            boss_dmg: dmg,
        }
    }
    fn effects(&mut self) {
        if self.shield > 0 {
            self.shield -= 1;
        }
        if self.poison > 0 {
            self.boss_hp -= 3;
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.mp += 101;
            self.recharge -= 1;
        }
    }
    fn player_action(&mut self, action: Spell, hard_mode: bool) -> bool {
        if hard_mode {
            self.hp -= 1;
        }
        if self.lose() {
            return false;
        }
        let price = PRICES[&action];
        if price > self.mp {
            return false;
        }
        match action {
            Spell::MagicMissile => {
                self.boss_hp -= 4;
            }
            Spell::Drain => {
                self.hp += 2;
                self.boss_hp -= 2;
            }
            Spell::Shield if self.shield == 0 => {
                self.shield = 6;
            }
            Spell::Poison if self.poison == 0 => {
                self.poison = 6;
            }
            Spell::Recharge if self.recharge == 0 => {
                self.recharge = 5;
            }
            _ => return false,
        }
        self.mp -= price;
        self.mana_spent += price;
        true
    }
    fn boss_action(&mut self) {
        if self.shield == 0 {
            self.hp -= self.boss_dmg;
        } else {
            self.hp -= self.boss_dmg - 7;
        }
    }
    fn win(&self) -> bool {
        self.hp > 0 && self.boss_hp <= 0
    }
    fn lose(&self) -> bool {
        self.hp <= 0
    }
}

pub fn task(&game: &Game, hard_mode: bool) -> Result<i32> {
    let mut situations = HashSet::from([game]);
    let mut best_mana = None;
    while !situations.is_empty() {
        let mut new_situations = HashSet::new();
        for game in situations {
            for action in [
                Spell::MagicMissile,
                Spell::Drain,
                Spell::Shield,
                Spell::Poison,
                Spell::Recharge,
            ] {
                let mut game = game;
                game.effects();
                if game.player_action(action, hard_mode) {
                    game.effects();
                    if game.win() {
                        best_mana = Some(best_mana.map_or_else(
                            || game.mana_spent,
                            |x: i32| x.min(game.mana_spent),
                        ));
                    }
                    game.boss_action();
                    if !game.lose()
                        && best_mana
                            .map_or_else(|| true, |x| x > game.mana_spent)
                    {
                        new_situations.insert(game);
                    }
                }
            }
        }
        situations = new_situations;
    }
    best_mana.ok_or(task_error!("No solution found"))
}

pub fn task1(game: &Game) -> Result<i32> {
    task(game, false)
}

pub fn task2(game: &Game) -> Result<i32> {
    task(game, true)
}
