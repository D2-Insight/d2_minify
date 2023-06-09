use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntoPrimitive, Hash, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
pub enum MiniStat {
    NoName = 0,
    Accuracy = 1,
    AimAssist = 2,
    Airborne = 3,
    AmmoCapacity = 4,
    Attack = 5,
    BlastRadius = 6,
    ChargeRate = 7,
    ChargeTime = 8,
    Discipline = 9,
    DrawTime = 10,
    GuardEfficiency = 11,
    GuardEndurance = 12,
    GuardResistance = 13,
    Handling = 14,
    Impact = 15,
    Intellect = 16,
    InventorySize = 17,
    Magazine = 18,
    Mobility = 19,
    Power = 20,
    Range = 21,
    RecoilDir = 22,
    Recovery = 23,
    Reload = 24,
    Resilience = 25,
    RPM = 26,
    ShieldDuration = 27,
    Stability = 28,
    Strength = 29,
    SwingSpeed = 30,
    Velocity = 31,
    Zoom = 32,
}

impl From<MiniStat> for Option<u32> {
    fn from(value: MiniStat) -> Self {
        Some(match value {
            MiniStat::Accuracy => 1591432999,
            MiniStat::AimAssist => 1345609583,
            MiniStat::Airborne => 2714457168,
            MiniStat::AmmoCapacity => 925767036,
            MiniStat::Attack => 1480404414,
            MiniStat::BlastRadius => 3614673599,
            MiniStat::ChargeRate => 3022301683,
            MiniStat::ChargeTime => 2961396640,
            MiniStat::Discipline => 1735777505,
            MiniStat::DrawTime => 447667954,
            MiniStat::GuardEfficiency => 2762071195,
            MiniStat::GuardEndurance => 3736848092,
            MiniStat::GuardResistance => 209426660,
            MiniStat::Handling => 943549884,
            MiniStat::Impact => 4043523819,
            MiniStat::Intellect => 144602215,
            MiniStat::InventorySize => 1931675084,
            MiniStat::Magazine => 3871231066,
            MiniStat::Mobility => 2996146975,
            MiniStat::Power => 1935470627,
            MiniStat::Range => 1240592695,
            MiniStat::RecoilDir => 2715839340,
            MiniStat::Recovery => 1943323491,
            MiniStat::Reload => 4188031367,
            MiniStat::Resilience => 392767087,
            MiniStat::RPM => 4284893193,
            MiniStat::ShieldDuration => 1842278586,
            MiniStat::Stability => 155624089,
            MiniStat::Strength => 4244567218,
            MiniStat::SwingSpeed => 2837207746,
            MiniStat::Velocity => 2523465841,
            MiniStat::Zoom => 3555269338,
            MiniStat::NoName => return None,
        })
    }
}

#[cfg(feature = "pre_gen")]
impl TryFrom<u32> for MiniStat {
    type Error = String;

    fn try_from(_value: u32) -> Result<Self, Self::Error> {
        Ok(match _value {
            1591432999 => Self::Accuracy,
            1345609583 => Self::AimAssist,
            2714457168 => Self::Airborne,
            925767036 => Self::AmmoCapacity,
            1480404414 => Self::Attack,
            3614673599 => Self::BlastRadius,
            3022301683 => Self::ChargeRate,
            2961396640 => Self::ChargeTime,
            1735777505 => Self::Discipline,
            447667954 => Self::DrawTime,
            2762071195 => Self::GuardEfficiency,
            3736848092 => Self::GuardEndurance,
            209426660 => Self::GuardResistance,
            943549884 => Self::Handling,
            4043523819 => Self::Impact,
            144602215 => Self::Intellect,
            1931675084 => Self::InventorySize,
            3871231066 => Self::Magazine,
            2996146975 => Self::Mobility,
            1935470627 => Self::Power,
            1240592695 => Self::Range,
            2715839340 => Self::RecoilDir,
            1943323491 => Self::Recovery,
            4188031367 => Self::Reload,
            392767087 => Self::Resilience,
            4284893193 => Self::RPM,
            1842278586 => Self::ShieldDuration,
            155624089 => Self::Stability,
            4244567218 => Self::Strength,
            2837207746 => Self::SwingSpeed,
            2523465841 => Self::Velocity,
            3555269338 => Self::Zoom,
            //Ew :(
            1885944937 | 953546184 | 2299076437 | 2755327046 | 3291498658 | 3291498656
            | 3123546339 | 3291498659 | 3291498661 | 3409715177 | 3784226438 => Self::NoName,
            n => return Err(format!("Unknown Stat Hash: {}\n", n)),
        })
    }
}
