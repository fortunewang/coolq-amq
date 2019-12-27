use std::net::IpAddr;
use serde::Deserialize;
use failure::Fallible;
use lapin::uri::{self as amqpuri, AMQPUri};

#[inline]
fn default_host() -> IpAddr { IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)) }

#[inline]
fn default_port() -> u16 { 5672 }

#[inline]
fn default_vhost() -> String { String::from("/") }

#[inline]
fn default_username() -> String { String::from("guest") }

#[inline]
fn default_password() -> String { String::from("guest") }

#[inline]
fn default_connection_timeout() -> u64 { 10 }

#[derive(Deserialize)]
struct Options {
    #[serde(default = "default_host")]
    pub host: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_vhost")]
    pub vhost: String,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            vhost: default_vhost(),
            username: default_username(),
            password: default_password(),
            connection_timeout: default_connection_timeout(),
        }
    }
}

pub struct Config {
    pub uri: AMQPUri,
    pub connection_timeout: u64,
}

impl From<Options> for Config {
    fn from(options: Options) -> Self {
        let uri = AMQPUri {
            scheme: amqpuri::AMQPScheme::AMQP,
            authority: amqpuri::AMQPAuthority {
                userinfo: amqpuri::AMQPUserInfo {
                    username: options.username,
                    password: options.password,
                },
                host: options.host.to_string(),
                port: options.port,
            },
            vhost: options.vhost,
            query: amqpuri::AMQPQueryString::default(),
        };
        let connection_timeout = options.connection_timeout;
        Self { uri, connection_timeout }
    }
}

pub fn read_config(app_dir: &str) -> Fallible<Config> {
    use std::path::Path;

    let config_path = Path::new(app_dir).join("config.toml");

    if !config_path.exists() {
        return Ok(Config::from(Options::default()));
    }

    let options = std::fs::read_to_string(&config_path)?;
    let options: Options = toml::from_str(&options)?;

    return Ok(Config::from(options));
}
