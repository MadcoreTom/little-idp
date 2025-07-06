// This isn't a database. maybe later
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

use crate::auth_code::hex_to_bytes;


#[derive(Debug, Deserialize, Clone)]
pub struct UserData {
    pub name: String,
    hash: String,
    pub sub: String,
}

// let mut auth_codes: HashMap<String, String> = HashMap::new();
lazy_static! {
    static ref USER_DATA: Mutex<HashMap<String, UserData>> = Mutex::new(HashMap::new());
}
lazy_static! {
    static ref AUTH_CODE: Mutex<HashMap<String, DbAuthCode>> = Mutex::new(HashMap::new());
}

pub fn init_db() {
    let file = File::open("users.json");
    let users: Vec<UserData>;
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            let data = serde_json::from_reader(reader);
            match data {
                Ok(d) => users = d,
                Err(e) => {
                    println!("Failed to read json - {}", e.to_string());
                    return;
                }
            }
        }
        Err(e) => {
            println!("Error loading user data - {}", e.to_string());
            return;
        }
    }

    // Lock USER_DATA and insert them, using the name as the key
    let mut up = USER_DATA.lock().unwrap();
    for user in users {
        up.insert(user.name.clone(), user);
    }
}

pub fn get_user_pass(user: String) -> Option<String> {
    let ud = USER_DATA.lock().unwrap();
    return ud.get(&user).map(|data| data.hash.to_string());
}

pub fn get_user(user: String) -> Option<UserData> {
    let ud: std::sync::MutexGuard<'_, HashMap<String, UserData>> = USER_DATA.lock().unwrap();
    return ud.get(&user).cloned();
}

struct DbAuthCode {
    key: String,
    secret_hash: String,
    user: String, // TODO add expiry
}

pub fn add_auth_code(key: String, secret_hash: String, user: String) {
    let val = DbAuthCode {
        key: key.clone(),
        secret_hash,
        user,
    };

    // Save
    {
        let mut ac = AUTH_CODE.lock().unwrap();
        ac.insert(key, val);
    }
}

pub fn get_user_by_auth_code(code: String) -> Option<UserData> {
    let parts: Vec<&str> = code.splitn(2, ':').collect();
    let key = parts[0];
    let secret = parts[1];

    let ac = AUTH_CODE.lock().unwrap();

    // ac.get(key)
    //     .and_then(|user| get_user(user.user.to_string()))
    //     .and_then(|user_data| {
    //         bcrypt::verify(secret, user_data.hash.as_str())
    //             .map(|valid| if valid { Some(user_data.clone()) } else { None })
    //             .unwrap_or_else(|_|None )
    //     })

    ac.get(key)
        .and_then(|user| {
            bcrypt::verify(hex_to_bytes(secret), user.secret_hash.as_str())
                .map(|valid| {
                    if valid {
                        println!("valid");
                        Some(user.user.to_string())
                    } else {
                        println!("Invalid for user {} {} {}" , user.user, secret, user.secret_hash);
                        None
                    }
                })
                .unwrap_or_else(|_| None)
        })
        .and_then(|user| get_user(user))
}
