use phper::Error;
use phper::values::ZVal;

pub fn clamp(arguments: &mut [ZVal]) -> Result<ZVal, Error> {
    if arguments.len() != 3 {
        return Err(Error::boxed("clamp expects exactly three arguments."));
    }

    let value = arguments.get(0).and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64))).ok_or_else(|| {
        Error::boxed("Value must be a valid floating point number or integer.")
    })?;

    let min = arguments.get(1).and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64))).ok_or_else(|| {
        Error::boxed("Min must be a valid floating point number or integer.")
    })?;

    let max = arguments.get(2).and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64))).ok_or_else(|| {
        Error::boxed("Max must be a valid floating point number or integer.")
    })?;

    Ok(ZVal::from(value.max(min).min(max)))
}