use serenity::{http::Http, http::HttpError, model::id::ChannelId, Error};

use crate::prototypes::MessageToChannel;

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

    pub async fn push_to_channel(&self, message: MessageToChannel) -> Result<(), HttpError> {
        match ChannelId::from(message.channel_id)
            .send_message(&self.http_client, |m| {
                m.content(message.content);
                m
            })
            .await
        {
            Err(e) => match e {
                Error::Http(http_error) => {
                    eprintln!("Http error here {:?}", http_error);
                    Err(*http_error)
                }
                _ => Ok(()),
            },
            _ => Ok(()),
        }
    }
}
