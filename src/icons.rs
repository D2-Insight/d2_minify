///This is used to produce links for icons
///Example: "https://www.bungie.net/common/destiny2_content/icons/0f584e8a13b2cc4cb60379b1777362e5.jpg"
///Doing 2 u64s instead of a u128 for wasm compatability
#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq, Default)]
pub struct MiniIcon {
    icon_array: [u64; 2],
}

//Needs to be in format of /common/destiny2_content/icons/ ... .jpg
//This format is given from api/rustgie.
impl From<String> for MiniIcon {
    fn from(url: String) -> Self {
        match url.len() != 67
            || &url[0..=30] != "/common/destiny2_content/icons/"
            || &url[63..=66] != ".jpg"
        {
            true => return MiniIcon::default(),
            false => (),
        }
        let hex = &url[31..=62];
        //Should be safe after confirming the rest? Unless bungie changes from hex :/
        //Only will be used during pregen anyways so o7
        MiniIcon {
            icon_array: [
                u64::from_str_radix(&hex[0..=15], 16).unwrap(),
                u64::from_str_radix(&hex[16..=31], 16).unwrap(),
            ],
        }
    }
}

impl From<MiniIcon> for Option<String> {
    fn from(val: MiniIcon) -> Self {
        if val == MiniIcon::default() {
            return None;
        }
        Some(format!(
            "https://www.bungie.net/common/destiny2_content/icons/{:016x}{:016x}.jpg",
            val.icon_array[0], val.icon_array[1]
        ))
    }
}
