use dotenv::dotenv;

pub fn load_env() {
    if dotenv().is_err() {
        dotenv::from_filename("services/auth-service/.env").ok();
    }

    if std::env::var("DATABASE_URL").is_err() {
        eprintln!("ERROR: DATABASE_URL environment variable is not set!");
        std::process::exit(1);
    }
}
