use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DamageInfo {
    pub name: String,
    pub attacks: Vec<AttackFlavor>,
    pub deaths: Vec<DeathFlavor>,
}
#[derive(Debug, Deserialize)]
pub struct AttackFlavor {
    pub player: (String, String),
    pub monster: (String, String),
}
#[derive(Debug, Deserialize)]
pub struct DeathFlavor {
    pub player: String,
    pub monster: (String, String),
}
