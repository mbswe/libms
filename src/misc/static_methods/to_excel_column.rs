use phper::{Error, Result};
use phper::values::ZVal;

pub fn to_excel_column(arguments: &mut [ZVal]) -> Result<ZVal> {
    if arguments.len() != 1 {
        return Err(Error::boxed("toExcelColumn expects exactly one argument."));
    }

    let value = arguments.get(0)
        .and_then(|v| v.as_long())
        .ok_or_else(|| Error::boxed("Value must be a valid integer."))?;

    let mut result = String::new();
    let mut value = value as i64;

    while value > 0 {
        let remainder = (value - 1) % 26;
        result.insert(0, (65 + remainder) as u8 as char);
        value = (value - remainder) / 26;
    }

    Ok(ZVal::from(result))
}