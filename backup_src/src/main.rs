mod database_manager;
mod fs_manager;
mod cache;

use cache::{Cache, Item};
use database_manager::{DatabaseManager, Database, Field};
use fs_manager::*;


fn main() {
    //create_dir("qwe");
    //create_file("database_test.alicedb");
    //println!("{:?}", list_dir("qwe"));
    //let mut database = Database::new("qwerty");
    //database.create_table("users", vec![Field::new("username"), Field::new("passwords")]);
    //println!("{:?}", database);
    //println!("{:?}", database.get_tables());
    //println!("{:?}", database.get_fields("users"));
    //database.set_data("users", "username".to_string(), "super_username".to_string());
    //database.set_data("users", "password".to_string(), "super_password".to_string());
}
