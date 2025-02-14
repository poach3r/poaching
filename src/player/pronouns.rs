pub const MALE: Pronouns = Pronouns {
    subject: "he",
    object: "him",
    possesive: "his",
    possessive_adj: "his",
    reflexive: "himself",
};

pub const FEMALE: Pronouns = Pronouns {
    subject: "she",
    object: "her",
    possesive: "hers",
    possessive_adj: "her",
    reflexive: "herself",
};

/// non-binary pronouns
pub const ENBY: Pronouns = Pronouns {
    subject: "they",
    object: "them",
    possesive: "theirs",
    possessive_adj: "their",
    reflexive: "themself",
};

/// The pronouns by which a player
/// is referred to in dialog.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Pronouns<'a> {
    pub subject: &'a str,
    pub object: &'a str,
    pub possesive: &'a str,
    pub possessive_adj: &'a str,
    pub reflexive: &'a str,
}
