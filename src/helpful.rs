use std::collections::HashMap;
use std::{env, io};
use Result;
use std::io::Read;
use sha256::digest;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::Error;
use regex::Regex;
use serde_json::Value;


pub fn get_env_vars(prefix: &str) -> HashMap<String, String> {
    env::vars() // Получаем итератор по переменным окружения
        .filter(|(key, _)| key.starts_with(prefix)) // Фильтруем те, что начинаются с заданного префикса
        .map(|(key, value)| (key.strip_prefix(prefix).unwrap().to_string(), value)) // Удаляем префикс из ключа
        .collect()
}


pub fn get_stdin() -> String {
    let content_length = std::env::var("CONTENT_LENGTH")
        .unwrap_or_else(|_| "0".to_string());

    if content_length == "0" {
        return "".to_string()
    }
    let length = match content_length.parse::<usize>() {
        Ok(l) => l,
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Not valid json data in stdin");
            std::process::exit(1);
        }
    };

    let mut buffer = vec![0; length];

    io::stdin().read_exact(&mut buffer).unwrap_or_default();

    let body = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(_) => {
            println!("HTTP/1.1 404\r\n");
            println!("Not valid json data in stdin");
            std::process::exit(1);
        }
    };

    body
}


pub fn parse_query_string(input: &str) -> Result<(String, HashMap<String, String>), &'static str> {
    let parts: Vec<&str> = input.splitn(2, '?').collect();
    match parts.len() {
        1 => Ok((parts[0].to_string(), HashMap::new())),
        2 => {
            let (base, query) = (parts[0].to_string(), parts[1]);
            let mut params: HashMap<String, String> = HashMap::new();

            for pair in query.split('&') {
                let key_value: Vec<&str> = pair.splitn(2, '=').collect();
                if key_value.len() != 2 {
                    return Err("Invalid query string format");
                }
                params.insert(key_value[0].to_string(), key_value[1].to_string());
            }

            Ok((base, params))
        }
        _ => Err("Input string contains more than one '?'"),
    }
}


pub fn hashed_user(login: &str, password: &str, email: &str, secret: &str) -> String {
    let combined_data = format!("{}{}{}{}", login, password, email, secret);
    let val = digest(combined_data);
    val
}


pub fn send_email(from_email: String, to_email: String, head: String, message: String) -> Result<String, Error> {
    let email = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(head)
        .header(ContentType::TEXT_HTML)
        .body(String::from(message))
        .unwrap();

    let creds = Credentials::new("--//--".to_owned(), "--//--".to_owned());

    let mailer = SmtpTransport::relay("--//--")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(asd) => {
            Ok("Ok".to_string())
        },
        Err(e) => Err(e)
    }
}


pub fn check_login(login: Option<&str>) -> &str {
    match login {
        Some(login) => {
            if (!login.len() < 3 || !login.len() > 20) && (login.chars().all(|c| c.is_alphanumeric())) {
                login
            } else {
                println!("HTTP/1.1 404\r\n");
                println!("Not valid login");
                std::process::exit(1);
            }
        },
        None => {
            println!("HTTP/1.1 404\r\n");
            println!("What's wrong with login (not send or wrong format)");
            std::process::exit(1);
        }
    }
}


pub fn check_password(password: Option<&str>) -> &str {
    match password {
        Some(password) => {
            if !password.len() < 8 || !password.len() > 32 {
                let mut has_digit = false;
                let mut has_uppercase = false;
                let mut has_lowercase = false;
                let mut has_special = false;

                for char in password.chars() {
                    if char.is_numeric() {
                        has_digit = true;
                    } else if char.is_uppercase() {
                        has_uppercase = true;
                    } else if char.is_lowercase() {
                        has_lowercase = true;
                    } else if "!@#$%^&*()_+-=[]{}|;:',.<>?/".contains(char) {
                        has_special = true;
                    }
                }
                if has_digit && has_uppercase && has_lowercase && has_special {
                    password
                } else {
                    println!("HTTP/1.1 404\r\n");
                    println!("Password should have 1 digit, 1 uppercase, 1 lowercase and 1 special symbol");
                    std::process::exit(1);
                }
            } else {
                println!("HTTP/1.1 404\r\n");
                println!("Wrong length in password");
                std::process::exit(1);
            }
        },
        None => {
            println!("HTTP/1.1 404\r\n");
            println!("What's wrong with password (not send or wrong format)");
            std::process::exit(1);
        }
    }
}


pub fn check_email(email: Option<&str>) -> &str {
    match email {
        Some(email) => {
            match Regex::new(r"(?i)^[a-z0-9_.+-]+@[a-z0-9-]+\.[a-z0-9-.]+$") {
                Ok(email_regex) => {
                    if email_regex.is_match(email) {
                        email
                    } else {
                        println!("HTTP/1.1 404\r\n");
                        println!("Not valid email");
                        std::process::exit(1);
                    }
                },
                Err(_) => {
                    println!("HTTP/1.1 404\r\n");
                    println!("What's wrong with regex to check email");
                    std::process::exit(1);
                }
            }
        },
        None => {
            println!("HTTP/1.1 404\r\n");
            println!("What's wrong with email (not send or wrong format)");
            std::process::exit(1);
        }
    }
}