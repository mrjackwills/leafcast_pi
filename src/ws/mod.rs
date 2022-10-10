mod connect;
mod connection_details;
mod ws_sender;

use connect::ws_upgrade;
use connection_details::ConnectionDetails;
use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
    StreamExt, TryStreamExt,
};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex as TokioMutex};
use tokio_tungstenite::{self, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{error, info};

use crate::{camera::Camera, env::AppEnv};

use ws_sender::WSSender;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WSReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WSWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// Handle each incoming ws message
async fn incoming_ws_message(mut reader: WSReader, mut ws_sender: WSSender) {
    while let Ok(Some(message)) = reader.try_next().await {
        match message {
            Message::Text(message) => {
                let mut ws_sender = ws_sender.clone();
                tokio::spawn(async move {
                    ws_sender.on_text(message).await;
                });
            }
            Message::Close(_) => {
                tokio::time::timeout(std::time::Duration::from_secs(2), ws_sender.close())
                    .await
                    .unwrap_or(());
                break;
            }
            _ => (),
        };
    }
    info!("incoming_ws_message done");
}
// need to spawn a new receiver on each connect
/// try to open WS connection, and spawn a ThreadChannel message handler
pub async fn open_connection(app_envs: AppEnv, camera: Arc<TokioMutex<Camera>>) {
    let mut connection_details = ConnectionDetails::default();
    loop {
        info!("in connection loop, awaiting delay then try to connect");
        connection_details.reconnect_delay().await;

        match ws_upgrade(&app_envs).await {
            Ok(socket) => {
                info!("connected in ws_upgrade match");
                connection_details.valid_connect();

                let (writer, reader) = socket.split();
                let writer = Arc::new(Mutex::new(writer));

                let ws_sender = WSSender::new(
                    app_envs.clone(),
                    Arc::clone(&camera),
                    connection_details.get_connect_instant(),
                    writer,
                );
                incoming_ws_message(reader, ws_sender).await;
                info!("incoming_ws_message done, reconnect next");
            }
            Err(e) => {
                let connect_error = format!("{}", e);
                error!(%connect_error);
                connection_details.fail_connect();
            }
        }
    }
}
