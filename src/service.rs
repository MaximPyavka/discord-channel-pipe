use tonic::{Request, Response, Status};

use crate::prototypes::{DiscordPushResult, MessageToChannel, DiscordPipe};
use crate::discord_push_handler::DiscordPusher;

#[derive(Debug)]
pub struct DiscordPipeService<T: DiscordPusher> {
    pub discord_bot: T,
}

#[tonic::async_trait]
impl<T: DiscordPusher> DiscordPipe for DiscordPipeService<T> {
    async fn push_message(
        &self,
        request: Request<MessageToChannel>,
    ) -> Result<Response<DiscordPushResult>, Status> {
        let error_message = match self.discord_bot.push_to_channel(request.into_inner()).await {
            Err(e) => Some(e.to_string()),
            _ => None,
        };

        Ok(Response::new(DiscordPushResult { error_message }))
    }
}
