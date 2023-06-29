use bevy::prelude::*;

use crate::hexagon::Cube;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Keyword {
    // amount of damage reduced
    // for every hit (like repulsion armor)
    Armor(i32),
    // Heals health every turn
    Regeneration(i32),
    // when attacking, the defender
    // does not striker back
    FastAttack,
    // if they kill they take
    // the place of the defender
    Haste,
}

#[derive(Clone, Copy)]
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
    pub fn new(
        position: Cube,
        ally: bool,
        health: i32,
        damage: i32,
        keywords: Vec<Keyword>,
        relative_move_hexes: Vec<Cube>,
        relative_attack_hexes: Vec<Cube>,
    ) -> Self {
        Self {
            position,
            ally,
            max_health: health,
            health,
            damage,
            keywords,
            actions: vec![Action::Move],
            move_hexes: relative_move_hexes,
            attack_hexes: relative_attack_hexes,
        }
    }

    pub fn test_new(position: Cube) -> Self {
        Self::new(
            position,
            true,
            10,
            4,
            Vec::new(),
            Cube::CUBE_DIRECTION_VECTORS.into(),
            Cube::CUBE_DIRECTION_VECTORS.into(),
        )
    }

    pub fn new_turn(&mut self) {
        let regen = self.keywords.iter().find_map(|k| match k {
            Keyword::Regeneration(amount) => Some(amount),
            _ => None,
        });

        self.health += regen.unwrap_or(&0);
        self.health = self.health.min(self.max_health);

        self.actions = vec![Action::Move, Action::Attack];
    }

    pub fn take_damage(&mut self, damage: i32) -> bool {
        let armor = self.keywords.iter().find_map(|k| match k {
            Keyword::Armor(amount) => Some(amount),
            _ => None,
        });
        self.health -= damage - armor.unwrap_or(&0);

        return self.health <= 0;
    }

    pub fn attack(&mut self, opponent: &mut Unit) {
        let killed = opponent.take_damage(self.damage);

        if !killed && !self.keywords.contains(&Keyword::FastAttack) {
            self.take_damage(opponent.damage);
        }

        if self.keywords.contains(&Keyword::Haste) && killed {
            self.position = opponent.position;
        }
    }

    pub fn absolute_attack_hexes(&self) -> Vec<Cube> {
        let mut hexes = Vec::new();

        for hex in &self.attack_hexes {
            hexes.push(self.position.cube_add(hex));
        }

        return hexes;
    }

    pub fn absolute_move_hexes(&self) -> Vec<Cube> {
        let mut hexes = Vec::new();

        for hex in &self.move_hexes {
            hexes.push(self.position.cube_add(hex));
        }

        return hexes;
    }
}
