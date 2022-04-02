use serenity::{http::Http, http::HttpError, model::id::ChannelId, Error};

use crate::prototypes::MessageToChannel;

use log::error;

#[tonic::async_trait]
pub trait DiscordPusher: 'static + Sync + Send {
    async fn push_to_channel(&self, message: MessageToChannel) -> Result<(), HttpError>;
}

#[derive(Debug)]
pub struct DiscordBot {
    http_client: Http,
}

impl DiscordBot {
    pub fn new(token: String) -> Self {
        DiscordBot {
            http_client: Http::new_with_token(&token),
        }
    }
}

#[tonic::async_trait]
impl DiscordPusher for DiscordBot {
    async fn push_to_channel(&self, message: MessageToChannel) -> Result<(), HttpError> {
        match ChannelId::from(message.channel_id)
            .send_message(&self.http_client, |m| {
                m.content(message.content);
                m
            })
            .await
        {
            Err(e) => {
                error!(
                    "Error from discord trying to send message: {:#?}",
                    e.to_string()
                );
                match e {
                    Error::Http(http_error) => Err(*http_error),
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
