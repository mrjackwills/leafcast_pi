use std::{collections::HashMap, env, fmt, time::SystemTime};

use jiff::tz::TimeZone;

use crate::app_error::AppError;

type EnvHashMap = HashMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_disp = match self {
            Self::Zero => 0,
            Self::Ninety => 90,
            Self::OneEighty => 180,
            Self::TwoSeventy => 270,
        };
        write!(f, "{to_disp}")
    }
}

#[derive(Debug, Clone)]
pub struct AppEnv {
    pub location_images: String,
    pub location_ip_address: String,
    pub location_log: String,
    pub log_level: tracing::Level,
    pub rotation: Rotation,
    pub start_time: SystemTime,
    pub timezone: TimeZone,
    pub ws_address: String,
    pub ws_apikey: String,
    pub ws_password: String,
    pub ws_token_address: String,
}

impl AppEnv {
    fn check_file_exists(filename: String) -> Result<String, AppError> {
        if std::fs::exists(&filename)? {
            Ok(filename)
        } else {
            Err(AppError::FileNotFound(filename))
        }
    }

    /// Parse "true" or "false" to bool, else false
    fn parse_boolean(key: &str, map: &EnvHashMap) -> bool {
        map.get(key).is_some_and(|value| value == "true")
    }

    /// Parse debug and/or trace into tracing level
    fn parse_log(map: &EnvHashMap) -> tracing::Level {
        if Self::parse_boolean("LOG_TRACE", map) {
            tracing::Level::TRACE
        } else if Self::parse_boolean("LOG_DEBUG", map) {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        }
    }

    /// Check that a given timezone is valid, else return UTC
    fn parse_rotation(map: &EnvHashMap) -> Rotation {
        if let Some(data) = map.get("ROTATION") {
            return match data.as_ref() {
                "90" => Rotation::Ninety,
                "180" => Rotation::OneEighty,
                "270" => Rotation::TwoSeventy,
                _ => Rotation::Zero,
            };
        }
        Rotation::Zero
    }

    fn parse_string(key: &str, map: &EnvHashMap) -> Result<String, AppError> {
        map.get(key)
            .map_or(Err(AppError::MissingEnv(key.into())), |value| {
                Ok(value.into())
            })
    }

    /// Check that a given timezone is valid, else return UTC
    fn parse_timezone(map: &EnvHashMap) -> TimeZone {
        map.get("TIMEZONE").map_or(TimeZone::UTC, |s| {
            jiff::tz::TimeZone::get(s).unwrap_or(TimeZone::UTC)
        })
    }

    /// Load, and parse .env file, return AppEnv
    fn generate() -> Result<Self, AppError> {
        let env_map = env::vars()
            .map(|i| (i.0, i.1))
            .collect::<HashMap<String, String>>();

        Ok(Self {
            location_images: Self::check_file_exists(Self::parse_string(
                "LOCATION_IMAGES",
                &env_map,
            )?)?,
            location_log: Self::check_file_exists(Self::parse_string("LOCATION_LOG", &env_map)?)?,
            location_ip_address: Self::check_file_exists(Self::parse_string(
                "LOCATION_IP_ADDRESS",
                &env_map,
            )?)?,
            log_level: Self::parse_log(&env_map),
            rotation: Self::parse_rotation(&env_map),
            start_time: SystemTime::now(),
            timezone: Self::parse_timezone(&env_map),
            ws_address: Self::parse_string("WS_ADDRESS", &env_map)?,
            ws_apikey: Self::parse_string("WS_APIKEY", &env_map)?,
            ws_token_address: Self::parse_string("WS_TOKEN_ADDRESS", &env_map)?,
            ws_password: Self::parse_string("WS_PASSWORD", &env_map)?,
        })
    }

    pub fn get() -> Self {
        dotenvy::dotenv().ok();
        match Self::generate() {
            Ok(s) => s,
            Err(e) => {
                println!("\n\x1b[31m{e}\x1b[0m\n");
                std::process::exit(1);
            }
        }
    }
}

