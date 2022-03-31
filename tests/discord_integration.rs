use discord_pipe::discord_push_handler::DiscordPusher;
use discord_pipe::prototypes::{
    DiscordPipeClient, DiscordPipeServer, DiscordPushResult, MessageToChannel,
};
use discord_pipe::service::DiscordPipeService;
use std::time::Duration;
use tokio::sync::oneshot::{self, Receiver};
use tokio::task::JoinHandle;
use tonic::transport::{Channel, Server};
use tonic::Request;

use serenity::FutureExt;

use serenity::http::HttpError;

async fn start_server<T: DiscordPusher>(
    discord_bot: T,
    rx: Receiver<()>,
    port: &'static str,
) -> JoinHandle<()> {
    let grpc_discord_service = DiscordPipeService { discord_bot };
    let svc = DiscordPipeServer::new(grpc_discord_service);
    let server_host = format!("127.0.0.1:{}", port);

    let jq = tokio::spawn(async move {
        Server::builder()
            .add_service(svc)
            .serve_with_shutdown(server_host.parse().unwrap(), rx.map(drop))
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    jq
}

#[tokio::test]
async fn test_discord_returns_fails_with_error() {
    struct ErrorDiscordBot {}

    #[tonic::async_trait]
    impl DiscordPusher for ErrorDiscordBot {
        async fn push_to_channel(&self, message: MessageToChannel) -> Result<(), HttpError> {
            assert_eq!(
                message,
                MessageToChannel {
                    channel_id: 12345,
                    content: "aaaa".to_string()
                }
            );
            Err(HttpError::RateLimitUtf8)
        }
    }

    let (tx, rx) = oneshot::channel::<()>();

    let jq = start_server(ErrorDiscordBot {}, rx, "1337").await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Client init
    let mut channel = DiscordPipeClient::connect("http://127.0.0.1:1337")
        .await
        .unwrap();

    let push_result = discord_return(&mut channel).await;
    assert_eq!(
        Some("Error decoding a header from UTF-8".to_owned()),
        push_result.error_message
    );

    tx.send(()).unwrap();
    jq.await.unwrap();
}

#[tokio::test]
async fn test_discord_succeeds() {
    struct OkDiscordBot {}

    #[tonic::async_trait]
    impl DiscordPusher for OkDiscordBot {
        async fn push_to_channel(&self, message: MessageToChannel) -> Result<(), HttpError> {
            assert_eq!(
                message,
                MessageToChannel {
                    channel_id: 12345,
                    content: "aaaa".to_string()
                }
            );
            Ok(())
        }
    }

    let (tx, rx) = oneshot::channel::<()>();

    let jq = start_server(OkDiscordBot {}, rx, "1338").await;
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Client init
    let mut channel = DiscordPipeClient::connect("http://127.0.0.1:1338")
        .await
        .unwrap();

    let push_result = discord_return(&mut channel).await;
    assert!(push_result.error_message.is_none());

    tx.send(()).unwrap();
    jq.await.unwrap();
}

async fn discord_return(client: &mut DiscordPipeClient<Channel>) -> DiscordPushResult {
    let message = MessageToChannel {
        channel_id: 12345,
        content: "aaaa".to_string(),
    };
    let response = client.push_message(Request::new(message)).await.unwrap();
    response.into_inner()
}
