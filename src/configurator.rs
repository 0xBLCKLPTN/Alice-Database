
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::copy;
use std::io::stdout;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
	#[serde(rename = "host")]
	pub host: Option<String>,

	#[serde(rename = "port")]
	pub port: Option<i32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cache_config {
	#[serde(rename = "enable_server")]
	pub enable_server: Option<bool>,

	#[serde(rename = "server")]
	pub server: Option<Server>,

	#[serde(rename = "capacity")]
	pub capacity: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alice_db_config {
	#[serde(rename = "enable_server")]
	pub enable_server: Option<bool>,

	#[serde(rename = "server")]
	pub server: Option<Server>,

	#[serde(rename = "path_to_databases")]
	pub path_to_databases: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
	#[serde(rename = "cache_config")]
	pub cache: Option<Cache_config>,

	#[serde(rename = "alice_db_config")]
	pub alice_db: Option<Alice_db_config>,
}

pub fn parse_config_file(path_to_config: &str) -> Result<Root, Box<dyn std::error::Error>> {
	let file = std::fs::File::open(path_to_config)?;
	let reader = std::io::BufReader::new(file);
	let p: Root = serde_json::from_reader(reader)?;
	Ok(p)
}
