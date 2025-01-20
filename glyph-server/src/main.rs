use clap::Parser;
use counter_glyph::{Config as CounterGlyphConfig, Glyph as CounterGlyph};
use glyph::Glyphic;
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
    config_file: String,
}

struct GlyphServer {
    counter: CounterGlyph,
}

#[derive(Serialize, Deserialize, Parser)]
enum Commands {
    Test,
}

async fn process_socket<T>(socket: TcpStream, addr: SocketAddr) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    println!("Got connection from {:#?}", addr);

    loop {
        let ready = socket
            .ready(Interest::READABLE)
            .await
            .expect("socket ready failed");

        if ready.is_readable() {
            let mut data = vec![0; 1024];

            match socket.try_read(&mut data) {
                Ok(n) => {
                    let input = std::str::from_utf8(&data[0..n]).expect("not utf8");
                    if let Ok(parsed) = interpreter::interpret_glyphic(input) {
                        println!("GLYPHIC: {parsed:#?}");
                    } else {
                        println!("GLYPHIC: INVALID");
                        // TODO: for now, just drop the connection if we receive invalid glyphic syntax
                        return Err(Error::ProcessSocketError);
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // TODO - add some more graceful handling here maybe for unexpected disconnect
                    // this solves the problem, but doesn't allow established sessions to continue
                    return Err(Error::ProcessSocketError);
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return Err(Error::ProcessSocketError);
                }
            }
        }
    }
}

async fn run_net(listener: TcpListener) {
    loop {
        let (socket, addr) = listener.accept().await.expect("could not accept listener");
        process_socket::<Commands>(socket, addr).await;
    }
}

impl GlyphServer {
    pub async fn load_glyph_from_file<T>(file: &str) -> T
    where
        T: Glyphic,
        for<'de> <T as Glyphic>::Config: serde::Deserialize<'de>,
        <T as Glyphic>::Config: Debug,
    {
        let file_contents = fs::read_to_string(&file).expect("Could not read input file");
        let config: T::Config = toml::from_str(&file_contents).expect("Could not parse TOML file");
        T::load(config).await
    }

    pub async fn new() -> Self {
        Self {
            counter: Self::load_glyph_from_file("../glyphs/counter-glyph/glyph_config.toml").await,
        }
    }

    pub async fn run(self) {
        // start our tcp listener
        let listener = TcpListener::bind("127.0.0.1:8080")
            .await
            .expect("Failed to start TCP listener");
        let net = tokio::spawn(run_net(listener));

        let out = tokio::spawn(self.counter.run());

        let res = tokio::join!(out, net);
        println!("output: {:#?}", res);
    }
}

// let recipe: TomlRecipe = toml::from_str(&file).expect("Could not parse TOML file");

//

//     pub fn load(glyph: T: Glyphic) {}
// }

#[tokio::main]
async fn main() {
    let s = GlyphServer::new().await;
    s.run().await;
}
