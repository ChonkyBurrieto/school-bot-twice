use core::panic;
use std::collections::{HashSet, HashMap};
use std::fs;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::StandardFramework;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{hook, command, group};
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Member;
use serenity::prelude::*;

fn get_all_subjects() -> Vec<String> {
    let file = fs::read_to_string("../assets/subjects.json").unwrap();
    serde_json::from_str(&file).unwrap()
}

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct CommandCounter;
impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
} 

struct Handler;

mod subjects;

#[group]
#[prefix("subjects")]
#[commands(set)]
struct Subjects;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        subjects::set_member_roles(&ctx, member).await;
        
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[hook]
async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("got command '{}' from '{}'", command_name, msg.author);
    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, res: CommandResult) {
    match res {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn bad_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[tokio::main]
async fn main() {
    let token = "MTA4MzEzMDI5MDYyMjQzNTQ3OA.GS9ZW3.4j7YFzqrxPEywa9IIv5pSDWQkYWNXeaRCK6TSE";
    let http = Http::new(&token);

    let (_owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("couldnt get bot id: {:?}", why),
            }
        }
        Err(why) => panic!("couldnt get app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c|  
            c.with_whitespace(true)
            .prefix(">")
            .delimiters(vec![", ", ","])
            .ignore_bots(true)
            .allow_dm(true)
            .case_insensitivity(false)
        )
        // .before(before)
        // .after(after)
        // .unrecognised_command(bad_command)
        // .normal_message(normal)
        .group(&SUBJECTS_GROUP);
        

    let intents = GatewayIntents::all();
        
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("client error: {:?}", why);
    }
}

#[command]
async fn set(ctx: &Context, msg: &Message) -> CommandResult {
    let mut member = msg.member(&ctx.http).await.unwrap();
    subjects::set_member_roles(ctx, member).await;

    Ok(())
}
