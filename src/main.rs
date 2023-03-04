use sysinfo::{CpuExt, System, SystemExt};
use axum::{Router, Server, routing::get, extract::State, Json, response::IntoResponse};
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpu_get))
        .with_state(AppState {sys: Arc::new(Mutex::new(System::new_all())) });
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Address of server is on {addr}");
    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}


 async fn root_get(State(state): State<AppState>) -> String {
     let mut sys = state.sys.lock().unwrap();
     sys.refresh_all();
     let hostname = sys.host_name().unwrap();
     hostname
 }

 async fn cpu_get(State(state): State<AppState>) -> impl IntoResponse {
     let mut sys = state.sys.lock().unwrap();
     sys.refresh_cpu();

     let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

     Json(v)
 }
