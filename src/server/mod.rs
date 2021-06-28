use crate::{backends::Backend, error::Error, secure::tokenizer::Tokenizer, Result};
use clap::{crate_authors, crate_version, Clap};
use rocket::{data::Limits, Build, Config, Rocket};
use rocket_okapi::{
    routes_with_openapi,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use std::path::Path;

/// Catchers like 500, 501, 404, etc
mod catchers;
/// All the Routes/endpoints
mod controllers;
/// All required Guards
mod guards;

/// Server & App Configurations
mod config;
use self::config::Settings;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct CliOpts {
    #[clap(short = 'c', long, about = "loads the server configurations")]
    config: Option<String>,
}

/// Parse the settings from the command line arguments
// fn parse_settings_from_cli() -> Result<Settings, String> {
fn parse_settings_from_cli() -> Result<Settings> {
    // parse the cli options
    let cli_opts = CliOpts::parse();
    let cfg_file = &cli_opts.config.unwrap_or_default();
    if cfg_file.is_empty() {
        // No config file, so start
        // with default settings
        Ok(Settings::default())
    } else {
        // Config file passed in cli, check
        // to see if config file exists
        if Path::new(cfg_file).exists() {
            // load settings from the config file or return error
            // if error in loading the given config file
            Settings::from_file(&cfg_file)
        } else {
            // config file does not exist, quit app
            Err(Error::ConfigFileNotFound)
        }
    }
}

/// Initialise the Rocket Server app
pub async fn init_server() -> Result<Rocket<Build>> {
    let settings = parse_settings_from_cli()?;

    let app_settings = settings.app.unwrap_or_default();

    if app_settings.db_url.is_empty() {
        return Err(Error::EmptyDBUrl)
    }

    let token_expires = parse_duration::parse(&settings.server.jwt_token_expiry)?;
    let jwt_secret = settings.server.secret_key.to_owned();

    let limits = Limits::new()
        .limit("forms", settings.server.forms_limit.into())
        .limit("json", settings.server.json_limit.into());

    let rocket_cfg = Config::figment()
        .merge(("address", settings.server.host.to_string()))
        .merge(("port", settings.server.port as u16))
        .merge(("limits", limits))
        .merge(("secret_key", (settings.server.secret_key.as_str())))
        .merge(("keep_alive", settings.server.keep_alive as u32));

    // Configure SSL status for the api server
    let rocket_cfg = if let Some(ssl_cfg) = settings.ssl {
        if ssl_cfg.enabled {
            // ssl is enabled
            if ssl_cfg.pem_certificate.is_some() && ssl_cfg.pem_private_key.is_some() {
                // merge the certs & key into rocket config
                rocket_cfg
                    .merge(("tls.certs", ssl_cfg.pem_certificate))
                    .merge(("tls.key", ssl_cfg.pem_private_key))
            } else {
                // ssl certificate info not available
                return Err(Error::SslCertificateError);
            }
        } else {
            // ssl not enabled
            rocket_cfg
        }
    } else {
        // no ssl configuration
        rocket_cfg
    };

    // Configure the Rocket server with configured settings
    let app = rocket::custom(rocket_cfg);

    // Catchers
    let app = app.register(
        "/",
        rocket::catchers![
            catchers::bad_request,
            catchers::forbidden,
            catchers::not_authorized,
            catchers::not_found,
            catchers::unprocessed_entity,
            catchers::internal_server_error
        ],
    );

    // Add the user routes
    let app = app.mount(
        "/user",
        routes_with_openapi![
            controllers::user::authenticate_user,
            controllers::user::add_user,
            controllers::user::delete_user,
            controllers::user::change_user_password,
            controllers::user::get_all_users,
        ],
    );

    // Add the swagger doc
    let app = app.mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    );

    let app = app
        // Add server settings to state just in case
        .manage(jwt_secret)
        // add tokenizer info to the state
        .manage(Tokenizer::new(
            token_expires,
            Some(&settings.server.secret_key),
        ))
        // add the Backend to the state
        .manage(Backend::new(&app_settings.db_url)?);

    // Return the configured Rocket App
    Ok(app)
}
