use fastldap::store::memory::MemoryBackend;
use fastldap::server::LdapServer;
use fastldap::store::ldif::load_ldif;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let backend = Arc::new(MemoryBackend::new());
    
    // Load some initial data
    let initial_ldif = "
dn: dc=example,dc=com
objectClass: top
objectClass: domain

dn: uid=testuser,dc=example,dc=com
objectClass: inetOrgPerson
objectClass: person
userPassword: password123
sn: User
cn: Test User
";
    load_ldif(&*backend, initial_ldif).await?;
    tracing::info!("Initial LDIF data loaded.");
    
    let server = LdapServer::new("0.0.0.0:389", backend, None).await?;
    server.run().await?;
    
    Ok(())
}
