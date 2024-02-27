use std::env;

pub mod proto {
    #![allow(clippy::nursery, clippy::pedantic)]
    tonic::include_proto!("streamingcalc");

    #[allow(dead_code)]
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("streamingcalc_descriptor");
}

fn get_env_var(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_server(with_scheme: bool) -> String {
    let scheme = get_env_var("SRV_SCHEME", "http");
    let host = get_env_var("SRV_HOST", "[::1]");
    let port = get_env_var("SRV_PORT", "50051");

    format!(
        "{}{host}:{port}",
        Some(scheme)
            .filter(|_| with_scheme)
            .map_or(String::new(), |s| s + "://")
    )
}
