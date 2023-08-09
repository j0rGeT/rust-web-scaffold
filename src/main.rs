use std::process::exit;
use rocketapi::server::init_server;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // start the server
    match init_server().await {
        Ok(server) => {
            server.launch().await;
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}
