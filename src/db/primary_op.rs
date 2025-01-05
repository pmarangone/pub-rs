use anyhow::Error;
use axum::Json;
use tokio_postgres::{Client, NoTls};
use tracing::info;

use crate::models::params::User;

// pub async fn create_connection() -> Result<(tokio_postgres::Client, tokio_postgres::Connection<>, Error> {
//     let (client, connection) = tokio_postgres::connect(
//         "host=localhost port=5432 dbname=database user=user password=password",
//         NoTls,
//     )
//     .await?;
//     // Ok(client, connection)
// }

pub async fn query_all() -> Result<Vec<User>, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 dbname=database user=user password=password",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut users: Vec<User> = vec![];

    for row in client
        .query(
            "SELECT id, name, surname, description, age FROM person",
            &[],
        )
        .await?
    {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let surname: &str = row.get(2);
        let description: &str = row.get(3);
        let age: i32 = row.get(4);

        // let data: Option<&[u8]> = row.get(2);

        users.push(User {
            name: name.to_string(),
            surname: surname.to_string(),
            description: description.to_string(),
            age,
        });
    }

    // Ok(Json(users))
    Ok(users)
}
