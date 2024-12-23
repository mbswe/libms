use phper::{Error, Result};
use phper::values::ZVal;

pub fn remap_range(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 5 {
        return Err(Error::boxed(
            "remapRange expects exactly five arguments: (value, oldMin, oldMax, newMin, newMax).",
        ));
    }

    fn as_f64(arg: Option<&ZVal>, name: &str) -> Result<f64> {
        arg.and_then(|v| v.as_double().or_else(|| v.as_long().map(|i| i as f64)))
            .ok_or_else(|| Error::boxed(format!("{name} must be a valid float or int.")))
    }

    let value   = as_f64(arguments.get(0), "value")?;
    let old_min = as_f64(arguments.get(1), "oldMin")?;
    let old_max = as_f64(arguments.get(2), "oldMax")?;
    let new_min = as_f64(arguments.get(3), "newMin")?;
    let new_max = as_f64(arguments.get(4), "newMax")?;

    if (old_max - old_min).abs() < f64::EPSILON {
        return Err(Error::boxed(
            "Cannot remap when oldMin == oldMax. The input range must be > 0.",
        ));
    }

    let mapped_value = new_min + ((value - old_min) / (old_max - old_min)) * (new_max - new_min);

    Ok(ZVal::from(mapped_value))
}