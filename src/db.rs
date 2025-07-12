// This isn't a database. maybe later
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

use crate::auth_code::{parse_auth_code, verify_auth_code};

#[derive(Debug, Deserialize, Clone)]
pub struct UserData {
    pub name: String,
    hash: String,
    pub sub: String
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

pub fn get_user(user: &str) -> Option<UserData> {
    let ud: std::sync::MutexGuard<'_, HashMap<String, UserData>> = USER_DATA.lock().unwrap();
    return ud.get(user).cloned();
}

#[derive(Clone)]
pub struct DbAuthCode {
    key: String,
    secret_hash: String,
    user: String, 
   pub nonce: String
}

pub fn add_auth_code(key: String, secret_hash: String, user: String, nonce: String) {
    let val = DbAuthCode {
        key: key.clone(),
        secret_hash,
        user,
        nonce
    };

    // Save
    {
        let mut ac = AUTH_CODE.lock().unwrap();
        ac.insert(key, val);
    }
}

#[derive(Clone)]
pub struct UserDataAndCode {
    pub user: Option<UserData>,
    pub code: DbAuthCode
}


pub fn get_user_by_auth_code(code: &str) -> Option<UserDataAndCode> {
    let auth_code = parse_auth_code(code);

    let ac = AUTH_CODE.lock().unwrap();
    ac.get(&auth_code.key)
        .filter(|user| verify_auth_code(auth_code, &user.key, &user.secret_hash))
        .map(|user| Some(UserDataAndCode {user: get_user(&user.user).clone(), code: user.clone()}))
        .unwrap_or_else(|| None)
}
