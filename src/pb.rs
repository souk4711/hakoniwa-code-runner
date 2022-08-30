pub mod languages {
    tonic::include_proto!("languages");
}

pub mod runs {
    tonic::include_proto!("runs");

    impl From<std::time::Duration> for Duration {
        fn from(d: std::time::Duration) -> Self {
            Self {
                seconds: d.as_secs(),
                nanos: d.subsec_nanos(),
            }
        }
    }
}
