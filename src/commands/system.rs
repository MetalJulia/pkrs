use serenity::{framework::standard::{Args, CommandResult, macros::command}, model::channel::Message, prelude::*};

#[command]
#[aliases(s)]
async fn system(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    Ok(())
}
