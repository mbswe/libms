use phper::{Error, Result};
use phper::values::ZVal;

pub fn from_excel_column(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 1 {
        return Err(Error::boxed("from_excel_column expects exactly one argument (a column string)."));
    }

    // 1. Get the ZStr from the argument
    let z_str = arguments[0].expect_z_str().map_err(|_| {
        Error::boxed("The argument must be a valid string.")
    })?;

    // 2. Convert the ZStr to a &str
    let col_str = z_str.to_str().map_err(|_| {
        Error::boxed("Failed to convert to a valid UTF-8 string.")
    })?;

    // 3. Iterate and compute the result (use i64)
    let mut result: i64 = 0;
    for (i, c) in col_str.chars().rev().enumerate() {
        if !c.is_ascii_uppercase() {
            return Err(Error::boxed("The column string must contain only uppercase letters (A-Z)."));
        }
        // 'A' as i64 = 65, so subtract 64 to get range [1..26]
        result += (c as i64 - 'A' as i64 + 1) * 26_i64.pow(i as u32);
    }

    // 4. Return the result as a ZVal integer
    Ok(ZVal::from(result))
}