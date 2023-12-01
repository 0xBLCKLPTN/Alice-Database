use std::error::Error;
use crate::cache;
use crate::cache::Cache;
use crate::alice_fs::*;

#[derive(Debug)]
pub struct AliceDB<T> {
    pub databases_path: String,
    pub tables: Vec<Table>,
    pub cache: Option<Cache<T>>
}

#[derive(Debug)]
pub struct AliceDBwCache<T> {
    pub databases_path: String,
    pub tables: Vec<Table>,
    pub cache: Cache<T>
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub fields: Vec<String>,
}

type Res = Result<(), Box<dyn Error>>;

impl<T> AliceDB<T> {
    pub fn create_connection(databases_path: &str, cache: Option<Cache<T>>, database_name: &str) -> Result<AliceDB<T>, Box<dyn Error>> {
        let mut tables: Vec<Table> = Vec::new();
        create_dir(&databases_path.to_string())?;
        create_dir(&(databases_path.to_string() + "/" + database_name));
        return match cache {
            Some(c) => Ok(
                AliceDB {
                    databases_path: databases_path.to_string() + "/" + database_name,
                    cache: Some(c),
                    tables
                }
            ),
            _ => Ok(
                AliceDB {
                    databases_path: databases_path.to_string() + "/" + database_name,
                    cache: None,
                    tables
                }
            ),
        }

    }
    pub fn create_table(&mut self, table: Table) -> Res {
        let mut first_column = String::new();
        for i in &table.fields {
            first_column += &(i.to_owned() + "\n");

        }
        create_file(&(self.databases_path.clone() + "/" + &table.name)).expect("Error");
        write_into_file(&(self.databases_path.clone() + "/" + &table.name), first_column);
        self.tables.push(table);
        Ok(())
    }

    pub fn delete_table(&mut self, table_name: &str) -> Res {
        Ok(self.tables.retain(|x| x.name == table_name ))

    }

    pub fn add_data_to_table(&mut self, table_name: &str, data: Vec<String>) -> Res {

        Ok(())
    }

    pub fn get_data_from_table(&mut self, table_name: &str) -> Res {
        Ok(())
    }
    pub fn delete_data_from_table(&mut self, table_name: &str) -> Res {
        Ok(())
    }
}