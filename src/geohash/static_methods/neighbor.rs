use geohash::{Direction, neighbor};
use phper::{Error, Result};
use phper::values::ZVal;

/// Returns the neighboring geohash in a specified direction using the `geohash` crate.
pub fn geohash_neighbor(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 2 {
        return Err(Error::boxed("neighbor expects exactly two arguments (a geohash and a direction)."));
    }

    // Get the geohash string
    let geohash = arguments[0]
        .as_z_str()
        .ok_or_else(|| Error::boxed("The first argument must be a valid geohash string."))
        .and_then(|zstr| zstr.to_str().map_err(|_| Error::boxed("Failed to convert geohash to a valid UTF-8 string.")))?;

    // Get the direction string
    let direction = arguments[1]
        .as_z_str()
        .ok_or_else(|| Error::boxed("The second argument must be a valid direction string."))
        .and_then(|zstr| zstr.to_str().map_err(|_| Error::boxed("Failed to convert direction to a valid UTF-8 string.")))?;

    // Convert the direction string to a `Direction` enum
    let direction_enum = match direction {
        "N" => Direction::N,
        "NE" => Direction::NE,
        "E" => Direction::E,
        "SE" => Direction::SE,
        "S" => Direction::S,
        "SW" => Direction::SW,
        "W" => Direction::W,
        "NW" => Direction::NW,
        _ => return Err(Error::boxed("Invalid direction. Valid directions are N, NE, E, SE, S, SW, W, NW.")),
    };

    // Compute the neighbor using the geohash crate
    let neighbor = neighbor(geohash, direction_enum)
        .map_err(|_| Error::boxed("Invalid geohash string or direction."))?;

    // Return the neighboring geohash
    Ok(ZVal::from(neighbor))
}