pub mod item;
pub mod pronouns;
pub mod status;

use log::warn;
use serde::{Deserialize, Serialize};

use pronouns::*;
use status::*;

/// Represents a player in the simulation.
#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub name: String,
    pub status: Status,
    pub pronouns: Pronouns<'a>,
    pub moved: bool,
    pub district: u8,
    pub inventory: Vec<&'a item::Item>,
    pub kills: u16,
}

/// Represents a player in JSON form.
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonPlayer {
    name: String,
    gender: String,
    district: u8,
}

impl<'a> From<&JsonPlayer> for Player<'a> {
    fn from(value: &JsonPlayer) -> Self {
        let pronouns = match value.gender.to_lowercase().as_str() {
            "male" => MALE,
            "female" => FEMALE,
            "enby" => ENBY,
            _ => {
                warn!(
                    "Failed to deserialize {}'s gender, defaulting to ENBY.",
                    &value.name
                );
                ENBY
            }
        };
        Self::new(value.name.clone(), pronouns, value.district)
    }
}

impl<'a> Player<'a> {
    /// Constructs a player at the start of a simulator.
    pub fn new(name: String, pronouns: Pronouns<'a>, district: u8) -> Self {
        Self {
            name,
            status: Status::Alive(AliveStatus::Healthy),
            pronouns,
            moved: false,
            district,
            inventory: Vec::new(),
            kills: 0,
        }
    }

    /// Returns the index of the first item in the
    /// `Player`'s inventory with the specified `kind`.
    /// If no such item exists then return `None`.
    pub fn get_item_kind(&self, kind: item::Kind) -> Option<usize> {
        for (i, item) in self.inventory.iter().enumerate() {
            if kind == item.kind {
                return Some(i);
            }
        }

        return None;
    }

    /// Returns the index of the first item in the
    /// `Player`'s inventory which matches `predicate_item`.
    /// If no such item exists then return `None`.
    pub fn get_item(&self, predicate_item: &item::Item) -> Option<usize> {
        for (i, item) in self.inventory.iter().enumerate() {
            if *item == predicate_item {
                return Some(i);
            }
        }

        None
    }

    /// Changes the player's `status` to `Dead`.
    pub fn kill(&mut self) {
        self.status = Status::Dead;
    }

    /// Changes a player's `status` to `Alive(Injured)` if their
    /// current `status` is `Alive(Healthy), otherwise sets it
    /// to `Dead`.
    pub fn hurt(&mut self) {
        match self.status {
            Status::Alive(AliveStatus::Healthy) => {
                self.status = Status::Alive(AliveStatus::Injured);
            }
            _ => {
                self.kill();
            }
        }
    }

    /// Changes a player's `status` to `Alive(Healthy)`.
    pub fn heal(&mut self) {
        self.status = Status::Alive(AliveStatus::Healthy);
    }
}
