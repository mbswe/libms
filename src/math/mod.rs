use phper::{
    classes::{ClassEntity, Visibility},
    modules::Module,
};

mod static_methods;

pub fn register_math_class(module: &mut Module) {
    let mut math_class = ClassEntity::new("MSMath");

    math_class.add_static_method(
        "clamp",
        Visibility::Public,
        static_methods::clamp,
    );

    math_class.add_static_method(
        "lerp",
        Visibility::Public,
        static_methods::lerp,
    );

    math_class.add_static_method(
        "inverseLerp",
        Visibility::Public,
        static_methods::inverse_lerp,
    );

    math_class.add_static_method(
        "remapRange",
        Visibility::Public,
        static_methods::remap_range,
    );

    math_class.add_static_method(
        "mean",
        Visibility::Public,
        static_methods::mean,
    );

    module.add_class(math_class);
}