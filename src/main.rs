use actix_web::{get, post, web::Path, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex; // Importing Mutex from std::sync

lazy_static! {
    // structure is table -> key -> value
    static ref DATA_STORE: Mutex<HashMap<String, HashMap<String, String>>> = {
        let data_store = HashMap::new();
        Mutex::new(data_store)
    };
}
#[get("/{table}/{key}")]
async fn read_key(path: Path<(String, String)>) -> impl Responder {
    let (table, key) = path.into_inner();
    let data_store = DATA_STORE.lock().unwrap();
    if let Some(table) = data_store.get(&table) {
        if let Some(_value) = table.get(&key) {
            HttpResponse::Ok().body(_value.clone())
        }
        else {
            HttpResponse::NotFound().body("Key not found")
        }	
    }
    else {
        HttpResponse::NotFound().body("Table not found")
    }
}

#[post("/{table}/{key}")]
async fn add_key(path: Path<(String, String)>, req_body: String) -> impl Responder {
    let body = req_body.clone(); // store json as a string and assume it is valid json

    let (tableName, key) = path.into_inner();
    // let table_data = *DATA_STORE.entry(table).or_insert(HashMap::new());
    let mut data_store = DATA_STORE.lock().unwrap();
    if let Some(table) = data_store.get(&tableName) {
        if let Some(_value) = table.get(&key) {
            // overwrite the existing value
            let mut table_data = table.clone();
            table_data.insert(key, body);
        }
        else {
            let mut table_data = table.clone();
            table_data.insert(key, body);
            data_store.insert(tableName, table_data);
        }	
    }
    else {
        let mut table_data = HashMap::new();
        table_data.insert(key, body);
        data_store.insert(tableName, table_data);
    }

    HttpResponse::Ok().body("key added")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read_key)
            .service(add_key)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}