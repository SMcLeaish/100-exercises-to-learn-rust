mod data;
mod store;

use store::TicketStore;
use tokio::net::TcpListener;

pub async fn ticket_server(
    listener: TcpListener,
    mut store: TicketStore,
) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
    }
}
