// This isn't a database. maybe later
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// let mut auth_codes: HashMap<String, String> = HashMap::new();
lazy_static! {
    static ref USER_PASS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
// TODO add a map of user->pass (bcrypted)

pub fn add_user(user: String, pass: String) {
    // makes accessing this global state threadsafe (i like it)
    // it unlocks it once it falls out of scope, so it helps to wrap with a smaller scope
    {
        let mut up = USER_PASS.lock().unwrap();
        match bcrypt::hash(&pass, 12){
            Ok(h) => {
                up.insert(user, h);
            },
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