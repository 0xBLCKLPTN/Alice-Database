mod cache;
mod configurator;
mod alice_database;
mod alice_fs;

use cache::{Cache, setup_cache};
use configurator::{Root, parse_config_file};
use alice_database::*;
use alice_fs::*;


fn main() {
    let path = "./config_example.json";
    //let k = parse_config_file(path);
    //let cache = setup_cache::<String>("users", k.unwrap().cache.unwrap());
    let mut db: AliceDB<String> = AliceDB::create_connection("./databases", None, "alice_database_test").unwrap();
    db.create_table(Table {name: "users".to_string(), fields: vec!["usernames".to_string(), "passwords".to_string()]} );
    db.create_table(Table {name: "passcodes".to_string(), fields: vec!["usernames".to_string(), "passwords".to_string()]} );
    println!("{:?}", db);

}
