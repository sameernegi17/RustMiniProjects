use sqlx::{MySql, Pool};
use anyhow::Result;
use inquire::Text;

use crate::db::{add_user, list_user};

pub async fn start_menu(pool: Pool<MySql>) -> Result<()>
{
    loop {
        println!("Choose your option");
        println!("1. Add a new user");
        println!("2. List All users");
        println!("3. Exit");

        let choice = Text::new("Enter chice (1,2,3) :").prompt()?;
    
        match choice.trim(){
            "1" => {
                let name = Text::new("Enter your name").prompt()?;
                let email = Text::new("Enter your email").prompt()?;
                add_user(&pool, &name, &email).await?;
            }
            "2" => {
                list_user(&pool).await?;
            }
            "3" => {
                println!("Exiting");
                break;
            }
            _ => {println!("Invalid choice Try Again");}
        }
    }

        Ok(())
}