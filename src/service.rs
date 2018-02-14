use std::env;
use std::time;
use std::fs;
use std::sync;
use std::path::Path;

use rouille;
use env_logger;
use chrono::Local;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;

use {ToTextResponse, ToJsonResponse, FromRequestBody, migrant_config};
use models;
use errors::*;


// convenience wrapper types
pub type DbPool = Pool<SqliteConnectionManager>;
pub type State = sync::Arc<Resources>;


/// Resource bag with database access
pub struct Resources {
    pub db: DbPool,
}
impl Resources {
    pub fn new(db: DbPool) -> Self {
        Self {
            db: db,
        }
    }
}


fn establish_connection_pool<T: AsRef<Path>>(database_path: T) -> DbPool {
    let manager = SqliteConnectionManager::file(database_path.as_ref());
    Pool::new(manager).expect("Failed to create pool.")
}


/// Initialize things
/// - env logger
/// - database connection pool
/// - server
/// - handle errors
pub fn start(host: &str, port: u16) -> Result<()> {
    // Set a custom logging format & change the env-var to "LOG"
    // e.g. LOG=info org_demo serve
    use std::io::Write;
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf, "{} [{}] - [{}] -> {}",
                Local::now().format("%Y-%m-%d_%H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unknown>"),
                record.args()
                )
            })
        .parse(&env::var("LOG").unwrap_or_default())
        .init();

    let db_config = migrant_config()?;
    let pool = establish_connection_pool(&db_config.database_path()?);
    let state = sync::Arc::new(Resources::new(pool));

    let addr = format!("{}:{}", host, port);
    info!("** Listening on {} **", addr);

    rouille::start_server(&addr, move |request| {
        let state = state.clone();

        let now = Local::now().format("%Y-%m-%d %H:%M%S");
        let log_ok = |req: &rouille::Request, resp: &rouille::Response, elap: time::Duration| {
            let ms = (elap.as_secs() * 1_000) as f32 + (elap.subsec_nanos() as f32 / 1_000_000.);
            info!("[{}] {} {} -> {} ({}ms)", now, req.method(), req.raw_url(), resp.status_code, ms)
        };
        let log_err = |req: &rouille::Request, elap: time::Duration| {
            let ms = (elap.as_secs() * 1_000) as f32 + (elap.subsec_nanos() as f32 / 1_000_000.);
            info!("[{}] Handler Panicked: {} {} ({}ms)", now, req.method(), req.raw_url(), ms)
        };

        // dispatch and handle errors
        rouille::log_custom(request, log_ok, log_err, move || {
            match route_request(request, state) {
                Ok(resp) => rouille::content_encoding::apply(request, resp),
                Err(e) => {
                    use self::ErrorKind::*;
                    error!("Handler Error: {}", e);
                    match *e {
                        BadRequest(ref s) => {
                            s.to_string().to_text_resp().with_status_code(400)
                        }
                        DoesNotExist(ref s) => {
                            s.to_string().to_text_resp().with_status_code(404)
                        }
                        _ => rouille::Response::text("Something went wrong").with_status_code(500),
                    }
                }
            }
        })
    });
}


fn serve_file<T: AsRef<Path>>(path: T) -> Result<rouille::Response> {
    let path = path.as_ref();
    let ext = path.extension().and_then(::std::ffi::OsStr::to_str).unwrap_or("");
    let f = fs::File::open(&path).map_err(ErrorKind::FileOpen)?;
    Ok(rouille::Response::from_file(rouille::extension_to_mime(ext), f))
}


/// Route the request to appropriate handler
fn route_request(request: &rouille::Request, state: State) -> Result<rouille::Response> {
    Ok(router!(request,
        (GET) ["/"] => {
            serve_file("static/index.html")?
        },

        // ---- Grabbing data ----
        (GET) ["/api/orgs"] => {
            let conn = state.db.get()?;
            let orgs = models::OrgInfo::get_all_orgs(&conn)?;
            json!({"orgs": orgs}).to_json_resp()?
        },
        (GET) ["/api/user/{id}", id: u64] => {
            let conn = state.db.get()?;
            let user = models::UserInfo::get_user(id as i64, &conn)?;
            if user.is_none() { bail_fmt!(ErrorKind::DoesNotExist, "No user found") }
            json!({"user": user}).to_json_resp()?
        },

        // ---- Checking if things exist ----
        (GET) ["/api/exists/org/{name}", name: String] => {
            let conn = state.db.get()?;
            let exists = models::Org::exists(&conn, &name)?;
            json!({"exists": exists}).to_json_resp()?
        },
        (GET) ["/api/exists/user/{email}", email: String] => {
            let conn = state.db.get()?;
            let exists = models::User::exists(&conn, &email)?;
            json!({"exists": exists}).to_json_resp()?
        },
        (GET) ["/api/exists/linode/{name}", name: String] => {
            let conn = state.db.get()?;
            let exists = models::Linode::exists(&conn, &name)?;
            json!({"exists": exists}).to_json_resp()?
        },

        // ---- Creating things ----
        (POST) ["/api/create/org"] => {
            #[derive(Deserialize)]
            struct Post {
                name: String,
            }
            let post = request.parse_json_body::<Post>()
                .map_err(|_| ErrorKind::BadRequest("Invalid post data".to_string()))?;
            let conn = state.db.get()?;
            if models::Org::exists(&conn, &post.name)? {
                bail_fmt!(ErrorKind::BadRequest, "Org already exists, {}", post.name);
            }
            let org_id = models::NewOrg { name: post.name }.insert(&conn)?;
            json!({"org_id": org_id}).to_json_resp()?
        },
        (POST) ["/api/create/user"] => {
            #[derive(Deserialize)]
            struct Post {
                org_ids: Vec<i64>,
                email: String,
            }
            let post = request.parse_json_body::<Post>()
                .map_err(|_| ErrorKind::BadRequest("Invalid post data".to_string()))?;
            let mut conn = state.db.get()?;
            let trans = conn.transaction()?;
            if models::User::exists(&trans, &post.email)? {
                bail_fmt!(ErrorKind::BadRequest, "User already exists, {}", post.email);
            }
            let user_id = models::NewUser { email: post.email }.insert(&trans)?;
            for id in &post.org_ids {
                models::NewUserOrg { org: *id, user: user_id }.insert(&trans)?;
            }
            trans.commit()?;
            json!({"user_id": user_id}).to_json_resp()?
        },
        (POST) ["/api/create/linode"] => {
            #[derive(Deserialize)]
            struct Post {
                org_id: i64,
                name: String,
            }
            let post = request.parse_json_body::<Post>()
                .map_err(|_| ErrorKind::BadRequest("Invalid post data".to_string()))?;
            let conn = state.db.get()?;
            if models::Linode::exists(&conn, &post.name)? {
                bail_fmt!(ErrorKind::BadRequest, "Linode already exists, {}", post.name);
            }
            let linode_id = models::NewLinode { name: post.name, org: post.org_id }.insert(&conn)?;
            json!({"linode_id": linode_id}).to_json_resp()?
        },

        // ---- misc ----
        (GET) ["/favicon.ico"]  => { serve_file("static/favicon.ico")? },
        (GET) ["/robots.txt"]   => { serve_file("static/robots.txt")? },
        _ => {
            // static files
            if let Some(req) = request.remove_prefix("/static") {
                let static_resp = rouille::match_assets(&req, "static");
                if static_resp.is_success() {
                    return Ok(static_resp)
                }
            }
            error!("{:?}", request);
            bail_fmt!(ErrorKind::DoesNotExist, "nothing here")
        }
    ))
}

