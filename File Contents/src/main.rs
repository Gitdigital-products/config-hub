use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    key: String,
    value: String,
}

type ConfigStore = Arc<DashMap<String, String>>;

#[tokio::main]
async fn main() {
    let store: ConfigStore = Arc::new(DashMap::new());

    let app = Router::new()
        .route("/set", post(set_config))
        .route("/get/:key", get(get_config))
        .with_state(store.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4100));
    println!("⚙️ Config Hub running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(store))
        .await
        .unwrap();
}

async fn set_config(
    axum::Json(config): axum::Json<Config>,
    state: axum::extract::State<ConfigStore>,
) -> Json<serde_json::Value> {
    state.insert(config.key.clone(), config.value.clone());
    Json(json!({ "status": "set", "config": config }))
}

async fn get_config(
    Path(key): Path<String>,
    state: axum::extract::State<ConfigStore>,
) -> Json<serde_json::Value> {
    if let Some(value) = state.get(&key) {
        Json(json!({ "status": "found", "key": key, "value": value.clone() }))
    } else {
        Json(json!({ "status": "not found", "key": key }))
    }
}
