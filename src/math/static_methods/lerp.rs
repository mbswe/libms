use phper::{Error, Result};
use phper::values::ZVal;

pub fn lerp(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 3 {
        return Err(Error::boxed("lerp expects exactly three arguments: (start, end, t)."));
    }

    let start = arguments.get(0)
        .and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64)))
        .ok_or_else(|| Error::boxed("start must be a valid float or int."))?;

    let end = arguments.get(1)
        .and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64)))
        .ok_or_else(|| Error::boxed("end must be a valid float or int."))?;

    let t = arguments.get(2)
        .and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64)))
        .ok_or_else(|| Error::boxed("t must be a valid float or int."))?;

    let result = start + t * (end - start);

    Ok(ZVal::from(result))
}