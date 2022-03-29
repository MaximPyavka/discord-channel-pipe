use tonic::transport::Server;
use log::info;

use discord_pipe::prototypes::DiscordPipeServer;

use env_logger;

use discord_pipe::discord_push_handler::DiscordBot;
use discord_pipe::utils::{get_discord_bot_token, get_service_socket};
use discord_pipe::service::DiscordPipeService;

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
