mod model;
mod db;
mod cli;

use anyhow::{Ok, Result};

use crate::{cli::start_menu, db::init_dn};

#[tokio::main]
async fn main() -> Result<()>{
    
    let pool = init_dn().await?;

    println!("Conencted to DB");

    start_menu(pool).await?;

    Ok(())
}
