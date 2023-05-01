use num_enum::{FromPrimitive, IntoPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};
#[repr(u8)]
#[derive(
    Deserialize_repr, Serialize_repr, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive,
)]
pub enum MiniFoundry {
    #[default]
    Unknown = 0,
    Suros = 1,
    Veist = 2,
    Omolon = 3,
    Hakke = 4,
    BlackArmory = 5,
    Daito = 6,
    TexMechanica = 7,
    FieldForged = 8,
}

impl From<MiniFoundry> for Option<String> {
    fn from(value: MiniFoundry) -> Self {
        let buffer = match value {
            MiniFoundry::Suros => "e9fcd73e969a9295c3ab4ee5743893c2",
            MiniFoundry::Veist => "33679ff3935b6b925f007181f0959d84",
            MiniFoundry::Omolon => "36de329ebf19e58fa0aa90f6828edd57",
            MiniFoundry::Hakke => "8516a3a35697546fc0c8e8b4ab83aae6",
            MiniFoundry::BlackArmory => "d1f7db758d2da8756c5d302b13b156ab",
            MiniFoundry::Daito => "26c980259713e164aec34002b4c76dca",
            MiniFoundry::TexMechanica => "08d0631148d6e0bdc202c5ecab25f781",
            MiniFoundry::FieldForged => "2b08717bae25bd46dffb7efba66a2371",
            MiniFoundry::Unknown => return None,
        };
        Some(format!(
            "https://www.bungie.net/common/destiny2_content/icons/{}.png",
            buffer
        ))
    }
}

impl From<String> for MiniFoundry {
    fn from(value: String) -> Self {
        match value.as_str() {
            "e9fcd73e969a9295c3ab4ee5743893c2" => MiniFoundry::Suros,
            "33679ff3935b6b925f007181f0959d84" => MiniFoundry::Veist,
            "36de329ebf19e58fa0aa90f6828edd57" => MiniFoundry::Omolon,
            "8516a3a35697546fc0c8e8b4ab83aae6" => MiniFoundry::Hakke,
            "d1f7db758d2da8756c5d302b13b156ab" => MiniFoundry::BlackArmory,
            "26c980259713e164aec34002b4c76dca" => MiniFoundry::Daito,
            "08d0631148d6e0bdc202c5ecab25f781" => MiniFoundry::TexMechanica,
            "2b08717bae25bd46dffb7efba66a2371" => MiniFoundry::FieldForged,
            _ => MiniFoundry::Unknown,
        }
    }
}
