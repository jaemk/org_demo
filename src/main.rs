#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
#[macro_use] extern crate rouille;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate env_logger;
extern crate chrono;
extern crate migrant_lib;
extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;

#[macro_use] mod macros;
mod errors;
mod service;
mod models;

use std::env;
use clap::{App, Arg, SubCommand};

use errors::*;


static APPNAME: &'static str = "OrgDemo";


// ---------------
// Traits for constructing `rouille::Response`s from other types
// ---------------

pub trait ToHtmlResponse {
    fn to_html_resp(&self) -> rouille::Response;
}

pub trait ToTextResponse {
    fn to_text_resp(&self) -> rouille::Response;
}

pub trait ToJsonResponse {
    fn to_json_resp(&self) -> Result<rouille::Response>;
}


impl ToHtmlResponse for String {
    fn to_html_resp(&self) -> rouille::Response {
        rouille::Response::html(self.as_str())
    }
}
impl ToTextResponse for String {
    fn to_text_resp(&self) -> rouille::Response {
        rouille::Response::text(self.as_str())
    }
}

impl ToJsonResponse for serde_json::Value {
    fn to_json_resp(&self) -> Result<rouille::Response> {
        let s = serde_json::to_string(self)?;
        let resp = rouille::Response::from_data("application/json", s.as_bytes());
        Ok(resp)
    }
}


/// Trait for parsing `json` from `rouille::Request` bodies into some type `T`
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Deserialize)]
/// struct PostData {
///     name: String,
///     age: u32,
/// }
///```
///
/// For a request with a body containing `json`
///
/// ```rust,ignore
/// let post_data = request.parse_json_body::<PostData>()?;
/// println!("{}", post_data.name);
/// ```
pub trait FromRequestBody {
    fn parse_json_body<T: serde::de::DeserializeOwned>(&self) -> Result<T>;
}

impl FromRequestBody for rouille::Request {
    fn parse_json_body<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        use std::io::Read;
        let mut body = self.data().expect("Can't read request body twice");
        let mut s = String::new();
        body.read_to_string(&mut s)?;
        let data = serde_json::from_str::<T>(&s)
            .map_err(|_| format_err!(ErrorKind::BadRequest, "malformed data"))?;
        Ok(data)
    }
}


/// Migration function to insert some sample data
fn migration_add_sample_data(config: migrant_lib::ConnConfig) -> std::result::Result<(), Box<std::error::Error>> {
    let org_names = ["James Inc", "Bean Group", "Cat Collective", "Dog Dancers"];
    let emails = [
        ("james@kominick.com", vec![0, 1, 2, 3]),
        ("bean@burrito.org", vec![1, 3]),
        ("cheese@pasta.io", vec![1, 2]),
    ];
    let linode_names = [
        ("charlie", 0),
        ("mac", 1),
        ("frank", 2),
        ("dennis", 3),
        ("dee", 2),
    ];

    let db_path = config.database_path()?;
    let mut conn = rusqlite::Connection::open(&db_path)?;
    let trans = conn.transaction()?;

    {
        let stmt = "insert into org (name) values (?)";
        let mut stmt = trans.prepare(&stmt)?;
        let org_ids = org_names.iter().map(|name| {
            Ok(stmt.insert(&[name])?)
        }).collect::<Result<Vec<i64>>>()?;

        let stmt = "insert into user (email) values (?)";
        let mut stmt = trans.prepare(&stmt)?;
        let user_ids = emails.iter().map(|&(email, _)| {
            Ok(stmt.insert(&[&email])?)
        }).collect::<Result<Vec<i64>>>()?;

        let stmt = "insert into user_org (user, org) values (?, ?)";
        let mut stmt = trans.prepare(&stmt)?;
        for (user_id, link) in user_ids.iter().zip(emails.iter()) {
            let org_indices = &link.1;
            for ind in org_indices {
                let org_id = org_ids[*ind];
                stmt.insert(&[user_id, &org_id])?;
            }
        }

        let stmt = "insert into linode (name, org) values (?, ?)";
        let mut stmt = trans.prepare(&stmt)?;
        for &(name, org_ind) in linode_names.iter() {
            let org_id = org_ids[org_ind];
            stmt.insert(&[&name, &org_id])?;
        }
    }
    trans.commit()?;
    Ok(())
}

fn migration_empty(_: migrant_lib::ConnConfig) -> std::result::Result<(), Box<std::error::Error>> {
    Ok(())
}


/// Build a migrant database configuration
pub fn migrant_config() -> Result<migrant_lib::Config> {
    let dir = env::current_dir()?;
    let db_path = dir.join("db/org_demo");
    let migration_dir = dir.join("migrations");
    let settings = migrant_lib::Settings::configure_sqlite()
        .database_path(&db_path)?
        .migration_location(&migration_dir)?
        .build()?;
    let mut config = migrant_lib::Config::with_settings(&settings);
    config.use_migrations(&[
        migrant_lib::FileMigration::with_tag("init")?
            .up("migrations/init/up.sql")?
            .up("migrations/init/up.sql")?
            .boxed(),
        migrant_lib::FnMigration::with_tag("populate")?
            .up(migration_add_sample_data)
            .down(migration_empty)
            .boxed(),
    ])?;
    Ok(config)
}


fn run() -> Result<()> {
    let matches = App::new(APPNAME)
        .version(crate_version!())
        .about("OrgDemo Sever")
        .subcommand(SubCommand::with_name("database")
            .about("Database functions")
            .subcommand(SubCommand::with_name("migrate")
                .about("Apply any available un-applied migrations"))
            .subcommand(SubCommand::with_name("shell")
                .about("Open a database shell")))
        .subcommand(SubCommand::with_name("serve")
            .about("Initialize Server")
            .arg(Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("3002")
                .help("Port to listen on."))
            .arg(Arg::with_name("public")
                .long("public")
                .help("Serve on '0.0.0.0' instead of 'localhost'"))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Output debug logging info. Shortcut for setting env-var LOG=debug")))
        .get_matches();

    match matches.subcommand() {
        ("serve", Some(serve_matches)) => {
            env::set_var("LOG", "info");
            if serve_matches.is_present("debug") { env::set_var("LOG", "debug"); }
            let port = serve_matches.value_of("port")
                .expect("default port should be set by clap")
                .parse::<u16>()
                .chain_err(|| "`--port` expects an integer")?;
            let host = if serve_matches.is_present("public") { "0.0.0.0" } else { "localhost" };
            service::start(&host, port)?;
        }
        ("database", Some(db_matches)) => {
            let config = migrant_config()?;
            config.setup()?;
            let config = config.reload()?;

            match db_matches.subcommand() {
                ("migrate", _) => {
                    println!("Applying migrations...");
                    let res = migrant_lib::Migrator::with_config(&config)
                        .all(true)
                        .show_output(true)
                        .apply();
                    match res {
                        Err(ref e) if e.is_migration_complete() => (),
                        res => res?,
                    }
                    let config = config.reload()?;
                    migrant_lib::list(&config)?;
                }
                ("shell", _) => {
                    migrant_lib::shell(&config)?;
                }
                _ => {
                    eprintln!("{}: see `database --help`", APPNAME);
                }
            }
        }
        _ => {
            eprintln!("{}: see `--help`", APPNAME);
        }
    }
    Ok(())
}


quick_main!(run);

