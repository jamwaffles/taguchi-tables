use oars::{
    constructors::BoseChecked,
    oa::{OAConstructor, OAResult, OA},
    OarsError,
};

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
