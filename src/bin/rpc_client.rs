use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

use discord_pipe::prototypes::{MessageToChannel, DiscordPipeClient};
use discord_pipe::utils::{get_client_endpoint, get_channel_id};


async fn test_call(client: &mut DiscordPipeClient<Channel>) -> Result<(), Box<dyn Error>> {
    let message = MessageToChannel {channel_id: get_channel_id(), content: "{
        \"Title\": \"New order\", \"Name\": \"TGTG\"}".to_string()};
    
    let response = client
        .push_message(Request::new(message))
        .await;

    println!("RESPONSE {:?}", response);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_addr = get_client_endpoint();
    let mut client = DiscordPipeClient::connect(service_addr).await?;
    test_call(&mut client).await?;
    Ok(())
}