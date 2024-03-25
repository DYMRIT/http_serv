mod helpful;
mod routing;
mod jwt;
mod handle;
mod db;

use std::io::stdin;
use chrono::{Duration, Utc};
use helpful::{
    get_stdin,
    get_env_vars,
    parse_query_string,
};
use routing::{routing};
use crate::helpful::send_email;
use redis::Commands;
use crate::db::{get_key, set_key};

const SECRET_JWT: &[u8] = b"--//--";
const SECRET_REGISTRATION: &str = "--//--";
const PASSWORD_DB: &str = "--//--";
const OUR_EMAIL: &str = "--//--";

fn main() {

    dotenv::dotenv().ok();
    let envs = get_env_vars("");
    let stdin = get_stdin();
    let stdin = if stdin == "".to_string() {
            serde_json::json!("")
        } else {
            match serde_json::from_str(&stdin) {
                Ok(v) => v,
                Err(_) => {
                    println!("HTTP/1.1 404\r\n");
                    println!("Not json format in HTTP_BODY");
                    std::process::exit(1);
                }
            }
        };
    let method = match envs.get("REQUEST_METHOD") {
        Some(method) => method,
        None => {
            println!("HTTP/1.1 404\r\n");
            println!("Don't send REQUEST_METHOD");
            std::process::exit(1);
        }
    };
    let uri = match envs.get("REQUEST_URI") {
        Some(uri) => uri,
        None => {
            println!("HTTP/1.1 404\r\n");
            println!("Don't send REQUEST_URI");
            std::process::exit(1);
        }
    };
    let (addr, query_hash) = match parse_query_string(&uri.clone()) {
        Ok((addr, query)) => (addr, query),
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Wrong REQUEST_URI");
            std::process::exit(1);
        }
    };

    routing(envs.clone(), stdin, method.clone(), addr, query_hash);
}


