use std::time::Duration;

use serenity::{model::prelude::{interaction::InteractionResponseType, Member}, prelude::Context};

const ALL_ROLES: &[&str] = &[
    "maths",
    "further-maths",
    "computer-science",
    "physics",
    "chemistry",
    "biology",
    "psychology",
    "sociology",
    "english-language",
    "english-literature",
    "business",
    "media-studies",
    "history",
    "geography",
    "religious-studies",
    "law",
    "french",
    "german",
    "spanish",
    "drama-theater-dance",
    "music",
    "product-design",
    "art"
]; 

const ALL_ROLES_IDS: &[u64] = &[1083129821544071312,
1083130239422570608,
1083171414774906923,
1083130459711602739,
1083130553236209745,
1083131016086040646,
1083131059467718787,
1083131111393210459,
1083131250925121666,
1083131200593481858,
1083131300279492758,
1083131381841928315,
1083131513727614997,
1083131618568441856,
1083131678333095957,
1083131799762391080,
1083131826501070918,
1083131982931824650,
1083136279555866624,
1083173157625671752,
1083173348676227152,
1083173607729025095,
1083173781985566760,];

pub async fn set_member_roles(ctx: &Context, mut member: Member) {
    for role in ALL_ROLES_IDS {
        member.remove_role(&ctx.http, *role).await.unwrap();
    }

    member.user.dm(&ctx, |m| m.content("select your a-levels: ")).await.unwrap();
    for i in 0..4 {
        let mut m = member.user.dm(&ctx, |m| m.content("").components(|c| {
            c.create_action_row(|row| {
                row.create_select_menu(|menu| {
                    menu.custom_id("first");
                    menu.placeholder("None");
                    menu.options(|f| {
                        for subj in ALL_ROLES {
                            f.create_option(|o| o.label(&subj).value(&subj));
                        }
                        f
                    })
                })
            })
        })).await.expect("failed lmao");

        let five_mins = Duration::from_secs(60 * 5);
        let interact = match m.await_component_interaction(&ctx).timeout(five_mins).await {
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