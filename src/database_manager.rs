use std::error::Error;
use crate::fs_manager::*;
use crate::fs_manager;
use std::io;

#[derive(Debug)]
pub struct Database {
    pub path: String,
    pub tables: Vec<Table>,
}


pub trait DatabaseManager {
    fn new(db_name: &str) -> Database;
    fn create_table(&mut self, table_name: &str, fields: Vec<Field>);
    fn get_tables(&self) -> Vec<String>;
    fn get_fields(&self, table_name: &str) -> Option<Vec<Field>>;
    fn set_data(&mut self, table_name: &str, field_name: String ,data: String) -> io::Result<()>;
    fn get_data(&mut self, table_name: &str);
}

impl DatabaseManager for Database {
    fn new(db_name: &str) -> Database {
        let mut tables = Vec::new();
        create_dir("databases/".to_string());
        create_dir("databases/".to_string() + db_name);
        Database { path: db_name.to_string() , tables }
    }

    fn create_table(&mut self, table_name: &str, fields: Vec<Field>) {
        create_file("databases/".to_string() + &self.path + "/" + table_name + ".alicedb");
        let mut first_column = String::new();
        for i in &fields {
            first_column += &(i.name.clone() + "\n");
        }
        write_into_file("databases/".to_string() + &self.path + "/" + table_name + ".alicedb", first_column);
        self.tables.push(Table::new(&table_name.to_string(), fields));
    }

    fn get_tables(&self) -> Vec<String>{
        let mut tables: Vec<String> = Vec::new();
        for i in &self.tables {
            tables.push(i.t_name.to_string());
        };
        tables
    }
    fn get_fields(&self, table_name: &str) -> Option<Vec<Field>> {
        for i in &self.tables {
            if i.t_name == table_name.to_string() {
                return Some(i.fields.clone());
            };
        }
        None
    }

    fn set_data(&mut self, table_name: &str, field_name: String ,data: String) -> io::Result<()> {
        fs_manager::into_field("databases/qwerty/users.alicedb".to_string(), &field_name, &data)?;
        Ok(())
        
    }
    fn get_data(&mut self, table_name: &str) {
        todo!();
    }

}

// <-----------------

#[derive(Debug, Clone)]
pub struct Table {
    pub t_name: String,
    pub fields: Vec<Field>
}

impl Table {
    fn new(t_name: &str, fields: Vec<Field>) -> Table {
        Table {t_name: t_name.to_string(), fields: fields.clone() }
    }
}


// <-----------

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String
}

impl Field {
    pub fn new(name: &str) -> Field { Field {name: name.to_string()} }
}