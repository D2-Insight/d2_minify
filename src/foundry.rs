use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(
    feature = "serde",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
pub enum MiniFoundry {
    NoFoundry = 0,
    Suros = 1,
    Veist = 2,
    Omolon = 3,
    Hakke = 4,
    BlackArmory = 5,
    Daito = 6,
    TexMechanica = 7,
    FieldForged = 8,
    Cassoid = 9,
}

impl From<MiniFoundry> for Option<String> {
    fn from(value: MiniFoundry) -> Self {
        let buffer = match value {
            MiniFoundry::Suros => "56920a380f4f328b65cd4940d57ad86d",
            MiniFoundry::Veist => "9cb266173ab772fa689b72bd365c28df",
            MiniFoundry::Omolon => "d7596444c338aa5339234cd017810946",
            MiniFoundry::Hakke => "b5db41f5d5cc0b8a9825da3592312593",
            MiniFoundry::BlackArmory => "0f06c268d1f1080cea32a0de86f18c00",
            MiniFoundry::Daito => "de7406450cbc1f0cd4a5c31a2b321a51",
            MiniFoundry::TexMechanica => "ed8c4feb3dd97c8685e102cf565f135c",
            MiniFoundry::FieldForged => "cdc43a4cee2c550fa340516e1fd00d15",
            MiniFoundry::Cassoid => "85600f3ad441a2511b00c2af1c2003ce",
            MiniFoundry::NoFoundry => return None,
        };
        Some(format!(
            "https://www.bungie.net/common/destiny2_content/icons/{}.png",
            buffer
        ))
    }
}

#[cfg(feature = "pre_gen")]
impl TryFrom<Option<String>> for MiniFoundry {
    type Error = String;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        let value = match value {
            Some(x) => x,
            None => {
                return Ok(MiniFoundry::NoFoundry);
            }
        };

        if value.len() != 67
            || &value[0..=30] != "/common/destiny2_content/icons/"
            || &value[63..=66] != ".png"
        {
            return Err(format!("Invalid Foundry format: {}\n", value));
        }

        Ok(match &value[31..=62] {
            "56920a380f4f328b65cd4940d57ad86d" => MiniFoundry::Suros,
            "9cb266173ab772fa689b72bd365c28df" => MiniFoundry::Veist,
            "d7596444c338aa5339234cd017810946" => MiniFoundry::Omolon,
            "b5db41f5d5cc0b8a9825da3592312593" => MiniFoundry::Hakke,
            "0f06c268d1f1080cea32a0de86f18c00" => MiniFoundry::BlackArmory,
            "de7406450cbc1f0cd4a5c31a2b321a51" => MiniFoundry::Daito,
            "ed8c4feb3dd97c8685e102cf565f135c" => MiniFoundry::TexMechanica,
            "cdc43a4cee2c550fa340516e1fd00d15" => MiniFoundry::FieldForged,
            "85600f3ad441a2511b00c2af1c2003ce" => MiniFoundry::Cassoid,
            n => return Err(format!("Foundry unknown: {}\n", n)),
        })
    }
}
