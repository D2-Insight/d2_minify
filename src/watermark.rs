use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]

pub enum MiniWatermark {
    Dawning = 1,
    FOTL = 2,
    GuardianGames = 3,
    Solstice = 4,
    CrimsonDays = 5,
    RedWar = 6,
    CurseOfOsiris = 7,
    Warmind = 8,
    Forsaken = 9,
    Forge = 10,
    Drifter = 11,
    Opulence = 12,
    Shadowkeep = 13,
    Undying = 14,
    Dawn = 15,
    Worthy = 16,
    Arrivals = 17,
    BeyondLight = 18,
    Hunt = 19,
    Chosen = 20,
    Splicer = 21,
    Lost = 22,
    Anniversary = 23,
    WitchQueen = 24,
    Risen = 25,
    Haunted = 26,
    Plunder = 27,
    Seraph = 28,
    LightFall = 29,
    Defiance = 30,
    TheDeep = 31,
}

//Produced URL for icon from season
//This is used during runtime
impl From<MiniWatermark> for String {
    fn from(val: MiniWatermark) -> Self {
        let buffer = match val {
            MiniWatermark::Dawning => "ad7fdb049d430c1fac1d20cf39059702",
            MiniWatermark::FOTL => "f80e39c767f309f0b2be625dae0e3744",
            MiniWatermark::GuardianGames => "efdb35540cd169fa6e334995c2ce87b6",
            MiniWatermark::CrimsonDays => "04de56db6d59127239ed51e82d16c06c",
            MiniWatermark::Solstice => "52523b49e5965f6f33ab86710215c676",
            MiniWatermark::RedWar => "fb50cd68a9850bd323872be4f6be115c",
            MiniWatermark::CurseOfOsiris => "2c024f088557ca6cceae1e8030c67169",
            MiniWatermark::Warmind => "ed6c4762c48bd132d538ced83c1699a6",
            MiniWatermark::Forsaken => "1b6c8b94cec61ea42edb1e2cb6b45a31",
            MiniWatermark::Forge => "448f071a7637fcefb2fccf76902dcf7d",
            MiniWatermark::Drifter => "1448dde4efdb57b07f5473f87c4fccd7",
            MiniWatermark::Opulence => "5364cc3900dc3615cb0c4b03c6221942",
            MiniWatermark::Shadowkeep => "2352f9d04dc842cfcdda77636335ded9",
            MiniWatermark::Undying => "e8fe681196baf74917fa3e6f125349b0",
            MiniWatermark::Dawn => "3ba38a2b9538bde2b45ec9313681d617",
            MiniWatermark::Worthy => "b12630659223b53634e9f97c0a0a8305",
            MiniWatermark::Arrivals => "4c25426263cacf963777cd4988340838",
            MiniWatermark::BeyondLight => "9e0f43538efe9f8d04546b4b0af6cc43",
            MiniWatermark::Hunt => "be3c0a95a8d1abc6e7c875d4294ba233",
            MiniWatermark::Chosen => "5ac4a1d48a5221993a41a5bb524eda1b",
            MiniWatermark::Splicer => "23968435c2095c0f8119d82ee222c672",
            MiniWatermark::Lost => "d92e077d544925c4f37e564158f8f76a",
            MiniWatermark::Anniversary => "671a19eca92ad9dcf39d4e9c92fcdf75",
            MiniWatermark::WitchQueen => "b973f89ecd631a3e3d294e98268f7134",
            MiniWatermark::Risen => "6e4fdb4800c34ccac313dd1598bd7589",
            MiniWatermark::Haunted => "ab075a3679d69f40b8c2a319635d60a9",
            MiniWatermark::Plunder => "a3923ae7d2376a1c4eb0f1f154da7565",
            MiniWatermark::Seraph => "e775dcb3d47e3d54e0e24fbdb64b5763",
            MiniWatermark::LightFall => "af00bdcd3e3b89e6e85c1f63ebc0b4e4",
            MiniWatermark::Defiance => "31445f1891ce9eb464ed1dcf28f43613",
            MiniWatermark::TheDeep => "6026e9d64e8c2b19f302dafb0286897b",
        };
        format!(
            "https://www.bungie.net/common/destiny2_content/icons/{}.png",
            buffer
        )
    }
}

