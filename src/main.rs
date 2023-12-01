mod cache;
mod configurator;
mod alice_database;

use cache::{Cache, setup_cache};
use configurator::{Root, parse_config_file};
use alice_database::*;



fn main() {
    let path = "./config_example.json";
    //let k = parse_config_file(path);
    //let cache = setup_cache::<String>("users", k.unwrap().cache.unwrap());
    let mut db: AliceDB<String> = AliceDB::create_connection("/databases", None);
    db.create_table(Table {name: "users".to_string(), fields: vec!["usernames".to_string(), "passwords".to_string()]} );
    db.create_table(Table {name: "passcodes".to_string(), fields: vec!["usernames".to_string(), "passwords".to_string()]} );
    println!("{:?}", db);

}
