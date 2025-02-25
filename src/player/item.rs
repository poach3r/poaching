#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub name: &'static str,
    pub kind: Kind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    Weapon,
    Defense,
    Healing,
    Consumable,
}

pub const SICKLE: Item = Item {
    name: "sickle",
    kind: Kind::Weapon,
};

pub const SWORD: Item = Item {
    name: "sword",
    kind: Kind::Weapon,
};

pub const KNIFE: Item = Item {
    name: "knife",
    kind: Kind::Weapon,
};

pub const EXPLOSIVES: Item = Item {
    name: "bag of explosives",
    kind: Kind::Weapon,
};

pub const SHIELD: Item = Item {
    name: "shield",
    kind: Kind::Defense,
};

pub const MEDKIT: Item = Item {
    name: "first-aid kit",
    kind: Kind::Healing,
};

pub const BREAD: Item = Item {
    name: "bread",
    kind: Kind::Consumable,
};

pub const WATER: Item = Item {
    name: "bottled water",
    kind: Kind::Consumable,
};
