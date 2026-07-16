use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use crate::store::backend::Backend;

pub enum Stream {
    Plain(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl Stream {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::Plain(s) => s.read(buf).await,
            Stream::Tls(s) => s.read(buf).await,
        }
    }
}

pub struct Connection<B: Backend> {
    stream: Stream,
    backend: Arc<B>,
}

impl<B: Backend + 'static> Connection<B> {
    pub fn new(stream: Stream, backend: Arc<B>) -> Self {
        Self { stream, backend }
    }
    
    pub async fn process(mut self) {
        let mut buf = vec![0; 4096];
        loop {
            match self.stream.read(&mut buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    tracing::debug!("Received {} bytes", n);
                    // In a complete implementation, we would decode BER, route operation and encode response
                }
                Err(e) => {
                    tracing::error!("Connection error: {:?}", e);
                    break;
                }
            }
        }
    }
}