/// Run tests with
///
/// cargo watch -q -c -w src/ -x 'test env_ -- --nocapture'
#[cfg(test)]
#[expect(clippy::unwrap_used)]
mod tests {
    use crate::S;

    use super::*;

    #[test]
    fn env_missing_env() {
        let mut map = HashMap::new();
        map.insert(S!("not_fish"), S!("not_fish"));
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
        map.insert(S!("LOCATION_SQLITE"), S!("/alarms.db"));

        // ACTION
        let result = AppEnv::parse_string("LOCATION_SQLITE", &map).unwrap();

        // CHECK
        assert_eq!(result, "/alarms.db");
    }

    #[test]
    fn env_parse_boolean_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("valid_true"), S!("true"));
        map.insert(S!("valid_false"), S!("false"));
        map.insert(S!("invalid_but_false"), S!("as"));

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
    fn env_parse_rotation_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!("90"));

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::Ninety);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!("180"));

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::OneEighty);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!("270"));

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::TwoSeventy);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!("0"));

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::Zero);

        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!("181"));

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::Zero);

        let mut map = HashMap::new();
        map.insert(S!("ROTATION"), S!());

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::Zero);

        // FIXTURES
        let map = HashMap::new();

        // ACTION
        let result = AppEnv::parse_rotation(&map);

        // CHECK
        assert_eq!(result, Rotation::Zero);
    }

    #[test]
    fn env_parse_timezone_ok() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("TIMEZONE"), S!("America/New_York"));

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result.iana_name(), Some("America/New_York"));

        let mut map = HashMap::new();
        map.insert(S!("TIMEZONE"), S!("Europe/Berlin"));

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result.iana_name(), Some("Europe/Berlin"));

        // FIXTURES
        let map = HashMap::new();

        // ACTION
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result.iana_name(), Some("UTC"));
    }

    #[test]
    fn env_parse_timezone_err() {
        // FIXTURES
        let mut map = HashMap::new();
        map.insert(S!("TIMEZONE"), S!("america/New__York"));

        // ACTION
        let result = AppEnv::parse_timezone(&map);
        // CHECK
        assert_eq!(result.iana_name(), Some("UTC"));

        // No timezone present
        // FIXTURES
        let map = HashMap::new();
        let result = AppEnv::parse_timezone(&map);

        // CHECK
        assert_eq!(result.iana_name(), Some("UTC"));
    }

    #[test]
    fn env_parse_log_valid() {
        // FIXTURES
        let map = HashMap::from([(S!("RANDOM_STRING"), S!("123"))]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::INFO);

        // FIXTURES
        let map = HashMap::from([(S!("LOG_DEBUG"), S!("false"))]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::INFO);

        // FIXTURES
        let map = HashMap::from([(S!("LOG_TRACE"), S!("false"))]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::INFO);

        // FIXTURES
        let map = HashMap::from([
            (S!("LOG_DEBUG"), S!("false")),
            (S!("LOG_TRACE"), S!("false")),
        ]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::INFO);

        // FIXTURES
        let map = HashMap::from([
            (S!("LOG_DEBUG"), S!("true")),
            (S!("LOG_TRACE"), S!("false")),
        ]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::DEBUG);

        // FIXTURES
        let map = HashMap::from([(S!("LOG_DEBUG"), S!("true")), (S!("LOG_TRACE"), S!("true"))]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::TRACE);

        // FIXTURES
        let map = HashMap::from([
            (S!("LOG_DEBUG"), S!("false")),
            (S!("LOG_TRACE"), S!("true")),
        ]);

        // ACTION
        let result = AppEnv::parse_log(&map);

        // CHECK
        assert_eq!(result, tracing::Level::TRACE);
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
        dotenvy::dotenv().ok();

        // ACTION
        let result = AppEnv::generate();

        assert!(result.is_ok());
    }
}
