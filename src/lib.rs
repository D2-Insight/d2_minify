pub mod foundry;
pub mod icons;
pub mod slot;
pub mod stats;
pub mod watermark;

extern crate alloc;

use foundry::MiniFoundry;
use icons::MiniIcon;
use slot::MiniSlot;
use stats::MiniStat;
use wasm_bindgen::prelude::*;
use watermark::MiniWatermark;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[wasm_bindgen(js_name = "getWatermark")]
pub fn get_watermark(input: u8) -> Result<String, String> {
    match MiniWatermark::try_from(input) {
        Ok(x) => Ok(x.into()),
        Err(x) => Err(format!("{:?}", x)),
    }
}

#[wasm_bindgen(js_name = "getSeason")]
pub fn get_season(input: u8) -> Result<Option<u8>, String> {
    match MiniWatermark::try_from(input) {
        Ok(x) => Ok(x.get_season()),
        Err(x) => Err(format!("MiniSeason {} is invalid\n", x.number)),
    }
}

#[wasm_bindgen(js_name = "getStatHash")]
pub fn get_stat_hash(input: u8) -> Result<Option<u32>, String> {
    match MiniStat::try_from(input) {
        Ok(x) => Ok(x.into()),
        Err(x) => Err(format!("{:?}", x)),
    }
}

#[wasm_bindgen(js_name = "getSlotHash")]
pub fn get_slot_hash(input: u8) -> Result<u32, String> {
    match MiniSlot::try_from(input) {
        Ok(x) => Ok(x.into()),
        Err(x) => Err(format!("{:?}", x)),
    }
}

#[wasm_bindgen(js_name = "getFoundryUrl")]
pub fn get_foundry_url(input: u8) -> Result<Option<String>, String> {
    match MiniFoundry::try_from(input) {
        Ok(x) => Ok(x.into()),
        Err(x) => Err(format!("{:?}", x)),
    }
}

#[wasm_bindgen(js_name = "getIconUrl")]
pub fn get_icon_url(left_part: u64, right_part: u64) -> String {
    MiniIcon(left_part, right_part).into()
}
