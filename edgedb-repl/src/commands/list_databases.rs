use crate::commands::Options;
use crate::commands::list;
use crate::client::Client;


pub async fn list_databases<'x>(cli: &mut Client<'x>, options: &Options)
    -> Result<(), anyhow::Error>
{
    let items = cli.query("SELECT name := sys::Database.name").await?;
    list::print(items, "List of databases", options).await?;
    Ok(())
}
