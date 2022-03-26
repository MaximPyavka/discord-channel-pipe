use tonic::transport::Server;
use tonic::{Request, Response, Status};
use log::info;

use discord_pipe::prototypes::{
    DiscordPipe, DiscordPipeServer, DiscordPushResult, MessageToChannel,
};

use env_logger;

use discord_pipe::discord_push_handler::DiscordBot;
use discord_pipe::utils::{get_discord_bot_token, get_service_socket};

#[derive(Debug)]
pub struct DiscordPipeService {
    discord_bot: DiscordBot,
}

#[tonic::async_trait]
impl DiscordPipe for DiscordPipeService {
    async fn push_message(
        &self,
        request: Request<MessageToChannel>,
    ) -> Result<Response<DiscordPushResult>, Status> {
        let error_message = match self.discord_bot.push_to_channel(request.into_inner()).await {
            Err(e) => { Some(e.to_string()) },
            _ => {None}
        };

        Ok(Response::new(DiscordPushResult {
            error_message
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = get_service_socket();

    info!("Started Discord Pipe rpc server on port {:?}", addr.port());

    let discord_bot = DiscordBot::new(get_discord_bot_token());
    let message_service = DiscordPipeService { discord_bot };
    let svc = DiscordPipeServer::new(message_service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
