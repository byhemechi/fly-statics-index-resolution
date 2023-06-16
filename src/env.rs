use std::net::SocketAddr;

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Env {
    pub host: SocketAddr,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            host: SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    }
}
