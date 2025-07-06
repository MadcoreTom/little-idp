// This isn't a database. maybe later
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

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

// pub fn add_user(user: String, pass: String) {
//     // makes accessing this global state threadsafe (i like it)
//     // it unlocks it once it falls out of scope, so it helps to wrap with a smaller scope
//     {
//         let mut up = USER_PASS.lock().unwrap();
//         match bcrypt::hash(&pass, 12) {
//             Ok(h) => {
//                 up.insert(user, h);
//             }
//             Err(_) => {
//                 println!("could not hash password");
//             }
//         }
//     }
// }

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

pub fn find_auth_code(code: String) -> Option<String> {
    let parts: Vec<&str> = code.splitn(2, ':').collect();
    let key = parts[0];
    // let secret = parts[1];
    {
        let ac = AUTH_CODE.lock().unwrap();
        let result = ac.get(key);
        match result {
            Some(row) => {
                // TODO bcrypt match the secret part
                return Some(row.user.to_string());
            }
            None => return None,
        }
    }
}
