use std::time::Duration;
use serenity::model::prelude::{interaction::InteractionResponseType, Member};
use serenity::prelude::Context;

const ALL_ROLES: &[&str] = &[
    "maths", // 1083129821544071312
    "further-maths", // 1083130239422570608
    "computer-science", // 1083171414774906923
    "physics", // 1083130553236209745
    "chemistry", // 1083130459711602739
    "biology", // 1083131016086040646
    "psychology", // 1083131059467718787
    "sociology", // 1083131111393210459
    "english-language", // 1083131200593481858
    "english-literature", // 1083131250925121666
    "business", // 1083173781985566760
    "media-studies", // 1083173607729025095
    "history", // 1083131381841928315
    "geography", // 1083131300279492758
    "religious-studies", // 1083131513727614997
    "law", // 1083131618568441856
    "french", // 1083131678333095957
    "german", // 1083131826501070918
    "spanish", // 1083131799762391080
    "drama-theater-dance", // 1083136279555866624
    "music", // 1083131982931824650
    "product-design", // 1083173348676227152
    "art" // 1083173157625671752
]; 

const ALL_ROLES_IDS: &[u64] = &[
    1083129821544071312, // CHECKED 
    1083130239422570608, // CHECKED
    1083171414774906923, // CHECKED
    1083130553236209745, // CHECKED
    1083130459711602739, // CHECKED
    1083131016086040646, // CHECKED
    1083131059467718787, // CHECKED
    1083131111393210459, // CHECKED
    1083131200593481858, // CHECKED
    1083131250925121666, // CHECKED
    1083173781985566760, // CHECKED
    1083173607729025095, // CHECKED
    1083131381841928315, // CHECKED
    1083131300279492758, // CHECKED
    1083131513727614997, // CHECKED
    1083131618568441856, // CHECKED
    1083131678333095957, // CHECKED
    1083131826501070918, // CHECKED
    1083131799762391080, // CHECKED
    1083136279555866624, // CHECKED
    1083131982931824650, // CHECKED
    1083173348676227152, // CHECKED
    1083173157625671752, // CHECKED
    
];

pub async fn set_member_roles(ctx: &Context, mut member: Member) {
    // stops people from stacking a-levels
    for role in ALL_ROLES_IDS {
        member.remove_role(&ctx.http, *role).await.unwrap();
    }

    member.user.dm(&ctx, |m| m.content("select your a-levels: ")).await.unwrap();
    // there is a max of 4 a-levels
    for i in 0..4 {
        let mut m = member.user.dm(&ctx, |m| m.content("").components(|c| {
            c.create_action_row(|row| {
                row.create_select_menu(|menu| {
                    menu.custom_id("first");
                    menu.placeholder("None");
                    menu.options(|f| {
                        for i in 0..ALL_ROLES.len() {
                            f.create_option(|o| o.label(&ALL_ROLES[i]).value(i));
                        }
                        f
                    })
                })
            })
        })).await.expect("failed to dm user");

        // dont let this go on for more than 5 minutes
        let five_mins = Duration::from_secs(60 * 5);
        let interact = match m.await_component_interaction(&ctx).timeout(five_mins).await {
            Some(x) => x,
            None => {
                // the fourth a-level is optional so its not an error
                if i != 3 {
                    m.reply(&ctx, "timed out").await.unwrap();
                }
                return;
            }
        };

        let choice: usize = interact.data.values[0].parse().unwrap();
        let subject_name = &ALL_ROLES[choice];
        let add_role = &ALL_ROLES_IDS[choice];

        interact.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage)
        }).await.unwrap();

        m.edit(&ctx, |m| m.content(format!("You chose: {}", subject_name)).components(|f| f)).await.unwrap();

        member.add_role(&ctx, *add_role).await.expect("failed to add role");
    }
}