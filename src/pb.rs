macro_rules! include_proto_under_mod {
    ($mod:ident, $proto:expr) => {
        pub mod $mod {
            tonic::include_proto!($proto);
        }
    };
}

include_proto_under_mod!(languages, "languages");
include_proto_under_mod!(runs, "runs");
