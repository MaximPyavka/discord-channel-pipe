use std::{env, str::FromStr};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    model::id::ChannelId,
    http::Http,
    prelude::*,
};

use serde_json::{Value, Map, to_vec};

mod constants;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        println!("MESSAGE!!!!");
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            let channel = msg.channel_id;
            println!("CHANNEL ID {}", channel);
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = "OTUzMzE3MzE2ODE1Mzg4NzMz.YjCz8A.k9kzyU__tWZRPN-_37izyDopX3Y".to_string();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    let http_client = Http::new_with_token(&token);
    let channel_id = u64::from_str("953211161095045170").unwrap();


    let channel_id = ChannelId(channel_id);

    let resp = channel_id.send_message(&http_client, |m| {
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
    



    // let mut obj = Map::with_capacity(10);
    // obj.insert("HI".to_string(), Value::String("Everyone".to_owned()));
    // println!("OBJ {:?}", obj);
    // let message = Value::Object(obj.clone());
    // let vec_mess = to_vec(&message).expect("VEC");
    // println!("VECTOR IS HERE {:?}", vec_mess);
    // let status = http_client.send_message(channel_id, &Value::Object(obj)).await;
    // if let Err(reason) = status {
    //     println!("ERROR WHY {:?}", reason);
    // }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}