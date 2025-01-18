// use thiserror::Error;

use tokio::sync::mpsc::{self, Receiver, Sender};
// use tokio::self;

// use futures::{future::Fuse, FutureExt};

/// The interface handle which will get sent to others (or ourself)
/// that would like to communicate with this channel.
pub struct InterfaceHandle<TX, RX>
where
    TX: std::marker::Send + 'static,
    RX: std::marker::Send + 'static,
{
    tx: Sender<TX>,
    rx: Receiver<RX>,
}

impl<TX, RX> InterfaceHandle<TX, RX>
where
    TX: std::marker::Send + 'static,
    RX: std::marker::Send + 'static,
{
    pub fn new(tx: Sender<TX>, rx: Receiver<RX>) -> Self {
        Self { tx, rx }
    }
}

/// An interface is attached to a node.
/// TX and RX directions are always referred to from the node's perspective
pub trait Interface {
    type TxMsg: std::marker::Send + 'static;
    type RxMsg: std::marker::Send + 'static;

    fn init_interface(
        depth: usize,
    ) -> (
        InterfaceHandle<Self::TxMsg, Self::RxMsg>,
        InterfaceHandle<Self::RxMsg, Self::TxMsg>,
    );
}

pub struct TestTxMessage {
    message: String,
}

pub struct TestRxMessage {
    message: String,
}

pub struct TestNode {
    interface: InterfaceHandle<TestTxMessage, TestRxMessage>,
}

impl TestNode {
    fn new(interface: InterfaceHandle<TestTxMessage, TestRxMessage>) -> Self {
        Self {
            interface
        }
    }
    pub async fn spawn() -> InterfaceHandle<TestRxMessage, TestTxMessage> {
        let (internal_handle, external_handle) = Self::init_interface(9);
        let node = TestNode::new(internal_handle);
        tokio::spawn(async move {node.run()});
        external_handle
    }
    fn run(self) {
        tokio::time::sleep(tokio::time::Duration::from_millis(3000));
        println!("Running");
    }
}

impl Interface for TestNode {
    type TxMsg = TestTxMessage;
    type RxMsg = TestRxMessage;

    fn init_interface(
        depth: usize,
    ) -> (
        InterfaceHandle<Self::TxMsg, Self::RxMsg>,
        InterfaceHandle<Self::RxMsg, Self::TxMsg>,
    ) {
        let (ch1_tx, ch1_rx) = mpsc::channel(depth);
        let (ch2_tx, ch2_rx) = mpsc::channel(depth);

        let int_handle = InterfaceHandle::<Self::TxMsg, Self::RxMsg>::new(ch1_tx, ch2_rx);
        let ext_handle = InterfaceHandle::<Self::RxMsg, Self::TxMsg>::new(ch2_tx, ch1_rx);
        (int_handle, ext_handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn interface_init() {
        let interface = TestNode::spawn();
        tokio::time::sleep(tokio::time::Duration::from_millis(9000)).await;
    }
}
