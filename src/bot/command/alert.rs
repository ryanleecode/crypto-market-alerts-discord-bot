use std::collections::HashMap;

use anyhow::Result;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};

use crate::db;

pub async fn run(
    db: &DatabaseConnection,
    command: &ApplicationCommandInteraction,
) -> Result<Vec<db::alert::Model>> {
    let options = &command.data.options;
    let options_map: HashMap<_, _> = options.into_iter().map(|o| (o.name.clone(), o)).collect();
    let category_option = options_map
        .get("category")
        .and_then(|o| o.resolved.as_ref())
        .ok_or_else(|| anyhow::Error::msg("missing category"))?;

    if let CommandDataOptionValue::String(category) = category_option {
        let alerts = db::alert::Entity::find()
            .filter(db::alert::Column::Category.contains(category))
            .all(db)
            .await?;

        Ok(alerts)
    } else {
        Ok(Vec::new())
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("alerts")
        .description("Query crypto market alerts")
        .create_option(|option| {
            option
                .name("category")
                .description("The category to query alerts from")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
