
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use axum::Router;
use axum::routing::{get};
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{ Pool, Postgres, query};
use tokio::sync::RwLock;
use crate::config::{AppConfig, WebState};
use crate::core::tsorfib_tun;

mod core;
mod config;


#[tokio::main]
async  fn main()  {
    config::init_logger();
    let app_config = config::config();
    
    // tsorfib_tun::connect(app_config.clone()).await;
    tsorfib_tun::new_peer(app_config.clone()).await;
    log::info!("starting tsorfib ...........");




    // let cors = config::allow_cors();
    let pool = config::postgres(app_config.clone()).await;
    let cache = config::dragonfly(app_config.clone()).await;
    let app_state = Arc::new(WebState {
        session_store: Arc::new(RwLock::new(HashMap::new())), // 初始化空的会话存储
        pool,
        app_config: app_config.clone(),
        cache
    });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    let listen_addr = "0.0.0.0:".to_string() + &app_config.port.to_string();
    // run our app with hyper, listening globally on port 3000
    info!("bifrost has been started Listening on   http://{}",listen_addr);

    // Bind and serve the app using Axum's Server
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await;
}


async fn root() -> &'static str {
    "Bifrost"
}