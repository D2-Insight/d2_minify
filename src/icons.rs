///This is used to produce links for icons
///Example: "https://www.bungie.net/common/destiny2_content/icons/0f584e8a13b2cc4cb60379b1777362e5.jpg"
///Doing 2 u64s instead of a u128 for wasm compatability
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MiniIcon(pub u64, pub u64);

impl From<MiniIcon> for String {
    fn from(val: MiniIcon) -> Self {
        format!(
            "https://www.bungie.net/common/destiny2_content/icons/{:016x}{:016x}.jpg",
            val.0, val.1
        )
    }
}

//Needs to be in format of /common/destiny2_content/icons/ ... .jpg
//This format is given from api/rustgie.
#[cfg(feature = "pre_gen")]
impl TryFrom<String> for MiniIcon {
    type Error = String;
    fn try_from(url: String) -> Result<Self, Self::Error> {
        if url.len() != 67
            || &url[0..=30] != "/common/destiny2_content/icons/"
            || &url[63..=66] != ".jpg"
        {
            return Err(format!("Invalid Icon format: {}\n", url));
        }

        Ok(MiniIcon(
            u64::from_str_radix(&url[31..=46], 16).unwrap(),
            u64::from_str_radix(&url[47..=62], 16).unwrap(),
        ))
    }
}
