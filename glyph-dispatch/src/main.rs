use operator_plugin::OperatorInterface;
use operator_plugin::OperatorMessage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operator Interface Error")]
    OperatorInterfaceError(#[from] operator_plugin::Error),
    #[error("Tokio Send Error {0}")]
    TokioSendError(#[from] tokio::sync::mpsc::error::SendError<OperatorMessage>),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (oi_tx, _) = OperatorInterface::spawn();
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        oi_tx
            .send(OperatorMessage {
                message: String::from("Test2"),
            })
            .await?;
    }
}
