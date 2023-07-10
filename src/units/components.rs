use bevy::prelude::*;

use crate::hexagon::{hex_to_pixel, hexes_in_range, hexes_in_ring, Cube, cube_scale_vec};

pub enum UnitDefault {
    Archer,
    BladeDancer,
    Scout,
    Knight,
    Catapult,
    Sniper,
    Newt,
}

impl UnitDefault {
    pub fn sprite_location(&self) -> String {
        match self {
            UnitDefault::Archer => "sprites/bow.png".to_string(),
            UnitDefault::BladeDancer => "sprites/knife.png".to_string(),
            UnitDefault::Scout => "sprites/boot.png".to_string(),
            UnitDefault::Knight => "sprites/shield.png".to_string(),
            UnitDefault::Catapult => "sprites/comet.png".to_string(),
            UnitDefault::Sniper => "sprites/gun.png".to_string(),
            UnitDefault::Newt => "sprites/frog.png".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Keyword {
    // amount of damage reduced
    // for every hit (like repulsion armor)
    Armor(i32),
    // Restores health every turn
    Regeneration(i32),
    // When attacked, will strike
    // the attacker
    StrikeBack,
    // if they kill they take
    // the place of the defender
    Nimble,
    // if you get a kill, it doesnt take
    // your attack action
    Executioner,
    // gets move action only when countdown
    // hits 0
    Slow { max_countdown: i32, countdown: i32 },
    // Attacking this unit does
    // not take the attack action
    Despised,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Move,
    Attack,
}

#[derive(Component, Clone)]
pub struct Unit {
    pub position: Cube,
    pub ally: bool,
    pub max_health: i32,
    pub health: i32,
    pub damage: i32,
    pub keywords: Vec<Keyword>,
    pub actions: Vec<Action>,
    // tiles relative to current
    // that can be moved to
    pub move_hexes: Vec<Cube>,
    // tiles relative to current
    // that can be attacked
    pub attack_hexes: Vec<Cube>,
}

impl Unit {
    pub fn new_default(default: UnitDefault, position: Cube, ally: bool) -> Self {
        let actions = vec![Action::Move];

        match default {
            UnitDefault::Archer => Unit {
                position,
                ally,
                max_health: 1,
                health: 1,
                damage: 2,
                keywords: Vec::new(),
                actions,
                move_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
                attack_hexes: hexes_in_ring(2, Cube::axial_new(0, 0)),
            },
            UnitDefault::BladeDancer => Unit {
                position,
                ally,
                max_health: 2,
                health: 2,
                damage: 2,
                keywords: vec![Keyword::Nimble, Keyword::Executioner],
                actions,
                move_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
                attack_hexes: Cube::CUBE_DIAGONAL_VECTORS.to_vec(),
            },
            UnitDefault::Scout => Unit {
                position,
                ally,
                max_health: 1,
                health: 1,
                damage: 1,
                keywords: Vec::new(),
                actions,
                move_hexes: hexes_in_range(2, Cube::axial_new(0, 0)),
                attack_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
            },
            UnitDefault::Knight => Unit {
                position,
                ally,
                max_health: 4,
                health: 4,
                damage: 2,
                keywords: vec![Keyword::Armor(1)],
                actions,
                move_hexes: [Cube::CUBE_DIRECTION_VECTORS, Cube::CUBE_DIAGONAL_VECTORS].concat(),
                attack_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
            },
            UnitDefault::Catapult => Unit {
                position,
                ally,
                max_health: 2,
                health: 2,
                damage: 4,
                keywords: Vec::new(),
                actions,
                move_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
                attack_hexes: hexes_in_ring(3, Cube::axial_new(0, 0)),
            },
            UnitDefault::Sniper => Unit {
                position,
                ally,
                max_health: 1,
                health: 1,
                damage: 10,
                keywords: vec![Keyword::Slow {
                    max_countdown: 2,
                    countdown: 2,
                }],
                actions,
                move_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
                attack_hexes: cube_scale_vec(Cube::CUBE_DIRECTION_VECTORS.to_vec(), 5),
            },
            UnitDefault::Newt => Unit {
                position,
                ally,
                max_health: 10,
                health: 6,
                damage: 2,
                keywords: vec![Keyword::StrikeBack, Keyword::Regeneration(2), Keyword::Despised],
                actions,
                move_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
                attack_hexes: Cube::CUBE_DIRECTION_VECTORS.to_vec(),
            },
        }
    }

    pub fn remove_action(&mut self, action: Action) -> bool {
        let Some(i) = self.actions.iter().position(|a| *a == action) else {
            return false;
        };

        self.actions.remove(i);
        return true;
    }

    pub fn new_turn(&mut self) {
        let regen = self.keywords.iter().find_map(|k| match k {
            Keyword::Regeneration(amount) => Some(amount),
            _ => None,
        });

        self.health += regen.unwrap_or(&0);
        self.health = self.health.min(self.max_health);

        let slow = self.keywords.iter_mut().find_map(|k| match k {
            Keyword::Slow {
                max_countdown,
                countdown,
            } => Some((max_countdown, countdown)),
            _ => None,
        });

        let Some((max_countdown, countdown)) = slow else {
            self.actions = vec![Action::Move, Action::Attack];
            return;
        };

        if self.actions.contains(&Action::Move) {
            self.actions = vec![Action::Move, Action::Attack];
            return;
        }

        if *countdown == 0 {
            self.actions = vec![Action::Move, Action::Attack];
            *countdown = *max_countdown;
        } else {
            *countdown -= 1;
        }
    }

    fn take_damage(&mut self, damage: i32) -> bool {
        let armor = self.keywords.iter().find_map(|k| match k {
            Keyword::Armor(amount) => Some(amount),
            _ => None,
        });
        let armored_damage = (damage - armor.unwrap_or(&0)).max(0);
        self.health -= armored_damage;
        println!("{}", self.health);

        return self.health <= 0;
    }

    pub fn attack(&mut self, my_transform: &mut Transform, opponent: &mut Unit) {
        let killed = opponent.take_damage(self.damage);

        if !((killed && self.keywords.contains(&Keyword::Executioner)) || opponent.keywords.contains(&Keyword::Despised)) {
            self.remove_action(Action::Attack);
        }
        
        if !killed && opponent.keywords.contains(&Keyword::StrikeBack) {
            self.take_damage(opponent.damage);
        }

        if self.keywords.contains(&Keyword::Nimble) && killed {
            self.position = opponent.position;

            let (x, y) = hex_to_pixel(self.position);
            my_transform.translation = Vec3::new(x, y, 1.0);
        }
    }

    pub fn absolute_attack_hexes(&self) -> Vec<Cube> {
        let mut hexes = Vec::new();

        for hex in &self.attack_hexes {
            hexes.push(self.position.cube_add(*hex));
        }

        return hexes;
    }

    pub fn absolute_move_hexes(&self) -> Vec<Cube> {
        let mut hexes = Vec::new();

        for hex in &self.move_hexes {
            hexes.push(self.position.cube_add(*hex));
        }

        return hexes;
    }
}
