use color_mixer_service::run;
use std::env;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .map(|p| p.parse::<u16>().expect("Invalid value for PORT!"))
        .unwrap_or(8080);
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind address and port");
    run(listener)?.await
}
