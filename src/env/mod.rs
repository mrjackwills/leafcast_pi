use dotenv::dotenv;
use std::{collections::HashMap, env, time::SystemTime};
use thiserror::Error;
use time::UtcOffset;
use time_tz::{timezones, Offset, TimeZone};

type EnvHashMap = HashMap<String, String>;

#[derive(Debug, Error)]

enum EnvError {
    #[error("missing env: '{0}'")]
    NotFound(String),
}

#[derive(Debug, Clone)]
pub struct AppEnv {
    pub trace: bool,
    pub location_log_combined: String,
    pub location_log_error: String,
    pub location_ip_address: String,
    pub location_images: String,
    pub rotation: u16,
    pub debug: bool,
    pub start_time: SystemTime,
    pub timezone: String,
    pub utc_offset: UtcOffset,
    pub ws_address: String,
    pub ws_apikey: String,
    pub ws_auth_address: String,
    pub ws_password: String,
}

impl AppEnv {
    /// Parse "true" or "false" to bool, else false
    fn parse_boolean(key: &str, map: &EnvHashMap) -> bool {
        match map.get(key) {
            Some(value) => value == "true",
            None => false,
        }
    }

    /// Return offset for given timezone, else utc
    fn parse_offset(map: &EnvHashMap) -> UtcOffset {
        if let Some(data) = map.get("TIMEZONE") {
            if let Some(value) = timezones::get_by_name(data) {
                return value
                    .get_offset_utc(&time::OffsetDateTime::now_utc())
                    .to_utc();
            }
        }
        UtcOffset::from_hms(0, 0, 0).unwrap()
    }

    fn parse_string(key: &str, map: &EnvHashMap) -> Result<String, EnvError> {
        match map.get(key) {
            Some(value) => Ok(value.into()),
            None => Err(EnvError::NotFound(key.into())),
        }
    }

    /// Check that a given timezone is valid, else return UTC
    fn parse_timezone(map: &EnvHashMap) -> String {
        if let Some(data) = map.get("TIMEZONE") {
            if timezones::get_by_name(data).is_some() {
                return data.to_owned();
            }
        }
        "Etc/UTC".to_owned()
    }

    /// Check that a given timezone is valid, else return UTC
    fn parse_rotation(map: &EnvHashMap) -> u16 {
        if let Some(data) = map.get("ROTATION") {
            if data == "180" {
                return 180;
            }
        }
        0
    }

    /// Load, and parse .env file, return AppEnv
    fn generate() -> Result<Self, EnvError> {
        let env_map = env::vars()
            .into_iter()
            .map(|i| (i.0, i.1))
            .collect::<HashMap<String, String>>();

        Ok(Self {
            debug: Self::parse_boolean("DEBUG", &env_map),
            // TODO location exists!
            location_images: Self::parse_string("LOCATION_IMAGES", &env_map)?,
            location_log_combined: Self::parse_string("LOCATION_LOG_COMBINED", &env_map)?,
            location_log_error: Self::parse_string("LOCATION_LOG_ERROR", &env_map)?,
            // TODO location exists!
            location_ip_address: Self::parse_string("LOCATION_IP_ADDRESS", &env_map)?,
            rotation: Self::parse_rotation(&env_map),
            start_time: SystemTime::now(),
            timezone: Self::parse_timezone(&env_map),
            trace: Self::parse_boolean("TRACE", &env_map),
            utc_offset: Self::parse_offset(&env_map),
            ws_address: Self::parse_string("WS_ADDRESS", &env_map)?,
            ws_apikey: Self::parse_string("WS_APIKEY", &env_map)?,
            ws_auth_address: Self::parse_string("WS_AUTH_ADDRESS", &env_map)?,
            ws_password: Self::parse_string("WS_PASSWORD", &env_map)?,
        })
    }

    pub fn get() -> Self {
        dotenv().ok();
        match AppEnv::generate() {
            Ok(s) => s,
            Err(e) => {
                println!("\n\x1b[31m{}\x1b[0m\n", e);
                std::process::exit(1);
            }
        }
    }
}

