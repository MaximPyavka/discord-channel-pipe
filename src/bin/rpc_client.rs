use std::error::Error;
use std::time::Duration;
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

use std::fs;
use std::path::{PathBuf, Path};

use discord_pipe::prototypes::{MessageToChannel, Empty, DiscordPipeClient};


async fn test_call(client: &mut DiscordPipeClient<Channel>) -> Result<(), Box<dyn Error>> {
    let message = MessageToChannel {channel_id: 123134, content: "{
        \"Title\": \"New order\", \"Name\": \"TGTG\"}".to_string()};
    
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