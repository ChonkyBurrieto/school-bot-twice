use std::fs;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Member;
use serenity::prelude::*;
use serenity::model::prelude::interaction::InteractionResponseType;

fn get_all_subjects() -> Vec<String> {
    let file = fs::read_to_string("assets/subjects.json").unwrap();
    serde_json::from_str(&file).unwrap()
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, mut member: Member) {
        member.user.dm(&ctx, |m| m.content("select your a-levels: ")).await.unwrap();
        for i in 0..4 {
            let mut m = member.user.dm(&ctx, |m| m.content("").components(|c| {
                c.create_action_row(|row| {
                    row.create_select_menu(|menu| {
                        menu.custom_id("first");
                        menu.placeholder("None");
                        menu.options(|f| {
                            for subj in get_all_subjects() {
                                f.create_option(|o| o.label(&subj).value(&subj));
                            }
                            f
                        })
                    })
                })
            })).await.expect("failed lamo");
    
            let interact = match m.await_component_interaction(&ctx).timeout(Duration::from_secs(60 * 5)).await {
                Some(x) => x,
                None => {
                    if i == 3 {
                        return;
                    }
                    m.reply(&ctx, "timed out").await.unwrap();
                    return;
                }
            };
    
            let picked_subject = &interact.data.values[0];    

            interact.create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::UpdateMessage)
            }).await.unwrap();

            m.edit(&ctx, |m| m.content(format!("You chose: {}", picked_subject)).components(|f| f)).await.unwrap();

            let add_role = match picked_subject.as_str() {
                "maths" => 1083129821544071312,
                "further-maths" => 1083130239422570608,
                "computer-science" => 1083171414774906923,
                "chemistry" => 1083130459711602739,
                "physics" => 1083130553236209745,
                "biology" => 1083131016086040646,
                "psychology" => 1083131059467718787,
                "sociology" => 1083131111393210459,
                "english-literature" => 1083131250925121666,
                "english-language" => 1083131200593481858,
                "geography" => 1083131300279492758,
                "history" => 1083131381841928315,
                "religious-studies" => 1083131513727614997,
                "law" => 1083131618568441856,
                "french" => 1083131678333095957,
                "spanish" => 1083131799762391080,
                "german" => 1083131826501070918,
                "music" => 1083131982931824650,
                "drama-theater-dance" => 1083136279555866624,
                "art" => 1083173157625671752,
                "product-design" => 1083173348676227152,
                "media-studies" => 1083173607729025095,
                "business" => 1083173781985566760,
                _ => continue,
            };

            member.add_role(&ctx, add_role).await.expect("failed to add role");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("{:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "MTA4MzEzMDI5MDYyMjQzNTQ3OA.GS9ZW3.4j7YFzqrxPEywa9IIv5pSDWQkYWNXeaRCK6TSE";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.unwrap();

    if let Err(why) = client.start().await {
        println!("client error: {:?}", why);
    }
}
