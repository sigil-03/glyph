use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::net::SocketAddr;
use thiserror::Error;
use tokio::io::{ErrorKind, Interest};
use tokio::net::{TcpListener, TcpStream};

#[derive(Error, Debug)]
enum Error {
    #[error("Error processing socket")]
    ProcessSocketError,
}

#[derive(Parser)]
struct LaunchOptions {
    #[arg(short, long)]
    glyph_server: String,
    #[command(subcommand)]
    mode: LaunchMode,
}

#[derive(Subcommand)]
enum LaunchMode {
    File{input_file: String},
    Interactive,   
}

struct GlyphClient {

}

impl GlyphClient {
    // pub async fn load_glyph_from_file<T>(file: &str) -> T
    // where
    //     T: Glyphic,
    //     for<'de> <T as Glyphic>::Config: serde::Deserialize<'de>,
    //     <T as Glyphic>::Config: Debug,
    // {
    //     let file_contents = fs::read_to_string(&file).expect("Could not read input file");
    //     let config: T::Config = toml::from_str(&file_contents).expect("Could not parse TOML file");
    //     T::load(config).await
    // }

    pub async fn new() -> Self {
        Self {
        }
    }

    pub async fn run(self) {
        // start our tcp listener
        let net = tokio::spawn(run_net());
        let res = tokio::join!(net);
        println!("output: {:#?}", res);
    }
}

// async fn process_socket<T>(socket: TcpStream, addr: SocketAddr) -> Result<T, Error>
// where
//     for<'de> T: Deserialize<'de>,
// {
//     println!("Got connection from {:#?}", addr);

//     loop {
//         let ready = socket
//             .ready(Interest::READABLE)
//             .await
//             .expect("socket ready failed");

//         if ready.is_readable() {
//             let mut data = vec![0; 1024];

//             match socket.try_read(&mut data) {
//                 Ok(n) => {
//                     let input = std::str::from_utf8(&data[0..n]).expect("not utf8");
//                     if let Some(parsed) = interpreter::interpret_glyphic(std::str::from_utf8(&data[0..n]).expect("not utf8")){
//                         println!("GLYPHIC: {parsed:#?}");
//                     } else {
//                         println!("GLYPHIC: INVALID");
//                     }
                        
//                 }
//                 Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
//                     // TODO - add some more graceful handling here maybe for unexpected disconnect
//                     // this solves the problem, but doesn't allow established sessions to continue
//                     return Err(Error::ProcessSocketError);
//                 }
//                 Err(e) => {
//                     println!("Error: {:#?}", e);
//                     return Err(Error::ProcessSocketError);
//                 }
//             }
//         }
//     }
// }

async fn run_net() {
    // open socket
    let socket = TcpStream::connect("127.0.0.1:8080").await.expect("Failed to open TCP socket");

    loop {
        let ready = socket
            .ready(Interest::WRITABLE)
            .await
            .expect("socket ready failed");

        if ready.is_writable() {
        }
    }
}


// let recipe: TomlRecipe = toml::from_str(&file).expect("Could not parse TOML file");

//

//     pub fn load(glyph: T: Glyphic) {}
// }

#[tokio::main]
async fn main() {
    let s = GlyphClient::new().await;
    s.run().await;
}
