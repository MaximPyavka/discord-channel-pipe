use tonic::transport::Server;
use tonic::{Request, Response, Status};

use std::net::{SocketAddr, Ipv4Addr, IpAddr};

use discord_pipe::prototypes::{DiscordPipe, DiscordPipeServer, Empty, MessageToChannel};

use discord_pipe::discord_push_handler::channel_handle;
use discord_pipe::utils::get_service_socket;

use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub struct DiscordPipeService {
    message_sender: Sender<MessageToChannel>,
}

#[tonic::async_trait]
impl DiscordPipe for DiscordPipeService {
    async fn push_message(
        &self,
        request: Request<MessageToChannel>,
    ) -> Result<Response<Empty>, Status> {
        let sender = self.message_sender.clone();
        tokio::spawn(async move {
            let send_to_channel = sender.send(request.into_inner()).await;
        });

        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = get_service_socket();

    let (tx, rx) = channel::<MessageToChannel>(10);
    let message_service = DiscordPipeService { message_sender: tx };
    let svc = DiscordPipeServer::new(message_service);

    let discord_ch_handle = channel_handle(rx);
    let server_run = Server::builder().add_service(svc).serve(addr);

    let (_, _) = tokio::join!(server_run, discord_ch_handle);

    Ok(())
}
