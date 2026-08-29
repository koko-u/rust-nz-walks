use secrecy::ExposeSecret;
use secrecy::SecretString;

use crate::shared;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(long, env = "SERVER_HOST", default_value = "localhost", value_parser = parse_host)]
    host: url::Host,
    #[arg(long, env = "SERVER_PORT", default_value_t = 3000)]
    port: u16,

    #[arg(long, env = "DATABASE_HOST", default_value = "localhost", value_parser = parse_host)]
    database_host: url::Host,
    #[arg(long, env = "DATABASE_PORT", default_value_t = 5432)]
    database_port: u16,
    #[arg(long, env = "DATABASE_USER", default_value = "postgres")]
    database_user: String,
    #[arg(long, env = "DATABASE_PASSWORD", default_value = "password")]
    database_password: SecretString,
    #[arg(long, env = "DATABASE_NAME", default_value = "mydb")]
    database_name: String,
    #[arg(long, env = "DATABASE_SSLMODE", default_value_t = SslMode::Prefer)]
    database_sslmode: SslMode,
    #[arg(long, env = "MAX_CONNECTIONS", default_value = "10")]
    max_connections: shared::MaxConnections,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, derive_more::Display, derive_more::FromStr)]
enum SslMode {
    #[display("disable")]
    Disable,
    #[display("allow")]
    Allow,
    #[display("prefer")]
    Prefer,
    #[display("require")]
    Require,
    #[display("verify-ca")]
    VerifyCa,
    #[display("verify-full")]
    VerifyFull,
}

impl Config {
    pub fn new() -> Result<Self, clap::Error> {
        use clap::Parser;
        Self::try_parse()
    }

    pub fn addrs(&self) -> impl tokio::net::ToSocketAddrs {
        (self.host.to_string(), self.port)
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{dbname}?sslmode={sslmode}",
            user = self.database_user,
            password = self.database_password.expose_secret(),
            host = self.database_host,
            port = self.database_port,
            dbname = self.database_name,
            sslmode = self.database_sslmode
        )
    }

    pub fn max_connections(&self) -> shared::MaxConnections {
        self.max_connections
    }
}

fn parse_host(s: &str) -> Result<url::Host, url::ParseError> {
    url::Host::parse(s)
}
