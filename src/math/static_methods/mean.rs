use phper::{Error, Result};
use phper::values::ZVal;

pub fn mean(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 1 {
        return Err(Error::boxed("mean expects exactly one argument (an array)."));
    }

    let arr = arguments[0]
        .expect_mut_z_arr()?;

    let arr_len = arr.len();
    if arr_len == 0 {
        return Err(Error::boxed("Cannot compute mean of an empty array."));
    }

    let mut sum = 0.0;
    for (_, val) in arr.iter() {
        let numeric_val = val
            .as_double()
            .or_else(|| val.as_long().map(|i| i as f64))
            .ok_or_else(|| Error::boxed("All elements must be numeric."))?;

        sum += numeric_val;
    }

    let mean_value = sum / (arr_len as f64);
    Ok(ZVal::from(mean_value))
}