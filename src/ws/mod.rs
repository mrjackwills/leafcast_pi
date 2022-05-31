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
use tracing::{error, info, trace};

use crate::{camera::Camera, env::AppEnv};

use ws_sender::WSSender;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WSReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type WSWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

/// handle each incoming ws message
async fn incoming_ws_message(mut reader: WSReader, ws_sender: WSSender) {
    loop {
        let mut ws = ws_sender.clone();

        // server sends a ping every 30 seconds, so just wait 45 seconds for any message, if not received then break
        let message_timeout =
            tokio::time::timeout(std::time::Duration::from_secs(45), reader.try_next()).await;

        match message_timeout {
            Ok(some_message) => match some_message {
                Ok(Some(m)) => {
                    tokio::spawn(async move {
                        match m {
                            m if m.is_close() => ws.close().await,
                            m if m.is_text() => ws.on_text(m.to_string().as_str()).await,
                            m if m.is_ping() => ws.ping().await,
                            _ => (),
                        };
                    });
                }
                Ok(None) => {
                    error!("None in incoming_ws_message");
                    ws.close().await;
                    break;
                }
                Err(e) => {
                    error!(%e);
                    error!("Error in incoming_ws_message");
                    ws.close().await;
                    break;
                }
            },
            Err(_) => {
                trace!("timeout error");
                ws.close().await;
                break;
            }
        }
    }
}

// need to spawn a new receiver on each connect
/// try to open WS connection, and spawn a ThreadChannel message handler
pub async fn open_connection(app_envs: AppEnv, camera: Arc<TokioMutex<Camera>>) {
    let mut connection_details = ConnectionDetails::new();
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
