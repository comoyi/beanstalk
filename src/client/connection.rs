pub struct ConnectionInfo {
    pub(crate) addr: ConnectionAddr,
}

pub struct ConnectionAddr {
    pub host: String,
    pub port: u16,
}
