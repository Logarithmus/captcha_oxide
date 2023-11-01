#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Socks4,
    Socks5,
}
