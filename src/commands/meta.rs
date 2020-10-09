use crate::ShardManagerContainer;
use serenity::{client::bridge::gateway::ShardId, framework::standard::{macros::command, CommandResult}};
use serenity::model::prelude::*;
use serenity::prelude::*;

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
    let _ = msg
        .reply(ctx, &format!("The shard latency is {:?}", runner.latency))
        .await;
    Ok(())
}
