use std::env;

use sqlx::{MySql, Pool};
use dotenvy::dotenv;
use crate::model::User;
use anyhow::{Ok, Result};

pub async fn init_dn() -> Result<Pool<MySql>> {

    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let pool = Pool::<MySql>::connect(&db_url).await?;

    sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS users (
    id BIGINT AUTO_INCREMENT NOT NULL UNIQUE,
    NAME VARCHAR(255) NOT NULL,
    email  VARCHAR(255) NOT NULL)"#).execute(&pool).await?;

    Ok(pool)
}

pub async fn add_user(pool: &Pool<MySql>, name : &str, email: &str) -> Result<()>
{
    sqlx::query("INSERT INTO users(name, email) VALUES(?,?)").bind(name).bind(email).execute(pool).await?;
     println!("Users Added!!!");
    Ok(())
}


pub async fn list_user(pool: &Pool<MySql>) -> Result<()>
{
    // cargo install sqlx-cli --no-default-features --features mysql
    // $env:DATABASE_URL = "mysql://root:rustdb@localhost:3306/world"
    // cargo sqlx prepare
    let users  = sqlx::query_as!(User, "SELECT id, name, email FROM users").fetch_all(pool).await.unwrap();
    

    println!("Users List");
    for user in users{
        println!("Id . {}, Name,  {} Email. {}", user.id, user.name, user.email)
    }
    
    Ok(())
}