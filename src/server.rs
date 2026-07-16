use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use std::sync::Arc;
use crate::store::backend::Backend;
use crate::connection::{Connection, Stream};

pub struct LdapServer<B: Backend> {
    listener: TcpListener,
    tls_acceptor: Option<TlsAcceptor>,
    backend: Arc<B>,
}

impl<B: Backend + 'static> LdapServer<B> {
    pub async fn new(addr: &str, backend: Arc<B>, tls_acceptor: Option<TlsAcceptor>) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener, tls_acceptor, backend })
    }
    
    pub async fn run(&self) -> Result<(), std::io::Error> {
        tracing::info!("Listening on {}", self.listener.local_addr()?);
        loop {
            let (stream, addr) = self.listener.accept().await?;
            tracing::info!("Accepted connection from {}", addr);
            
            let backend = self.backend.clone();
            
            if let Some(ref acceptor) = self.tls_acceptor {
                let acceptor = acceptor.clone();
                tokio::spawn(async move {
                    match acceptor.accept(stream).await {
                        Ok(tls_stream) => {
                            let conn = Connection::new(Stream::Tls(tls_stream), backend);
                            conn.process().await;
                        }
                        Err(e) => {
                            tracing::error!("TLS handshake failed: {:?}", e);
                        }
                    }
                });
            } else {
                let conn = Connection::new(Stream::Plain(stream), backend);
                tokio::spawn(async move {
                    conn.process().await;
                });
            }
        }
    }
}
