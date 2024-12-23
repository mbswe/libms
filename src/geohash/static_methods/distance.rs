use geohash::{decode};
use phper::{values::ZVal, Result, Error};
use haversine::{distance, Location, Units};

pub fn geohash_distance(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 2 {
        return Err(Error::boxed("geo_hash_distance expects exactly two arguments."));
    }

    let geohash1 = arguments[0].expect_z_str()?.to_str()?;
    let geohash2 = arguments[1].expect_z_str()?.to_str()?;
    let coord1 = decode(geohash1).map_err(|e| Error::boxed(e))?;
    let coord2 = decode(geohash2).map_err(|e| Error::boxed(e))?;
    let loc1 = Location { latitude: coord1.0.x, longitude: coord1.0.y };
    let loc2 = Location { latitude: coord2.0.x, longitude: coord2.0.y };
    let distance = distance(loc1, loc2, Units::Kilometers);

    Ok(ZVal::from(distance))
}