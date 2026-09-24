use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use fern::Dispatch;
use log::info;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::RwLock;
use tower_http::cors::{AllowMethods, Any, CorsLayer};
extern crate r2d2;
use r2d2_redis::RedisConnectionManager;

#[derive(Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub db_url: String,
    pub max_connect_num: u32,
    pub port: u32,
    pub wg_private_key: String,
    pub wg_public_key: String,
    pub wg_endpoint: String,
    pub tsorfib_endpoint: String,
    pub iframe_name: String,
    pub iframe_port: String,
}


#[derive(Clone)]
pub struct WebState {
    pub session_store: Arc<RwLock<HashMap<String, i32>>>,
    pub pool: Pool<Postgres>,// 存储会话数据
    pub app_config: AppConfig,
    pub cache: r2d2::Pool<RedisConnectionManager>
}


pub async fn postgres(app_config: AppConfig) ->Pool<Postgres>{
     PgPoolOptions::new()
        .max_connections(app_config.max_connect_num)
        .connect(app_config.db_url.as_str())
        .await
        .expect("Failed to connect to the database")
}


pub async fn dragonfly(app_config: AppConfig)->r2d2::Pool<RedisConnectionManager>{
    let manager = r2d2_redis::RedisConnectionManager::new("redis://localhost").unwrap();
     r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap()

}

pub fn config() -> AppConfig {
    dotenv().ok();
    info!("loading env...");
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let app_name = env::var("APPNAME")
        .expect("APPNAME must be set in the .env file");

    let max_connect_num:u32 = env::var("DATABASE_MAX_CONNECTIONS").unwrap()
        .parse()
        .expect("DATABASE_MAX_CONNECTIONS must be set in the .env file");

    let port:u32 = env::var("PORT").unwrap()
        .parse()
        .expect("PORT must be set in the .env file");

    let cache_url = env::var("CACHE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let wg_private_key = env::var("WG_PRIVATE_KEY")
        .expect("WG_PRIVATE_KEY must be set in the .env file");

    let wg_public_key = env::var("WG_SERVER_PUBLIC_KEY")
        .expect("WG_SERVER_PUBLIC_KEY must be set in the .env file");

    let wg_endpoint = env::var("WG_SERVER_ENDPOINT")
        .expect("DATABASE_URL must be set in the .env file");

    let tsorfib_endpoint = env::var("TSORFIB_ENDPOINT")
        .expect("DATABASE_URL must be set in the .env file");

    let iframe_name = env::var("IFRAME_NAME")
        .expect("DATABASE_URL must be set in the .env file");

    let iframe_port = env::var("IFRAME_PORT")
        .expect("IFRAME_PORT must be set in the .env file");

    info!("env loaded env...");
    AppConfig {
        db_url,
        max_connect_num,
        app_name,
        port,
        wg_private_key,
        wg_public_key,
        wg_endpoint,
        tsorfib_endpoint,
        iframe_name,
        iframe_port
    }

}



// pub fn allow_cors() -> CorsLayer{
//     CorsLayer::new()
//         .allow_origin(Any) // 允许所有来源
//         .allow_methods(
//             AllowMethods::list(vec![
//                 http::Method::GET,    // http::Method::GET instead of Method::GET
//                 http::Method::POST,   // http::Method::POST instead of Method::POST
//                 http::Method::PUT,    // http::Method::PUT instead of Method::PUT
//                 http::Method::DELETE, // http::Method::DELETE instead of Method::DELETE
//             ])
//         ).allow_headers(Any)
// }


pub fn init_logger() -> Result<(), fern::InitError> {
    let log_file = "logs/app.log";

    // 创建日志记录器
    Dispatch::new()
        // 设置输出到文件
        .chain(fern::log_file(log_file)?)
        // 设置输出到标准输出
        .chain(fern::Output::call(|record| {
            println!("{}", record.args());
        }))
        // 设置日志级别
        .level(log::LevelFilter::Info)
        .apply()?;

    Ok(())
}