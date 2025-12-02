use renet::{
    ConnectionConfig, DefaultChannel, RenetClient, RenetServer,
};
use std::time::{Duration, SystemTime};
use std::net::UdpSocket;
use matchbox_socket::WebRtcSocket;

pub const PROTOCOL_ID: u64 = 7;

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig::default()
}

use futures::future::BoxFuture;

pub fn setup_server() -> (RenetServer, WebRtcSocket, BoxFuture<'static, ()>) {
    let server = RenetServer::new(connection_config());
    let (socket, message_loop) = WebRtcSocket::new_reliable("ws://localhost:3536/room");
    
    let loop_fut = Box::pin(async move {
        let _ = message_loop.await;
    });
    
    (server, socket, loop_fut)
}

pub fn setup_client() -> (RenetClient, WebRtcSocket, BoxFuture<'static, ()>) {
    let client = RenetClient::new(connection_config());
    let (socket, message_loop) = WebRtcSocket::new_reliable("ws://localhost:3536/room");
    
    let loop_fut = Box::pin(async move {
        let _ = message_loop.await;
    });
    
    (client, socket, loop_fut)
}
