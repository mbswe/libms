use geohash::{encode, Coord};
use phper::{values::ZVal, Result, Error};

pub fn geohash_encode(arguments: &mut [ZVal]) -> Result<ZVal> {
    let lat = arguments.get(0).and_then(|v| v.as_double()).ok_or_else(|| {
        Error::boxed("Latitude must be a valid floating point number.")
    })?;

    let lon = arguments.get(1).and_then(|v| v.as_double()).ok_or_else(|| {
        Error::boxed("Longitude must be a valid floating point number.")
    })?;

    // Check if len is provided, otherwise use default value of 12
    let default_len: i64 = 12;
    let len = arguments.get(2).and_then(|v| v.as_long()).unwrap_or(default_len);

    let c = Coord { x: lat, y: lon };
    let hash = encode(c, len as usize).map_err(|e| {
        Error::boxed(format!("Encoding error: {}", e))
    })?;

    Ok(ZVal::from(hash))
}