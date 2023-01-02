use crate::env::AppEnv;

mod font;

use figlet_rs::FIGfont;
use font::{Color, FontName};

const RESET: &str = "\x1b[0m";

/// Convert input string to ASCII art
fn create_art(input: &str, fontname: FontName) -> String {
    FIGfont::from_content(FontName::get(fontname)).map_or(String::new(), |font| {
        let figure = font.convert(input);
        figure.map_or(String::new(), |text| text.to_string())
    })
}

/// Add color to a given string
fn paint_text(text: &str, color: Color) -> String {
    let tint = Color::get(color);
    let painted = text
        .lines()
        .into_iter()
        .map(|i| format!("{tint}{i}\n"))
        .collect::<String>();
    format!("{painted}{RESET}")
}

/// Show the intro texts
fn display_intro(app_envs: &AppEnv) -> String {
    let leafcast = paint_text(&create_art("Leafcast", FontName::Colossal), Color::Magenta);

    let author = env!("CARGO_PKG_AUTHORS");
    let semver = env!("CARGO_PKG_VERSION");
    let version = paint_text(&format!("v{semver}    {author}"), Color::Green);

    let mut output = format!("{leafcast}{version}");

    match app_envs.log_level {
        tracing::Level::TRACE => {
            output.push('\n');
            let debug = paint_text("!! TRACE MODE !!", Color::BgRed);
            for _ in 0..=2 {
                output.push_str(&debug);
            }
        }
        tracing::Level::DEBUG => {
            output.push('\n');
            let debug = paint_text("!! DEBUG MODE !!", Color::BgRed);
            for _ in 0..=2 {
                output.push_str(&debug);
            }
        }
        _ => {}
    }
    output.push('\n');
    output
}

pub struct Intro {
    data: String,
}

impl Intro {
    pub fn new(app_envs: &AppEnv) -> Self {
        Self {
            data: display_intro(app_envs),
        }
    }
    pub fn show(self) {
        println!("{}", self.data);
    }
}

/// WordArt tests
///
/// cargo watch -q -c -w src/ -x 'test word_art -- --nocapture'
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::env::EnvTimeZone;

    use super::*;
    use std::time::SystemTime;

    #[test]
    fn word_art_display_intro_trace() {
        let na = String::from("na");
        let args = AppEnv {
            location_images: na.clone(),
            location_ip_address: na.clone(),
            location_log: na.clone(),
            log_level: tracing::Level::TRACE,
            rotation: 0,
            start_time: SystemTime::now(),
            timezone: EnvTimeZone::new(String::new()),
            ws_address: na.clone(),
            ws_apikey: na.clone(),
            ws_password: na.clone(),
            ws_token_address: na,
        };

        let result = display_intro(&args);
        assert!(result.contains("!! TRACE"));
        assert!(!result.contains("!! DEBUG"));
    }

    #[test]
    fn word_art_display_intro_debug() {
        let na = String::from("na");
        let args = AppEnv {
            location_images: na.clone(),
            location_ip_address: na.clone(),
            location_log: na.clone(),
            log_level: tracing::Level::DEBUG,
            rotation: 0,
            start_time: SystemTime::now(),
            timezone: EnvTimeZone::new(String::new()),
            ws_address: na.clone(),
            ws_apikey: na.clone(),
            ws_password: na.clone(),
            ws_token_address: na,
        };

        let result = display_intro(&args);
        assert!(!result.contains("!! TRACE"));
        assert!(result.contains("!! DEBUG"));
    }

    #[test]
    fn word_art_display_intro() {
        let na = String::from("na");
        let args = AppEnv {
            location_images: na.clone(),
            location_ip_address: na.clone(),
            location_log: na.clone(),
            log_level: tracing::Level::INFO,
            rotation: 0,
            start_time: SystemTime::now(),
            timezone: EnvTimeZone::new(String::new()),
            ws_address: na.clone(),
            ws_apikey: na.clone(),
            ws_password: na.clone(),
            ws_token_address: na,
        };

        let result = display_intro(&args);
        assert!(!result.contains("!! DEBUG"));
        assert!(!result.contains("!! TRACE"));
    }
}
