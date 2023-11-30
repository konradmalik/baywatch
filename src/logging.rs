pub fn init(default_level: &str) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_level))
        .init();
}
