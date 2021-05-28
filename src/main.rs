//! src/main.rs;

use email_rust::startup::run;
use std::net::TcpListener;
use email_rust::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we can't read the config file
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Have removed hard coded 8000 - not coming from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
