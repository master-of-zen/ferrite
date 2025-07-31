use super::config::FerriteConfig;

fn main() {
    let config = FerriteConfig::default();
    println!(
        "{}",
        toml::to_string_pretty(&config).expect("Failed to serialize config")
    );
}
