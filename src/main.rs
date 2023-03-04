use sysinfo::{CpuExt, System, SystemExt};
use axum::{Router, Server, routing::get, extract::State};
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .with_state(AppState {sys: Arc::new(Mutex::new(System::new())) });
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
     use std::fmt::Write;
     let mut s = String::new();
     let mut sys = state.sys.lock().unwrap();
     sys.refresh_cpu();
     for (i,cpu) in sys.cpus().iter().enumerate() {
         let i = i + 1;
         writeln!(&mut s, "CPU {i} {}%", cpu.cpu_usage()).unwrap();
     }
     s
 }
