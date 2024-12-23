use phper::{
    modules::Module,
    php_get_module,
};

pub mod math;
pub mod geohash;
pub mod misc;

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.on_module_init(|| {});
    module.on_module_shutdown(|| {});
    module.on_request_init(|| {});
    module.on_request_shutdown(|| {});

    math::register_math_class(&mut module);
    geohash::register_geohash_class(&mut module);
    misc::register_misc_class(&mut module);

    module
}