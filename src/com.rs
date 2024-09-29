use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProxyConfig {
    pub proxy_ip: String,
    pub proxy_port: String,
    pub redis_auth: String,
}

#[derive(Deserialize, Debug)]
pub struct SliceConfig {
    pub master: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub proxy: ProxyConfig,
    pub slice: Vec<SliceConfig>,
}
