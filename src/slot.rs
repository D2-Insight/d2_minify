use num_enum::IntoPrimitive;
use serde_repr::{Deserialize_repr, Serialize_repr};

///This is used to convert a u32 "equipmentSlotTypeHash" to and from a u8 enum to save space.
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
pub enum MiniSlot {
    KineticWeapons = 1,
    EnergyWeapons = 2,
    PowerWeapons = 3,
    Ghost = 4,
    Helmet = 5,
    Gauntlets = 6,
    ChestArmor = 7,
    LegArmor = 8,
    Vehicle = 9,
    Emblems = 10,
    Ships = 11,
    ClanBanners = 12,
    Subclass = 13,
}

impl From<MiniSlot> for u32 {
    fn from(value: MiniSlot) -> Self {
        match value {
            MiniSlot::KineticWeapons => 1498876634,
            MiniSlot::EnergyWeapons => 2465295065,
            MiniSlot::PowerWeapons => 953998645,
            MiniSlot::Ghost => 4023194814,
            MiniSlot::Helmet => 3448274439,
            MiniSlot::Gauntlets => 3551918588,
            MiniSlot::ChestArmor => 14239492,
            MiniSlot::LegArmor => 20886954,
            MiniSlot::Vehicle => 2025709351,
            MiniSlot::Emblems => 4274335291,
            MiniSlot::Ships => 284967655,
            MiniSlot::ClanBanners => 4292445962,
            MiniSlot::Subclass => 3284755031,
        }
    }
}

impl TryFrom<u32> for MiniSlot {
    type Error = String;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1498876634 => Self::KineticWeapons,
            2465295065 => Self::EnergyWeapons,
            953998645 => Self::PowerWeapons,
            4023194814 => Self::Ghost,
            3448274439 => Self::Helmet,
            3551918588 => Self::Gauntlets,
            14239492 => Self::ChestArmor,
            20886954 => Self::LegArmor,
            2025709351 => Self::Vehicle,
            4274335291 => Self::Emblems,
            284967655 => Self::Ships,
            4292445962 => Self::ClanBanners,
            3284755031 => Self::Subclass,
            n => return Err(format!("Unknown Slot Hash: {}\n", n)),
        })
    }
}
