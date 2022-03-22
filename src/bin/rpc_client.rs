use std::error::Error;
use std::time::Duration;
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

use std::fs;
use std::path::{PathBuf, Path};

use discord_pipe_service::discord_pipe_client::{DiscordPipeClient};
use discord_pipe_service::{Empty, MessageToChannel};

pub mod discord_pipe_service {
    tonic::include_proto!("discord_pipe");
}


async fn test_call(client: &mut DiscordPipeClient<Channel>) -> Result<(), Box<dyn Error>> {
    let message = MessageToChannel {channel_id: 1234, body: "KOKOKO".to_string()};
    
    let response = client
        .push_message(Request::new(message))
        .await?;

    println!("REPONSE {:#?}", response);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordPipeClient::connect("http://[::1]:10000").await?;
    test_call(&mut client).await?;
    Ok(())
}