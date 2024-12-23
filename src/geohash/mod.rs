use phper::{
    classes::{ClassEntity, Visibility},
    modules::Module,
};

pub mod static_methods;

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

    geohash_class.add_static_method(
        "neighbors",
        Visibility::Public,
        static_methods::geohash_neighbors,
    );

    geohash_class.add_static_method(
        "neighbor",
        Visibility::Public,
        static_methods::geohash_neighbor,
    );

    module.add_class(geohash_class);
}