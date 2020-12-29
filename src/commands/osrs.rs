use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandError, CommandResult},
    model::prelude::*,
};

use crate::utls::discordhelpers;
use serenity::builder::CreateEmbed;
use crate::utls::constants::*;

use crate::utls::osrsutils::PlayerStats;

#[command]
#[sub_commands(lookup)]
pub async fn osrs(_ctx: &Context, _msg: &Message, _args: Args) -> CommandResult {
    debug!("Command executed");
    Ok(())
}

#[command]
pub async fn lookup(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from(
            "Give me a player name to lookup!\n\nUsage: !osrs lookup <player>",
        ));
    }

    let arg = args.rest();

    debug!("Looking up: {}", arg);
    let player = PlayerStats::new(arg).await;

    if player.total == 0 {
        return Err(CommandError::from(
            format!("Unable to find player '{}' or Jagex is currently experiencing server issues.", arg),
        ));
    }


    let mut vec = Vec::new();
    vec.push(("\u{200B}", format!("{} {}",
        "<:Attackicon:792938075860500480>", player.attack,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Hitpointsicon:792938074874183710>", player.hp,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Miningicon:792938074463928390>", player.mining,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Strengthicon:792938074099023932>", player.strength,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Agilityicon:792938076031680512>", player.agility,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Smithingicon:792938073696239647>", player.smithing,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Defenceicon:792938075437006858>", player.defense,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Herbloreicon:792938074669449247>", player.herblore,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Fishingicon:792938074853081108>", player.fishing
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Rangedicon:792938074622918717>", player.ranged,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Thievingicon:792938073843302442>", player.theiving,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Cookingicon:792938075960901665>", player.cooking
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Prayericon:792938074476773396>", player.prayer,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Craftingicon:792938075381694516>", player.crafting,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Firemakingicon:792938074850066492>", player.firemaking
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Magicicon:792938074187104259>", player.magic,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Fletchingicon:792938074703265812>", player.fletching,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Woodcuttingicon:792938074119602196>", player.woodcutting
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Runecrafticon:792938074522910741>", player.runecrafting
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Slayericon:792938074190774302>", player.slayer,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Farmingicon:792938075444871188>", player.farming
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Constructionicon:792938075897987143>", player.construction,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "<:Huntericon:792938074572193832>", player.hunter,
    ), true));
    vec.push(("\u{200B}", format!("{} {}",
        "*Total level:* ", player.total,
    ), true));

    let mut emb = CreateEmbed::default();
    emb.footer(|f | {
        f.text(&format!("Requested by: {}", msg.author.tag()));
        f
    });
    emb.fields(vec);
    emb.title(format!("{} player levels", arg));
    emb.color(COLOR_OKAY);
    //emb.description(description);
    let mut emb_msg = discordhelpers::embed_message(emb);
    msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    debug!("Command executed");
    Ok(())
}