//Expects String to be /common/destiny2_content/icons/ ... .png
//Comes from API / Rustgie
//Only for pregen
#[cfg(feature = "pre_gen")]
impl TryFrom<Option<String>> for MiniWatermark {
    type Error = String;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        //Red war exotics come back from api as None
        let value = match value {
            Some(x) => x,
            None => {
                return Ok(MiniWatermark::RedWar);
            }
        };
        if value.len() != 67
            || &value[0..=30] != "/common/destiny2_content/icons/"
            || &value[63..=66] != ".png"
        {
            return Err(format!("Invlaid watermark format: {}\n", value));
        }
        //Extracts the needed portion
        Ok(match &value[31..=62] {
            "ad7fdb049d430c1fac1d20cf39059702" => MiniWatermark::Dawning,
            "f80e39c767f309f0b2be625dae0e3744" => MiniWatermark::FOTL,
            "efdb35540cd169fa6e334995c2ce87b6" => MiniWatermark::GuardianGames,
            "04de56db6d59127239ed51e82d16c06c" => MiniWatermark::CrimsonDays,
            "52523b49e5965f6f33ab86710215c676" => MiniWatermark::Solstice,
            "fb50cd68a9850bd323872be4f6be115c" => MiniWatermark::RedWar,
            "2c024f088557ca6cceae1e8030c67169" => MiniWatermark::CurseOfOsiris,
            "ed6c4762c48bd132d538ced83c1699a6" => MiniWatermark::Warmind,
            "1b6c8b94cec61ea42edb1e2cb6b45a31" => MiniWatermark::Forsaken,
            "448f071a7637fcefb2fccf76902dcf7d" => MiniWatermark::Forge,
            "1448dde4efdb57b07f5473f87c4fccd7" => MiniWatermark::Drifter,
            "5364cc3900dc3615cb0c4b03c6221942" => MiniWatermark::Opulence,
            "2352f9d04dc842cfcdda77636335ded9" => MiniWatermark::Shadowkeep,
            "e8fe681196baf74917fa3e6f125349b0" => MiniWatermark::Undying,
            "3ba38a2b9538bde2b45ec9313681d617" => MiniWatermark::Dawn,
            "b12630659223b53634e9f97c0a0a8305" => MiniWatermark::Worthy,
            "4c25426263cacf963777cd4988340838" => MiniWatermark::Arrivals,
            "9e0f43538efe9f8d04546b4b0af6cc43" => MiniWatermark::BeyondLight,
            "be3c0a95a8d1abc6e7c875d4294ba233" => MiniWatermark::Hunt,
            "5ac4a1d48a5221993a41a5bb524eda1b" => MiniWatermark::Chosen,
            "23968435c2095c0f8119d82ee222c672" => MiniWatermark::Splicer,
            "d92e077d544925c4f37e564158f8f76a" => MiniWatermark::Lost,
            "671a19eca92ad9dcf39d4e9c92fcdf75" => MiniWatermark::Anniversary,
            "b973f89ecd631a3e3d294e98268f7134" => MiniWatermark::WitchQueen,
            "6e4fdb4800c34ccac313dd1598bd7589" => MiniWatermark::Risen,
            "ab075a3679d69f40b8c2a319635d60a9" => MiniWatermark::Haunted,
            "a3923ae7d2376a1c4eb0f1f154da7565" => MiniWatermark::Plunder,
            "e775dcb3d47e3d54e0e24fbdb64b5763" => MiniWatermark::Seraph,
            "af00bdcd3e3b89e6e85c1f63ebc0b4e4" => MiniWatermark::LightFall,
            "31445f1891ce9eb464ed1dcf28f43613" => MiniWatermark::Defiance,
            "6026e9d64e8c2b19f302dafb0286897b" => MiniWatermark::TheDeep,
            n => return Err(format!("Unknown watermark hash: {}\n", n)),
        })
    }
}

//Gives season number from enum
impl MiniWatermark {
    pub fn get_season(&self) -> Option<u8> {
        Some(match self {
            Self::RedWar => 1,
            Self::CurseOfOsiris => 2,
            Self::Warmind => 3,
            Self::Forsaken => 4,
            Self::Forge => 5,
            Self::Drifter => 6,
            Self::Opulence => 7,
            Self::Shadowkeep => 8,
            Self::Undying => 8,
            Self::Dawn => 9,
            Self::Worthy => 10,
            Self::Arrivals => 11,
            Self::BeyondLight => 12,
            Self::Hunt => 12,
            Self::Chosen => 13,
            Self::Splicer => 14,
            Self::Lost => 15,
            Self::Anniversary => 15,
            Self::WitchQueen => 16,
            Self::Risen => 16,
            Self::Haunted => 17,
            Self::Plunder => 18,
            Self::Seraph => 19,
            Self::LightFall => 20,
            Self::Defiance => 20,
            Self::TheDeep => 21,
            _ => return None,
        })
    }
}
