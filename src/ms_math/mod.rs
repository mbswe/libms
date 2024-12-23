use phper::{
    classes::{ClassEntity, Visibility},
    modules::Module,
};

mod static_methods;

pub fn register_ms_math_class(module: &mut Module) {
    let mut ms_math_class = ClassEntity::new("MSMath");

    ms_math_class.add_static_method(
        "clamp",
        Visibility::Public,
        static_methods::clamp,
    );

    module.add_class(ms_math_class);
}