// This isn't a database. maybe later
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// let mut auth_codes: HashMap<String, String> = HashMap::new();
lazy_static! {
    static ref USER_PASS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
lazy_static! {
    static ref AUTH_CODE: Mutex<HashMap<String, DbAuthCode>> = Mutex::new(HashMap::new());
}

pub fn add_user(user: String, pass: String) {
    // makes accessing this global state threadsafe (i like it)
    // it unlocks it once it falls out of scope, so it helps to wrap with a smaller scope
    {
        let mut up = USER_PASS.lock().unwrap();
        match bcrypt::hash(&pass, 12) {
            Ok(h) => {
                up.insert(user, h);
            }
            Err(_) => {
                println!("could not hash password");
            }
        }
    }
}

pub fn get_user_pass(user: String) -> Option<String> {
    {
        let up = USER_PASS.lock().unwrap();
        return up.get(&user).map(|s| s.to_string());
        // match pass {
        //     Some(x) => return Some(x.to_string()),
        //     None => return None;
        // }
    }
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
