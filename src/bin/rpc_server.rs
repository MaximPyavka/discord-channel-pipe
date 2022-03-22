use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use tonic::transport::Server;
use tonic::{Request, Response, Status};

use discord_pipe_service::discord_pipe_server::{DiscordPipe, DiscordPipeServer};
use discord_pipe_service::{Empty, MessageToChannel};

pub mod discord_pipe_service {
    tonic::include_proto!("discord_pipe");
}

use discord_channel_pusher::discord_push_handler::push_to_channel;


#[derive(Debug)]
pub struct DiscordPipeService {}

#[tonic::async_trait]
impl DiscordPipe for DiscordPipeService {
    async fn push_message(
        &self,
        request: Request<MessageToChannel>,
    ) -> Result<Response<Empty>, Status> {
        // println!("GOT REQUEST {:#?}", request);

        let request_message = request.into_inner();
        println!("MESSAGE {:#?}", request_message);

        let message = "KOKOKO".to_string();
        push_to_channel(12345, message).await;


        Ok(Response::new(Empty {}))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    println!("RouteGuideServer listening on: {}", addr);

    let message_service = DiscordPipeService{};
    let svc = DiscordPipeServer::new(message_service);
    // let streeeem: ReceiverStream<_> = rx.into();

    let grpc_server_launch = Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
