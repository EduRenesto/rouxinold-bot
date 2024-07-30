mod oci;

use serenity::{all::{GuildId, Message, ReactionType, RoleId}, async_trait, prelude::*};
use color_eyre::eyre::eyre;

type Result<T> = color_eyre::Result<T>;

struct RouxinoldHandler {
    guild_id: GuildId,
    role_id: RoleId,
    instance_id: String,
}

impl RouxinoldHandler {
    async fn new() -> Result<Self> {
        let guild_id = std::env::var("ROUXINOLD_GUILD_ID")?.parse()?;
        let role_id = std::env::var("ROUXINOLD_ROLE_ID")?.parse()?;
        let instance_id = std::env::var("ROUXINOLD_INSTANCE_ID")?.parse()?;

        Ok(Self {
            guild_id: GuildId::new(guild_id),
            role_id: RoleId::new(role_id),
            instance_id,
        })
    }

    async fn validate_msg(&self, ctx: &Context, msg: &Message) -> Result<bool> {
        if !msg.content.starts_with("!rxnl") {
            return Ok(false)
        }

        {
            let guild = msg
                .guild_id
                .ok_or(eyre!("msg has no guild"))?;

            if guild != self.guild_id {
                return Err(eyre!("rouxinold cannot be called from guild {}", guild));
            }
        }


        let has_role = msg
            .author
            .has_role(ctx.http(), self.guild_id, self.role_id)
            .await?;

        if !has_role {
            let author_nick = msg
                .author_nick(ctx.http())
                .await
                .ok_or(eyre!("user has no nickname"))?;

            return Err(eyre!("user {} does not have permissions to run \"{}\"", author_nick, msg.content))
        }

        Ok(true)
    }

    async fn handle_up(&self, ctx: &Context, msg: &Message) -> Result<()> {
        msg.react(ctx.http(), ReactionType::Unicode("👀".to_string())).await?;

        oci::start_instance(&self.instance_id).await?;

        msg.reply(ctx.http(), "rxnl-mc1 is up!").await?;

        Ok(())
    }

    async fn message_internal(&self, ctx: Context, msg: Message) -> Result<()> {
        let should_answer = self.validate_msg(&ctx, &msg).await?;

        if !should_answer {
            return Ok(())
        }

        let mut tokens = msg.content.split(" ");
        tokens.next().unwrap();

        match tokens.next() {
            Some("up") => self.handle_up(&ctx, &msg).await?,
            _ => return Err(eyre!("incorrect usage")),
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for RouxinoldHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        match self.message_internal(ctx, msg).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error answering message: {}", e.to_string())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    if let Ok(file) = std::env::var("ROUXINOLD_ENV_FILE") {
        dotenvy::from_filename(file)?;
    } else {
        let _ = dotenvy::dotenv();
    }

    let token = std::env::var("ROUXINOLD_DISCORD_TOKEN")?;

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = RouxinoldHandler::new().await?;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await?;

    client.start().await?;

    Ok(())
}
