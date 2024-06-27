pub use macros::Config;

#[cfg(test)]
mod test {
    #[test]
    fn test_config() {
        #[derive(Debug, macros::Config)]
        struct Config {
            #[config(rename = "TEST")]
            test: i32,
            #[config(rename = "DOTENV")]
            dotenv: bool,
            test_automatic_rename: i32,
        }

        std::env::set_var("TEST", "42");
        std::env::set_var("TEST_AUTOMATIC_RENAME", "42");

        let config = Config::from_env().unwrap();
        assert_eq!(config.test, 42);
        assert_eq!(config.dotenv, true);
        assert_eq!(config.test_automatic_rename, 42);
    }

    #[test]
    fn test_config_error() {
        #[derive(Debug, macros::Config)]
        struct Config {
            #[config(rename = "TEST_UNSET")]
            _test: i32,
        }

        let config = Config::from_env();
        assert!(config.is_none());
    }
}
