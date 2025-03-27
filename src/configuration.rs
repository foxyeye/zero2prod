use config::{Config, File};
use secrecy::{ExposeSecret, SecretBox};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //let mut settings = config::Config::default();
    //settings.merge(config::File::with_name("configuration"))?;
    //settings.try_into()

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let base_configuration_file = base_path.join("configuration").join("base");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    let config_ = Config::builder()
        .add_source(File::with_name(
            base_configuration_file
                .to_str()
                .expect("Path contains invalid UTF-8 characters"),
        ))
        .add_source(File::with_name(
            base_path
                .join("configuration")
                .join(environment.as_str())
                .to_str()
                .expect("Path contains invalid UTF-8 characters"),
        ))
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()?;
    let config: Settings = config_.try_deserialize()?;
    Ok(config)
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )))
    }

    pub fn connection_string_without_db(&self) -> SecretBox<String> {
        SecretBox::new(Box::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )))
    }
}
