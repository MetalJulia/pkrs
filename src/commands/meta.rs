use crate::ShardManagerContainer;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, CommandResult},
};
use std::time::Instant;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg
                .reply(ctx, "I couldn't acquire the shard manager to do that.")
                .await;
            return Ok(());
        }
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(ctx, "I couldn't find my own shard 🤔");
            return Ok(());
        }
    };
    let latency = runner.latency.map_or(None, |l| Some(l.as_millis()));
    let reply = latency.map_or_else(
        || "Pong! The shard latency is unavailable. The message latency is".to_string(),
        |l| {
            format!(
                "Pong! The shard latency is `{}ms`. The message latency is",
                l
            )
        },
    );
    let msg_latency_start = Instant::now();
    let mut msg = msg.channel_id.say(ctx, reply.clone()).await?;
    let msg_latency = msg_latency_start.elapsed();
    let _ = msg
        .edit(&ctx, |m| {
            m.content(format!("{} `{}ms`.", reply, msg_latency.as_millis()))
        })
        .await;
    Ok(())
}