/// Run tests with
///
/// cargo watch -q -c -w src/ -x 'test env_ -- --nocapture'
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_missing_env() {
        let mut map = HashMap::new();
        map.insert("not_fish".to_owned(), "not_fish".to_owned());
        // ACTION
        let result = AppEnv::parse_string("fish", &map);

        // CHECK
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "missing env: 'fish'");
    }

    #[test]
    fn env_parse_string_valid() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("LOCATION_SQLITE".to_owned(), "/alarms.db".to_owned());

        // ACTION
        let result = AppEnv::parse_string("LOCATION_SQLITE", &map).unwrap();

        // CHECK
        assert_eq!(result, "/alarms.db");
    }

    #[test]
    fn env_parse_boolean_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("valid_true".to_owned(), "true".to_owned());
        map.insert("valid_false".to_owned(), "false".to_owned());
        map.insert("invalid_but_false".to_owned(), "as".to_owned());

        // ACTION
        let result01 = AppEnv::parse_boolean("valid_true", &map);
        let result02 = AppEnv::parse_boolean("valid_false", &map);
        let result03 = AppEnv::parse_boolean("invalid_but_false", &map);
        let result04 = AppEnv::parse_boolean("missing", &map);

        // CHECK
        assert!(result01);
        assert!(!result02);
        assert!(!result03);
        assert!(!result04);
    }

    #[test]
    fn env_parse_offset_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "America/New_York".to_owned());

        // ACTION
        let result = AppEnv::parse_offset(&map);

        // CHECK
        assert_eq!(result, UtcOffset::from_hms(-4, 0, 0).unwrap());

        // FIXTURES
        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "Europe/Berlin".to_owned());

        // ACTION
        let result = AppEnv::parse_offset(&map);

        // CHECK
        assert_eq!(result, UtcOffset::from_hms(2, 0, 0).unwrap());

        // FIXTURES
        let map = HashMap::new();

        // ACTION
        let result = AppEnv::parse_offset(&map);

        // CHECK
        assert_eq!(result, UtcOffset::from_hms(0, 0, 0).unwrap())
    }

    #[test]
    fn env_parse_offset_err() {
        // typo time zone
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "america/New_York".to_owned());

        // ACTION
        let result = AppEnv::parse_offset(&map);
        // CHECK
        assert_eq!(result, UtcOffset::from_hms(0, 0, 0).unwrap());

        // No timezone present
        // FIXTURES
        let map = HashMap::new();
        let result = AppEnv::parse_offset(&map);

        // CHECK
        assert_eq!(result, UtcOffset::from_hms(0, 0, 0).unwrap())
    }
    #[test]
    fn env_parse_rotation_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("ROTATION".to_owned(), "180".to_owned());

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, 180);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert("ROTATION".to_owned(), "0".to_owned());

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, 0);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert("ROTATION".to_owned(), "181".to_owned());

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, 0);

        let mut map = HashMap::new();
        map.insert("ROTATION".to_owned(), "".to_owned());

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, 0);

        // FIXTURES
        let map = HashMap::new();

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, 0);
    }

    #[test]
    fn env_parse_timezone_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "America/New_York".to_owned());

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result, "America/New_York");

        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "Europe/Berlin".to_owned());

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result, "Europe/Berlin");

        // FIXTURES
        let map = HashMap::new();

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result, "Etc/UTC");
    }

    #[test]
    fn env_parse_timezone_err() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert("TIMEZONE".to_owned(), "america/New_York".to_owned());

        // ACTION
        let result = AppEnv::parse_timezone(&map);
        // CHECK
        assert_eq!(result, "Etc/UTC");

        // No timezone present
        // FIXTURES
        let map = HashMap::new();
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result, "Etc/UTC");
    }
    #[test]
    fn env_panic_appenv() {
        // ACTION
        let result = AppEnv::generate();

        assert!(result.is_err());
    }

    #[test]
    fn env_return_appenv() {
        // FIXTURES
        dotenv().ok();

        // ACTION
        let result = AppEnv::generate();

        assert!(result.is_ok());
    }
}
