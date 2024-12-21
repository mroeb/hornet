
mod commands;


use std::env;

use dotenv::dotenv;
use serenity::{all::{Command, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, GatewayIntents, Interaction, Ready}, async_trait, Client};
use tokio::task;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    


    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

        if let Interaction::Command(command) = interaction {
            
            let content = match command.data.name.as_str() {
                "hornet" => Some(commands::hornet::run(&command.data.options())),
                "hello" => Some(commands::hello::run(&command.data.options())),
                _ => Some ("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }

        }

    }



    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} spawned", ready.user.name);
        
        println!("- Guilds: ");
        for g in ready.guilds.iter() {
            println!("{}", g.id);
        }

        let _ = Command::create_global_command(&ctx.http, commands::hornet::register()).await;
        println!("/hornet command registered");
        let _ = Command::create_global_command(&ctx.http, commands::hello::register()).await;
        println!("/hello command registered");
        
    }
}


#[tokio::main]
async fn main() {
    

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the enviroment");

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;


    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    let hornet_thread = task::spawn(async move {
        if let Err(why) = client.start().await  {
            println!("Client error: {why:?}");
        }
    });

    let _ = tokio::join!(hornet_thread);
}
