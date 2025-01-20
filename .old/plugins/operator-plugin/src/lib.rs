use thiserror::Error;

use tokio::sync::mpsc::{self, Receiver, Sender};

use futures::{FutureExt, future::Fuse};
pub struct ChannelInterface<TX, RX>
where
    TX: std::marker::Send + 'static,
    RX: std::marker::Send + 'static,
{
    rx: Receiver<RX>,
    tx: Sender<TX>,
}

impl<TX, RX> ChannelInterface<TX, RX>
where
    TX: std::marker::Send + 'static,
    RX: std::marker::Send + 'static,
{
    pub fn new(tx: Sender<TX>, rx: Receiver<RX>) -> Self {
        Self { rx, tx }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to send TX")]
    TxError(#[from] tokio::sync::mpsc::error::SendError<OperatorMessage>),
}

#[derive(Debug)]
pub struct OperatorMessage {
    pub message: String,
}

impl OperatorMessage {
    pub fn print(&self) {
        println!("{}", &self.message)
    }
}

pub struct OperatorInterface {
    interface: ChannelInterface<OperatorMessage, OperatorMessage>,
}

// Struct Management
impl OperatorInterface {
    pub fn new(tx: Sender<OperatorMessage>, rx: Receiver<OperatorMessage>) -> Self {
        Self {
            interface: ChannelInterface::new(tx, rx),
        }
    }
}

// Async functions
impl OperatorInterface {
    pub fn spawn() -> (Sender<OperatorMessage>, Receiver<OperatorMessage>) {
        let (ch1_tx, ch1_rx) = mpsc::channel(9);
        let (ch2_tx, ch2_rx) = mpsc::channel(9);

        let mut intf = Self::new(ch2_tx, ch1_rx);
        tokio::spawn(async move { intf.run().await });
        (ch1_tx, ch2_rx)
    }
}

pub async fn user_input() -> OperatorMessage {
    tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
    OperatorMessage {
        message: String::from("Test"),
    }
}

// Interface Functions
impl OperatorInterface {
    pub async fn rx(&mut self) -> Option<OperatorMessage> {
        let msg = self.interface.rx.recv().await;
        msg
    }
    pub async fn tx(tx: &Sender<OperatorMessage>, msg: OperatorMessage) -> Result<(), Error> {
        println!("TX: {:#?}", msg);
        Ok(tx.send(msg).await?)
    }
    pub async fn run(&mut self) -> Result<(), Error> {
        let tx_fut = Fuse::terminated();
        let tx = self.interface.tx.clone();
        tokio::pin!(tx_fut);
        loop {
            tokio::select! {
                msg = self.rx() => {
                    if let Some(msg) = msg {
                        msg.print();
                    }
                }
                _ = &mut tx_fut => {}
                input = user_input() => {
                    tx_fut.set(Self::tx(&tx, input).fuse());

                }
            }
        }
    }
}

// TODO:
// write the cli which will then operate on the tx and rx channels
