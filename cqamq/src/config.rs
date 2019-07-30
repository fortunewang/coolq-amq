use failure::{Fallible, err_msg};
use lapin::uri::AMQPUri;

pub struct Config {
    pub uri: AMQPUri,
}

pub fn read_config(app_dir: &str) -> Fallible<Config> {
    use std::path::Path;

    let mut config = Config {
        uri: AMQPUri::default(),
    };

    let config_path = Path::new(app_dir).join("config.toml");

    if !config_path.exists() {
        return Ok(config);
    }

    let options = std::fs::read_to_string(&config_path)?;
    let options: toml::Value = toml::from_str(&options)?;

    if let Some(host) = options.get("host") {
        config.uri.authority.host = host.as_str()
            .ok_or(err_msg("config 'host' is not a string"))?
            .to_owned();
    }
    if let Some(port) = options.get("port") {
        config.uri.authority.port = port.as_integer()
            .ok_or(err_msg("config 'host' is not a integer"))?
            as u16;
    }
    if let Some(username) = options.get("username") {
        config.uri.authority.userinfo.username = username.as_str()
            .ok_or(err_msg("config 'username' is not a string"))?
            .to_owned();
    }
    if let Some(password) = options.get("password") {
        config.uri.authority.userinfo.password = password.as_str()
            .ok_or(err_msg("config 'password' is not a string"))?
            .to_owned();
    }
    if let Some(vhost) = options.get("vhost") {
        config.uri.vhost = vhost.as_str()
            .ok_or(err_msg("config 'vhost' is not a string"))?
            .to_owned();
    }
    return Ok(config);
}
