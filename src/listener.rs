use std::sync::{Arc, Mutex};
use basic_http::server::HttpServer;
use hub_channel::container::ChannelContainer;

pub struct RpcListener {
    socket: HttpServer,
}

impl RpcListener {
    pub fn new(ip: &str, port: u16, channel_container: Arc<Mutex<Arc<ChannelContainer>>>) -> Self {
        let mut server = HttpServer::new(ip, port, channel_container, None);
        server.append_handler("/", crate::handler::rpc_handler);
        Self { socket: server }
    }

    pub fn run(&self) {
        self.socket.bind();
    }
}