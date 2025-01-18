use futures::{FutureExt, future::Fuse};
use operator_plugin::OperatorInterface;
use operator_plugin::OperatorMessage;
use serde::Deserialize;
use thiserror::Error;
use tokio::io::{Stdin, Stdout};
use tokio::{self, select};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use url::Url;

use clap::Parser;

use serde::Serialize;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operator Interface Error")]
    OperatorInterfaceError(#[from] operator_plugin::Error),
    #[error("Tokio Send Error {0}")]
    TokioSendError(#[from] tokio::sync::mpsc::error::SendError<OperatorMessage>),
}

#[derive(Parser)]
struct LaunchOptions {
    #[arg(short, long)]
    target_url: Url,
    #[arg(short, long)]
    prompt_file: String,
}

struct Cli {
    input: String,
}

struct UserCli {
    reader: tokio::io::BufReader<Stdin>,
    writer: tokio::io::BufWriter<Stdout>,
}

// let mut reader = tokio::io::BufReader::new(tokio::io::stdin());

impl UserCli {
    fn new() -> Self {
        Self {
            reader: tokio::io::BufReader::new(tokio::io::stdin()),
            writer: tokio::io::BufWriter::new(tokio::io::stdout()),
        }
    }
    async fn user_input(&mut self) -> Cli {
        let mut buffer = Vec::new();

        self.writer
            .write_all("> ".as_bytes())
            .await
            .expect("failed to write to stdout buffer");
        self.writer
            .flush()
            .await
            .expect("failed to flush stdout buffer");

        self.reader
            .read_until(b'\n', &mut buffer)
            .await
            .expect("Failed to get input");
        Cli {
            input: String::from_utf8(buffer).expect("failed to convert buffer"),
        }
    }
}

#[derive(Serialize)]
struct LlamaServerRequest {
    prompt: String,
    n_predict: usize,
}
#[derive(Deserialize)]
struct LlamaServerResponse {
    content: String,
}

// template
// <|im_start|>system<|im_sep|>You are a helpful assistant<|im_end|><|im_start|>user<|im_sep|>Hello<|im_end|><|im_start|>assistant<|im_sep|>
fn generate_prompt(system_prompt: &str, user_prompt: &str) -> String {
    let out = format!(
        "<|im_start|>system<|im_sep|>{system_prompt}<|im_end|><|im_start|>user<|im_sep|>{user_prompt}<|im_end|><|im_start|>assistant<|im_sep|>"
    );
    out
}

fn load_prompt_from_file(file: &str) -> String {
    std::fs::read_to_string(file).expect("Could not read input file")
}

struct GlyphDispatch {
    system_prompt: String,
    target_url: Url,
    client: reqwest::Client,
}
impl GlyphDispatch {
    fn new(config: LaunchOptions) -> Self {
        Self {
            system_prompt: load_prompt_from_file(&config.prompt_file),
            target_url: config.target_url,
            client: reqwest::Client::new(),
        }
    }
    async fn handle_user_input(&self, input: Cli) {
        let req = LlamaServerRequest {
            prompt: generate_prompt(&self.system_prompt, &input.input),
            n_predict: 128,
        };
        println!("URL: {}", self.target_url.as_str());
        let res = self
            .client
            .post(format!("{}completion", self.target_url.as_str()))
            .json(&req)
            .send()
            .await
            .expect("failed to handle user input");
        let res: LlamaServerResponse =
            serde_json::from_str(&res.text().await.expect("failed to get resp text"))
                .expect("failed to parse input");
        println!(">> {}", res.content);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let (_oi_tx, mut oi_rx) = OperatorInterface::spawn();
    // let oi_rx_fut = oi_rx.recv().fuse();
    // // let glyph_rx_fut
    // tokio::pin!(rx_fut);
    let config = LaunchOptions::parse();
    let mut user_cli = UserCli::new();
    let glyph_dispatch = GlyphDispatch::new(config);

    loop {
        select! {
            input = user_cli.user_input() => {
                glyph_dispatch.handle_user_input(input).await;
            }
        }
        // select! {
        //     Some(msg) = &mut rx_fut => {
        //         msg.print();
        //     }
        // }
        // tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        // oi_tx
        //     .send(OperatorMessage {
        //         message: String::from("Test2"),
        //     })
        //     .await?;
    }
}
