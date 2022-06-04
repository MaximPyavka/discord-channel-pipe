pub mod discord_pipe_service {
    tonic::include_proto!("discord_pipe");
}

pub use discord_pipe_service::discord_pipe_server::{DiscordPipe, DiscordPipeServer};
pub use discord_pipe_service::discord_pipe_client::DiscordPipeClient;
pub use discord_pipe_service::{DiscordPushResult, MessageToChannel};

