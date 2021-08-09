#![allow(unused_imports)]
use crate::config::{load_config, parse_ini, Config, LogLevel, ServerMode};
use std::collections::HashMap;

#[test]
fn test_parse_ini() {
    let testcase =
        "[s1]\nkey1=value \nkey2 = value2   ; comment\nkey3 = \"value\"\n\n[s2]\nkey1=value";

    let mut expected_hashmap: HashMap<String, String> = HashMap::new();
    expected_hashmap.insert("s1.key1".into(), "value".into());
    expected_hashmap.insert("s1.key2".into(), "value2".into());
    expected_hashmap.insert("s1.key3".into(), "value".into());
    expected_hashmap.insert("s2.key1".into(), "value".into());

    let hashmap = parse_ini(testcase).unwrap();
    assert_eq!(hashmap, expected_hashmap);
}

#[test]
fn test_parse_config() {
    let config_string = r#"
; Humphrey Configuration File

[server]
address = "127.0.0.1"      ; address to host the server on
port = 8000                ; port to host the server on
mode = "static"            ; server routing mode

[log]
level = "info"
file = "humphrey.log"

[static]
directory = "/var/www"
cache = 128M
cache_time = 60"#;

    let config = load_config(Some(config_string.into())).unwrap();
    let expected_config = Config {
        address: "127.0.0.1".into(),
        port: 8000,
        mode: ServerMode::Static,
        log_level: LogLevel::Info,
        log_file: Some("humphrey.log".into()),
        cache_limit: 128 * 1024 * 1024,
        cache_time_limit: 60,
        directory: Some("/var/www".into()),
        proxy_target: None,
        load_balancer_targets: None,
        load_balancer_mode: None,
    };

    assert_eq!(config, expected_config);
}
