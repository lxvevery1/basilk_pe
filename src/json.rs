use chrono::NaiveDate;
use serde_json::{from_str, to_string, Value};
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{
    migration::{Migration, JSON_VERSIONS},
    project::Project,
};

pub struct Json;

static DIR_CONFIG_NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: Mutex<String> = Mutex::new(String::new());

impl Json {
    pub fn get_dir_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap();
        path.push(DIR_CONFIG_NAME);

        path
    }

    fn get_json_path(version: String) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(Json::get_dir_path().as_path());
        path.push(format!("{version}.json"));

        path
    }

    pub fn check() -> Result<bool, Box<dyn Error>> {
        fs::create_dir_all(Json::get_dir_path())?;

        // Create the state to save the json version
        let mut version_state = VERSION.lock().unwrap();

        // Pick the version from the internal file
        let mut json_version_from_file: Vec<&str> = JSON_VERSIONS
            .into_iter()
            .filter(|version| Path::new(&Json::get_json_path(version.to_string())).is_file())
            .collect();

        // If the file doesn't exist create a new one with the last version
        if json_version_from_file.is_empty() {
            let last_json_version = JSON_VERSIONS.last().unwrap();
            let path = Json::get_json_path(last_json_version.to_string());

            let mut file = File::create(path).unwrap();
            let _ = file.write_all(b"[]");

            json_version_from_file = vec![last_json_version];
            version_state.push_str(json_version_from_file[0]);

            return Ok(false);
        }

        // Save into the internal state the last json version
        version_state.push_str(json_version_from_file[0]);

        // Read the internal file
        let path = Json::get_json_path(json_version_from_file[0].to_string());
        let json_raw = fs::read_to_string(&path).unwrap();
        let json = from_str::<Vec<Value>>(&json_raw).unwrap();

        if json.is_empty() {
            return Ok(false);
        }

        // Load all migrations
        let migrations = Migration::get_migrations(json_version_from_file[0], json);

        if migrations.is_empty() {
            return Ok(false);
        }

        // Loop thru all migrations and apply them!
        for (version, migration) in migrations.iter() {
            let path = Json::get_json_path(version_state.to_string());
            let new_path = Json::get_json_path(version.to_string());

            let new_json = migration;

            fs::write(&path, new_json).unwrap();
            fs::rename(&path, new_path)?;

            // Save into the internal state the json version of the last migration applied
            version_state.clear();
            version_state.push_str(version)
        }

        Ok(true)
    }

    pub fn read() -> Vec<Project> {
        let version = VERSION.lock().unwrap().to_string();
        let path = Json::get_json_path(version);

        // Read the JSON file
        let json = fs::read_to_string(path).unwrap();

        // Parse the JSON into a vector of projects
        let projects = from_str::<Vec<Project>>(&json).unwrap();

        // Parse dates and handle invalid dates gracefully
        let mut projects_with_dates: Vec<_> = projects
            .into_iter()
            .map(|p| {
                // Attempt to parse the title as a date
                let date = NaiveDate::parse_from_str(&p.title, "%d.%m.%Y").ok();
                (p, date)
            })
            .collect();

        // Sort projects by date (projects with invalid dates will be placed first)
        projects_with_dates.sort_by(|(_, date1), (_, date2)| {
            match (date1, date2) {
                (Some(d1), Some(d2)) => d1.cmp(d2), // Both dates are valid, compare them
                (Some(_), None) => std::cmp::Ordering::Greater, // Valid dates come after invalid dates
                (None, Some(_)) => std::cmp::Ordering::Less, // Invalid dates come before valid dates
                (None, None) => std::cmp::Ordering::Equal, // Both dates are invalid, keep their order
            }
        });

        // Extract the sorted projects (discard the dates)
        let sorted_projects: Vec<Project> =
            projects_with_dates.into_iter().map(|(p, _)| p).collect();

        sorted_projects
    }

    pub fn write(projects: Vec<Project>) {
        let version = VERSION.lock().unwrap().to_string();
        let path = Json::get_json_path(version);

        fs::write(path, to_string(&projects).unwrap()).unwrap();
    }
}
