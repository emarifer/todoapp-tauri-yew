use chrono::{Local, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub map: HashMap<String, (bool, String, u64)>,
}

impl Todo {
    pub fn new() -> Result<Todo, Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(get_database())?;

        // Serializar el archivo json como HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn insert(&mut self, key: &str) {
        // Insertamos un nuevo valor en nuestro mapa.
        // Por default, el value va a ser true por default.
        let (date, timestamp) = get_time_and_date();
        let key = [key.trim(), "___", &timestamp.to_string()].concat();
        self.map.insert(key, (true, date, timestamp));
    }

    pub fn save(self) -> Result<String, Box<dyn std::error::Error>> {
        // Abrir db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true) // VER NOTA-1 ABAJO:
            .create(true)
            .open(get_database())?;

        // Escribir en el archivo con serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok("ToDo saved successfully".into())
    }

    pub fn complete(&mut self, key: &str) -> Option<()> {
        match self.map.get_mut(key.trim()) {
            Some(v) => {
                let (_, date, timestamp) = v.clone();
                Some(*v = (false, date, timestamp))
            }
            None => None,
        }
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        match self.map.remove_entry(key.trim()) {
            Some(e) => Some(e.0),
            None => None,
        }
    }
}

fn get_database() -> String {
    let mut database = String::new();

    match home::home_dir() {
        Some(path) => match path.join(".db.json").to_str() {
            None => println!("The path is not a valid UTF-8 sequence"),
            Some(my_home) => database = my_home.to_string(),
        },
        None => println!("Unable to get Home directory"),
    }

    database
}

fn get_time_and_date() -> (String, u64) {
    let now = Local::now();
    let date = Utc::now().date_naive().format("%d-%m-%Y");
    // let (is_pm, hour) = now.hour12();
    let hour = now.hour();

    let duration = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!");

    let timestamp = duration.as_secs();

    let time_and_date = format!(
        "{:02}:{:02}:{:02} • {}",
        hour,
        now.minute(),
        now.second(),
        // if is_pm { "PM" } else { "AM" },
        date
    );

    (time_and_date, timestamp)
}
