use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use nexus_sdk::stwo::seq::Proof;
use nexus_sdk::{stwo::seq::Stwo, KnownExitCodes, Local, Prover, Verifiable, Viewable};
use postcard::{from_bytes, to_allocvec};
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

mod proving;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Runs the HTTP server.
    Server,
    /// Verifies a proof.
    Verify,
}

// Handle proof requests.
async fn handle_post(body: web::Bytes) -> impl Responder {
    match String::from_utf8(body.to_vec()) {
        Ok(package_string) => {
            println!(
                "Received proof request",
            );
            proving::prove_query(&package_string);
            HttpResponse::Ok().body(format!("Proof completed"))
        }
        Err(e) => {
            println!("Error converting payload to string: {}", e);
            HttpResponse::BadRequest().body("Invalid UTF-8 in request body")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server => {
            println!("Starting server at http://{}", SERVER_ADDRESS);

            HttpServer::new(|| {
                App::new()
                    .route("/", web::post().to(handle_post))
                    .route("/package", web::post().to(handle_post))
            })
            .bind(SERVER_ADDRESS)?
            .run()
            .await?;
        }
        Commands::Verify => {
            proving::verify_proof();
        }
    }
    Ok(())
}
