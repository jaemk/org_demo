use std::io;
use log;
use migrant_lib;
use rusqlite;
use r2d2;
use serde_json;


error_chain! {
    foreign_links {
        LogInit(log::SetLoggerError);
        FileOpen(io::Error);
        Migrant(migrant_lib::errors::Error);
        Sqlite(rusqlite::Error);
        R2D2(r2d2::Error);
        Json(serde_json::Error);
    }
    errors {
        DoesNotExist(s: String) {
            description("Query result does not exist")
            display("DoesNotExist Error: {}", s)
        }
        BadRequest(s: String) {
            description("Bad request")
            display("BadRequest: {}", s)
        }
    }
}

