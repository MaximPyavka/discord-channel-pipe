use std::{env, str::FromStr};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    model::id::ChannelId,
    http::Http,
    prelude::*,
};

use serde_json::{Value, Map, to_vec};

struct Handler;

pub async fn push_to_channel(channel_id: u64, message: String) {
    let token = "discord-bot-token".to_string();

    let mut client =
        Client::builder(&token).await.expect("Err creating client");

    let http_client = Http::new_with_token(&token);

    let channel = ChannelId(channel_id);

    let resp = channel.send_message(&http_client, |m| {
        m.content("test");

        m.embed(|mut e| {
            e.title("This is an embed");
            e.description("With a description");

            e
        });

        m
    }).await;

    if let Err(reason) = resp {
            println!("ERROR WHY {:?}", reason);
        }
}