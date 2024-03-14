use std::sync::Arc;

use tokio::net::{TcpListener, UdpSocket};

use crate::cipher::RsaCipher;
use crate::core::service::PacketHandler;
use crate::core::store::cache::AppCache;
use crate::ConfigInfo;

mod tcp;
mod udp;

pub async fn start(
    udp: Arc<UdpSocket>,
    tcp: TcpListener,
    config: ConfigInfo,
    rsa_cipher: Option<RsaCipher>,
) {
    let cache = AppCache::new();
    let handler = PacketHandler::new(cache, config, rsa_cipher);
    tcp::start(udp.clone(), tcp, handler.clone()).await;
    udp::start(udp, handler.clone()).await;
}
