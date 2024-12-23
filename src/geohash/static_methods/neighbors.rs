use phper::{Error, Result};
use phper::values::ZVal;
use phper::arrays::ZArray;

/// Returns the 8 neighboring geohashes of a given geohash using the `geohash` crate.
pub fn geohash_neighbors(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 1 {
        return Err(Error::boxed("neighbors expects exactly one argument (a geohash)."));
    }

    // Get the geohash string
    let geohash = arguments[0]
        .as_z_str()
        .ok_or_else(|| Error::boxed("The argument must be a valid geohash string."))
        .and_then(|zstr| zstr.to_str().map_err(|_| Error::boxed("Failed to convert geohash to a valid UTF-8 string.")))?;

    // Compute neighbors using the geohash crate
    let neighbors = geohash::neighbors(geohash).map_err(|_| Error::boxed("Invalid geohash string."))?;

    // Create a PHP array to hold the neighbors
    let mut result = ZVal::from(ZArray::new());
    let array = result.as_mut_z_arr().unwrap();

    // Insert each neighbor into the PHP array
    array.insert("N", ZVal::from(neighbors.n));
    array.insert("NE", ZVal::from(neighbors.ne));
    array.insert("E", ZVal::from(neighbors.e));
    array.insert("SE", ZVal::from(neighbors.se));
    array.insert("S", ZVal::from(neighbors.s));
    array.insert("SW", ZVal::from(neighbors.sw));
    array.insert("W", ZVal::from(neighbors.w));
    array.insert("NW", ZVal::from(neighbors.nw));

    Ok(result)
}