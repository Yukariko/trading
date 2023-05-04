use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let app_key = std::env::var("APP_KEY").expect("APP_KEY must be set.");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set.");
    println!("{}, {}", app_key, app_secret);
}
