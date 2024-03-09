use crate::ftp_server::ftp_auth::PMAuthenticator;
use crate::ftp_server::ftp_backend::Vfs;


async fn start_ftp_server(){
    let server = libunftp::Server::with_authenticator(
        Box::new(|| Vfs::new()),
        std::sync::Arc::new(PMAuthenticator{})
    )
    .greeting("Welcome to your PMCloud FTP file server")
    .passive_ports(50000..65535);

    server.listen("127.0.0.1:2121").await;
}


