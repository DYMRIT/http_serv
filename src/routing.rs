use std::collections::HashMap;

use crate::handle::{handle_activate_user, handle_register};
use crate::jwt::create_jwt;
use crate::SECRET_JWT;


pub fn routing(
    envs: HashMap<String, String>,
    stdin: serde_json::Value,
    method: String,
    addr: String,
    query: HashMap<String, String>
) {
    println!("HTTP/1.1 200\r\n");
    println!("envs: {:?}", &envs);
    println!("stdin: {:?}", &stdin);
    println!("HTTP_METHOD: {:?}", &method);
    println!("addr: {:?}, query: {:?}", &addr, &query);

    let jwt = match create_jwt("dmitr", SECRET_JWT) {
        Ok(jwt) => jwt,
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Failed create JWT");
            std::process::exit(1);
        }
    };
    println!("jwt: {}", jwt);
    if addr == "/register".to_string() {
        println!("---> in {:12}", addr);
        handle_register(envs.clone(), stdin.clone());
    } else if addr == "/activate".to_string() {
        println!("In ---> {:12}", "handle_activate_user");
        handle_activate_user(envs.clone(), stdin.clone());
    } else {
        println!("---> in other {:12}", &addr);
    }
}