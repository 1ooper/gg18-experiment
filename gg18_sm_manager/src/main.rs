use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

use rocket::serde::json::Json;
use rocket::{post, routes, State};

use common_lib::{Entry, Index, Key, Params, PartySignup};

#[post("/get", format = "json", data = "<request>")]
fn get(
    db_mtx: &State<RwLock<HashMap<Key, String>>>,
    request: Json<Index>,
) -> Json<Result<Entry, ()>> {
    let index: Index = request.0;
    let hm = db_mtx.read().unwrap();
    match hm.get(&index.key) {
        Some(v) => {
            let entry = Entry {
                key: index.key,
                value: v.clone(),
            };
            Json(Ok(entry))
        }
        None => Json(Err(())),
    }
}

#[post("/set", format = "json", data = "<request>")]
fn set(db_mtx: &State<RwLock<HashMap<Key, String>>>, request: Json<Entry>) -> Json<Result<(), ()>> {
    let entry: Entry = request.0;
    let mut hm = db_mtx.write().unwrap();
    hm.insert(entry.key.clone(), entry.value);
    Json(Ok(()))
}

#[post("/signupkeygen", format = "json", data= "<uuid>")]
fn signup_keygen(db_mtx: &State<RwLock<HashMap<Key, String>>>, uuid: Json<String>) -> Json<Result<PartySignup, ()>> {
    let data = fs::read_to_string("params.json")
        .expect("Unable to read params, make sure config file is present in the same folder ");
    let params: Params = serde_json::from_str(&data).unwrap();
    let parties = params.parties.parse::<u16>().unwrap();

    let key = uuid.into_inner();

    let party_signup = {
        let hm = db_mtx.read().unwrap();
        match hm.get(&key) {
            Some(value) => {
                let client_signup: PartySignup = serde_json::from_str(&value).unwrap();
                if client_signup.number < parties {
                    Ok(
                        PartySignup{
                            number: client_signup.number + 1,
                            uuid: client_signup.uuid,
                        }
                    )
                } else {
                    Err(())
                }
            }
            None => {
                let uuid = key.clone();
                Ok(
                    PartySignup{
                        number: 1,
                        uuid,
                    }
                )
            }
        }
    };
    if party_signup.is_ok() {
        let mut hm = db_mtx.write().unwrap();
        hm.insert(key, serde_json::to_string(&party_signup.clone().unwrap()).unwrap());
    }
    Json(party_signup)
}

#[post("/signupsign", format = "json", data= "<uuid>")]
fn signup_sign(db_mtx: &State<RwLock<HashMap<Key, String>>>, uuid: Json<String>) -> Json<Result<PartySignup, ()>> {
    //read parameters:
    let data = fs::read_to_string("params.json")
        .expect("Unable to read params, make sure config file is present in the same folder ");
    let params: Params = serde_json::from_str(&data).unwrap();
    let threshold = params.threshold.parse::<u16>().unwrap();
    let key = uuid.into_inner();

    let party_signup = {
        let hm = db_mtx.read().unwrap();
        match hm.get(&key) {
            Some(value) => {
                let client_signup: PartySignup = serde_json::from_str(&value).unwrap();
                if client_signup.number < threshold + 1 {
                    Ok(
                        PartySignup{
                            number: client_signup.number + 1,
                            uuid: client_signup.uuid,
                        }
                    )
                } else {
                    Err(())
                }
            }
            None => {
                let uuid = key.clone();
                Ok(
                    PartySignup{
                        number: 1,
                        uuid,
                    }
                )
            }
        }
    };
    if party_signup.is_ok() {
        let mut hm = db_mtx.write().unwrap();
        hm.insert(key, serde_json::to_string(&party_signup.clone().unwrap()).unwrap());
    }
    Json(party_signup)
}

#[tokio::main]
async fn main() {
    // let mut my_config = Config::development();
    // my_config.set_port(18001);
    let db: HashMap<Key, String> = HashMap::new();
    let db_mtx = RwLock::new(db);
    //rocket::custom(my_config).mount("/", routes![get, set]).manage(db_mtx).launch();

   

    rocket::build()
        .mount("/", routes![get, set, signup_keygen, signup_sign])
        .manage(db_mtx)
        .launch()
        .await
        .unwrap();
}