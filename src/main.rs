use sysinfo::{CpuExt, System, SystemExt};
use axum::{Router, Server, routing::get, extract::State, Json, response::IntoResponse};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app_state = AppState::default();
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpu_get))
        .with_state(app_state.clone());

    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);

        }
    });
    

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Address of server is on {addr}");
    server.await.unwrap();
}

#[derive(Default, Clone)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}


 async fn root_get() -> String {
     let mut sys = System::new();
     sys.refresh_all();
     let hostname = sys.host_name().unwrap();
     hostname
 }

 async fn cpu_get(State(state): State<AppState>) -> impl IntoResponse {
     let v = state.cpus.lock().unwrap().clone();
     Json(v)
 }
