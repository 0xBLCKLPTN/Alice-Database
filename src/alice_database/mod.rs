use std::error::Error;
use crate::cache;
use crate::cache::Cache;

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
    pub fn create_connection(databases_path: &str, cache: Option<Cache<T>>) -> AliceDB<T> {
        let mut tables: Vec<Table> = Vec::new();
        return match cache {
            Some(c) => AliceDB { databases_path: databases_path.to_string(), cache: Some(c), tables },
            _ => AliceDB { databases_path: databases_path.to_string(), cache: None, tables },
        }

    }
    pub fn create_table(&mut self, table: Table) -> Res {
        let mut first_column = String::new();
        for i in &table.fields {
            first_column += &(i.to_owned() + "\n");
        }
        self.tables.push(table);
        Ok(())
    }

    pub fn delete_table(&mut self, table_name: &str) -> Res {
        Ok(self.tables.retain(|x| x.name == table_name ))

    }

    pub fn add_data_to_table(&mut self, table_name: &str) -> Res {
        Ok(())
    }

    pub fn get_data_from_table(&mut self, table_name: &str) -> Res {
        Ok(())
    }
    pub fn delete_data_from_table(&mut self, table_name: &str) -> Res {
        Ok(())
    }
}