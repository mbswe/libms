use phper::{
    classes::{ClassEntity, Visibility},
    modules::Module,
};

mod static_methods;

pub fn register_geohash_class(module: &mut Module) {
    let mut geohash_class = ClassEntity::new("MSGeoHash");

    geohash_class.add_static_method(
        "encode",
        Visibility::Public,
        static_methods::geohash_encode,
    );

    geohash_class.add_static_method(
        "decode",
        Visibility::Public,
        static_methods::geohash_decode,
    );

    geohash_class.add_static_method(
        "distance",
        Visibility::Public,
        static_methods::geohash_distance,
    );

    module.add_class(geohash_class);
}