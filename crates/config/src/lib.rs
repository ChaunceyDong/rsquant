use std::{fs, path};

use serde::{Deserialize, Serialize};

use quant_core::{Error, Result};

pub struct ConfigBuilder;

impl ConfigBuilder {
    fn read_config_file(path: path::PathBuf) -> Result<String> {
        if !path.exists() {
            let mut curr_config_path = std::env::current_dir().unwrap();
            curr_config_path.push("quant.toml");
            if !curr_config_path.exists() {
                let config_dir = path.parent().ok_or(Error::Custom(
                    "Failed to get config file parent path.".to_owned(),
                ))?;
                std::fs::create_dir_all(config_dir)?;
                fs::File::create(path)?;

                Err(Error::Custom("Please fill in the config file.".to_owned()))
            } else {
                fs::read_to_string(curr_config_path).map_err(Error::from)
            }
        } else {
            fs::read_to_string(path).map_err(Error::from)
        }
    }

    fn parse_config(config: String) -> Result<QuantConfig> {
        serde_json::from_str::<QuantConfig>(&config).map_err(Error::from)
    }

    pub fn build(path: path::PathBuf) -> Result<QuantConfig> {
        let config_content = ConfigBuilder::read_config_file(path)?;
        ConfigBuilder::parse_config(config_content)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QuantConfig {
    pub api_credentials: CredentialsConfig,
    pub email: EmailConfig,
    pub network: NetworkConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
    pub market: MarketConfig,
    pub strategy: StrategyConfig,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CredentialsConfig {
    Binance(BinanCredentialsConfig),
    #[default]
    Okx,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BinanCredentialsConfig {
    pub signature_type: String,
    pub api_key: String,
    pub api_secret: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub from_email: String,
    pub to_emails: Vec<String>,
    pub from_passwd: String,
    pub smtp_addr: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub proxy: Option<ProxyConfig>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub https_proxy: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub log_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DatabaseConfig {
    Postgresql(PostgresqlConfig),
    Sqlite(SqliteConfig),
    #[default]
    None,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostgresqlConfig {
    pub pg_addr: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub db_path: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MarketData {
    #[default]
    Kline,
    Ticker,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MarketConfig {
    pub data: MarketData,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StrategySignal {
    #[default]
    Macd,
    Rsi,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub signal: StrategySignal,
}
