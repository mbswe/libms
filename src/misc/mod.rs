use phper::classes::{ClassEntity, Visibility};
use phper::modules::Module;

mod static_methods;

pub fn register_misc_class(module: &mut Module) {
    let mut misc_class = ClassEntity::new("MSMisc");

    misc_class.add_static_method(
        "toExcelColumn",
        Visibility::Public,
        crate::misc::static_methods::to_excel_column,
    );

    misc_class.add_static_method(
        "fromExcelColumn",
        Visibility::Public,
        crate::misc::static_methods::from_excel_column,
    );

    module.add_class(misc_class);
}