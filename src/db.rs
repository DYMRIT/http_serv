use std::process::id;
use redis::Commands;


fn connection(password: &str, id: i32) -> redis::Connection {
    let url_client = format!("--//--", password, id);
    let client = match redis::Client::open(url_client) {
        Ok(client) => client,
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Something wrong with client DB");
            std::process::exit(1);
        }
    };
    let mut con = match client.get_connection() {
        Ok(con) => con,
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Something wrong with DB connection");
            std::process::exit(1);
        }
    };
    con
}


pub fn set_key(key: &str, value: &str, password: &str, id: i32) -> redis::RedisResult<()> {
    let mut con = connection(password, id);
    con.set(key, value)
}


pub fn get_key(key: &str, password: &str, id: i32) -> redis::RedisResult<String> {
    let mut con = connection(password, id);
    con.get(key)
}


pub fn delete_key(key: &str, password: &str, id: i32) -> redis::RedisResult<()> {
    let mut con = connection(password, id);
    con.del(key)
}