use confique::Config;

//struct for config connection to Postgres
#[derive(Config, Debug)]
pub struct PostgresConfig {
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) db: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) connect_timeout: u64,
}
