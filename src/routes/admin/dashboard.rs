use crate::authentication::UserId;
// use actix_session::Session;
// use crate::session_state::TypedSession;
use crate::utils::e500;
use actix_web::http::header::ContentType;
// use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn admin_dashborad(
    // session: TypedSession,
    pool: web::Data<PgPool>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, actix_web::Error> {
    // let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
    //     get_username(user_id, &pool).await.map_err(e500)?
    // } else {
    //     return Ok(HttpResponse::SeeOther()
    //         .insert_header((LOCATION, "/login"))
    //         .finish());
    // };
    let user_id = user_id.into_inner();
    let username = get_username(*user_id, &pool).await.map_err(e500)?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Admin dashboard</title>
                </head>
                <body>
                    <p>Welcome {username}!</p>
                    <p>Available actions:</p>
                    <ol>
                        <li><a href="/admin/password">Change password</a></li>
                        <li>
                            <form name="logoutForm" action="/admin/logout" method="post">
                                <input type="submit" value="Logout">
                            </form>
                        </li>
                    </ol>
                </body>
                </html>
            "#
        )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
            SELECT username FROM users WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a useranme.")?;
    Ok(row.username)
}
