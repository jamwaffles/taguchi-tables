mod utils;

use log::Level;
use oars::{
    constructors::BoseChecked,
    oa::{OAConstructor, OA},
    OarsError,
};
use wasm_bindgen::prelude::*;

/// Errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to generate the underlying orthogonal array: {}", 0)]
    GenError(#[from] OarsError),
}

pub fn create(parameter_count: usize, variations_per_param: usize) -> Result<OA<usize>, Error> {
    let config = BoseChecked {
        prime_base: parameter_count,
        dimensions: variations_per_param,
    };

    Ok(config.verify()?.gen()?)
}

#[derive(Debug, Default, Copy, Clone, serde::Deserialize)]
pub struct Config {
    something: u8,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize)]
pub struct Taguchi {
    foo: u32,
}

#[wasm_bindgen]
pub fn taguchi(config: JsValue) -> JsValue {
    console_log::init_with_level(Level::Debug).ok();

    let config: Config = serde_wasm_bindgen::from_value(config).expect("Bad config");

    log::debug!("Here agai");

    dbg!(config);

    serde_wasm_bindgen::to_value(&Taguchi::default()).expect("Bad output")
}

#[cfg(test)]
mod tests {
    use super::*;
    use oars::{
        constructors::{Bose, BoseChecked},
        oa::OAConstructor,
    };

    #[test]
    fn p2_l2() {
        let table = create(2, 2).expect("Gen");

        dbg!(table);
    }

    #[test]
    fn p3_l2() {
        let table = create(2, 3).expect("Gen");

        dbg!(table);
    }

    #[test]
    fn p3_l3() {
        let table = create(3, 3).expect("Gen");

        dbg!(table);
    }

    #[test]
    fn p2_l3() {
        let table = create(3, 2).expect("Gen");

        dbg!(table);
    }
}
