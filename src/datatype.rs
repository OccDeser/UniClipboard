
pub struct RemoteClipboard {
    pub host: String,
    pub port: u16,
}

pub struct LocalClipboard {
    pub port: u16,
    pub password: String,
    pub peers: Vec<RemoteClipboard>
}