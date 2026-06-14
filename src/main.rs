mod config;
use config::Config;

fn main() {
    let app_config = Config::from_env();
    println!("server port : {}", app_config.database_url);
    println!("redis url : {}", app_config.redis_url);
    println!("server port : {}", app_config.server_port);

    let table = Config::table();
    println!("table {}", table);
}
