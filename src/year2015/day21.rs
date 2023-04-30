use crate::*;

use itertools::Itertools;

#[derive(Clone, Copy)]
pub struct Character {
    hp: i16,
    damage: i16,
    armor: i16,
}

pub fn parse_input(input: &str) -> Result<Character> {
    let (hp, damage, armor) = scan_fmt::scan_fmt!(
        input,
        "Hit Points: {}\nDamage: {}\nArmor: {}",
        i16,
        i16,
        i16
    )?;
    Ok(Character { hp, damage, armor })
}

const WEAPONS: &[(usize, i16)] = &[(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
const ARMORS: &[(usize, i16)] =
    &[(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
const RINGS: &[(usize, i16, i16)] = &[
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

impl Character {
    fn get_hitted(&mut self, other: &Character) -> bool {
        self.hp -= (other.damage - self.armor).max(1);
        self.hp <= 0
    }

    fn win(&mut self, other: &Character) -> bool {
        let mut other = *other;
        loop {
            if other.get_hitted(self) {
                return true;
            }
            if self.get_hitted(&other) {
                return false;
            }
        }
    }
}

fn search(boss: &Character, need_min: bool) -> Result<usize> {
    let iter = WEAPONS.iter().flat_map(|weapon| {
        ARMORS.iter().flat_map(|armor| {
            (0..=2).flat_map(|count| {
                RINGS.iter().combinations(count).filter_map(|rings| {
                    let mut you = Character {
                        hp: 100,
                        damage: weapon.1
                            + rings.iter().map(|ring| ring.1).sum::<i16>(),
                        armor: armor.1
                            + rings.iter().map(|ring| ring.2).sum::<i16>(),
                    };

                    if need_min == you.win(boss) {
                        Some(
                            weapon.0
                                + armor.0
                                + rings
                                    .iter()
                                    .map(|ring| ring.0)
                                    .sum::<usize>(),
                        )
                    } else {
                        None
                    }
                })
            })
        })
    });
    if need_min {
        iter.min().ok_or(task_error!("Empty iterator on min"))
    } else {
        iter.max().ok_or(task_error!("Empty iterator on min"))
    }
}

pub fn task1(boss: &Character) -> Result<usize> {
    search(boss, true)
}

pub fn task2(boss: &Character) -> Result<usize> {
    search(boss, false)
}
