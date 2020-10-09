use crate::commands::{admin::*, meta::*};
use clap::{app_from_crate, Arg, ArgSettings, ValueHint};
use log::{debug, error, info, log, log_enabled, trace, warn, Level, LevelFilter};
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config as LogConfig, Logger, Root as LogRoot},
    encode::pattern::PatternEncoder,
};
use serde::{Deserialize, Serialize};
use serenity::{
    async_trait, client::bridge::gateway::ShardManager, framework::standard::macros::group,
    framework::StandardFramework, http::Http as DiscordHttp, model::id::UserId, prelude as discord,
};
use sqlx::PgPool;
use std::{clone::Clone, collections::HashSet, io::BufReader, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    pub prefixes: Vec<String>,
    pub token: String,
    pub db_conn: String,
    pub owners: HashSet<UserId>,
}

impl discord::TypeMapKey for Config {
    type Value = Config;
}

struct ShardManagerContainer;
impl discord::TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(quit, ping)]
struct General;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = app_from_crate!()
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets config file location")
                .takes_value(true)
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("verbose")
                .setting(ArgSettings::MultipleOccurrences)
                .max_values(3)
                .takes_value(false)
                .short('v')
                .long("--verbose")
                .about("Increase verbosity. You should never need more than one.")
                .long_about("Increases verbosity. Due to the extreme amount of data levels two and three provide, you should never use them unless you are debugging.")
        )
        .get_matches();
    let config_file: PathBuf = match matches.value_of_os("config") {
        Some(path) => path.into(),
        None => std::env::current_dir()?.join("config.yml"),
    };
    // d l t - m n
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} {d} {t} {M} - {m} {n}")))
        .build();
    let log_config = LogConfig::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(LogRoot::builder().appender("stdout").build(
            match matches.occurrences_of("verbose") {
                0 => LevelFilter::Warn,
                1 => LevelFilter::Info,
                2 => LevelFilter::Debug,
                3 => LevelFilter::Trace,
                x => panic!(
                    "Somehow got more verbosity arguments than the CLI should have allowed: {}",
                    x
                ),
            },
        ))?;
    let _handle = log4rs::init_config(log_config)?;
    log_expensive!(Level::Trace, "test?");
    let config_file = std::fs::File::open(config_file)?;
    let config: Config = serde_yaml::from_reader(BufReader::new(config_file))?;
    let pool = PgPool::connect(&config.db_conn).await?;

    let discord_http = DiscordHttp::new_with_token(&config.token);
    let (config, _bot_id) = {
        let info = discord_http.get_current_application_info().await?;
        let mut owners = HashSet::from(config.owners);
        owners.insert(info.owner.id);
        (
            Config {
                prefixes: config.prefixes,
                token: config.token,
                db_conn: config.db_conn,
                owners,
            },
            info.id,
        )
    };
    // let config = Config {
    //     prefixes: config.prefixes,
    //     token: config.token,
    //     db_conn: config.db_conn,
    //     owners,
    // };
    let conf_cl = config.clone();
    let framework = StandardFramework::new()
        .configure(move |c| c.owners(conf_cl.owners).prefixes(conf_cl.prefixes))
        .group(&GENERAL_GROUP);

    let mut client = discord::Client::new(&config.token)
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(config);
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    client.start().await?;

    Ok(())
}

#[macro_use]
mod macros;
mod commands;
mod models;
