use geohash::{decode};
use phper::{values::ZVal, arrays::ZArray, Result, Error};

pub fn geohash_decode(arguments: &mut [ZVal]) -> Result<ZVal> {
    let hash_val = arguments[0].expect_z_str().map_err(|_| {
        Error::boxed("Geohash must be a valid string.")
    })?;
    let hash_str = hash_val.to_str()?;
    match decode(hash_str) {
        Ok((coord, _error_lat, _error_lon)) => {
            let mut result = ZArray::new();
            result.insert("lat", coord.y);
            result.insert("lon", coord.x);
            Ok(ZVal::from(result))
        },
        Err(e) => {
            let err_msg = format!("Error decoding geohash: {}", e);
            Err(Error::boxed(err_msg))
        }
    }
}