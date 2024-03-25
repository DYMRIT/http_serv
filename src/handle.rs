use std::collections::HashMap;
use chrono::{Duration, Utc};
use serde_json::Value;
use redis::Commands;
use crate::db::{delete_key, set_key};
use crate::helpful::{check_email, check_login, check_password, hashed_user, send_email};
use crate::{OUR_EMAIL, PASSWORD_DB, SECRET_REGISTRATION};



// Добавить проверку того, что таких 3 значения нет
pub fn handle_register(envs: HashMap<String, String>, stdin: serde_json::Value) {
    let login = check_login(stdin["login"].as_str());
    let password = check_password(stdin["password"].as_str());
    let email = check_email(stdin["email"].as_str());
    println!("login: {} \npassword: {} \nemail: {}", &login, &password, &email);
    let hash = hashed_user(&login, &password, &email, &SECRET_REGISTRATION);
    println!("hash: {}", hash);
    let time =  Utc::now().timestamp() + 5*60;
    match set_key(&hash, &time.to_string(), PASSWORD_DB, 1) {
        Ok(_) => println!("Success set"),
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Failed set key for registration");
            std::process::exit(1);
        }
    }
    let url_to_activate = format!("http://localhost:8081/activate?token={}",hash);
    let url_activate = format!("Для подтверждения перейдите на адрес: {}", url_to_activate);
    println!("{:?}", url_activate);
    match send_email(OUR_EMAIL.to_string(), email.to_string(), "Сообщение об подтверждении почты".to_string(), url_activate) {
        Ok(_) => println!("Success send message to activate"),
        Err(e1) => {
            println!("HTTP/1.1 404\r\n");
            println!("{:?}", e1);
            match delete_key(&hash, PASSWORD_DB, 1) {
                Ok(_) => println!("Failed send message, was delete data for this user"),
                Err(e2) => {
                    println!("{:?}", e2);
                    println!("Failed send message, failed delete data for this user")
                }
            }
            std::process::exit(1);
        }
    }
}


pub fn handle_activate_user(envs: HashMap<String, String>, stdin: serde_json::Value) {
}