use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;
// use actix_web::http::header::HeaderValue;
// use reqwest::header::HeaderValue;
// use std::collections::HashSet;

#[actix_web::test]
async fn an_error_flash_message_is_set_on_failure() {
    let app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "random-username",
        "password": "random-password"
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/login");

    // let cookies: HashSet<_> = response
    //     .headers()
    //     .get_all("Set-Cookie")
    //     .into_iter()
    //     .collect();
    // assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));
    // let flash_cookie = response.cookies().find(|c| c.name() == "_flash").unwrap();
    // assert_eq!(flash_cookie.value(), "Authentication failed");

    let html_page = app.get_login_html().await;
    // tracing::info!(html_page, "HTML RETURN");
    // tracing::error!("Failed to execute query: {:?}", html_page);
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    //Reload the login page
    let html_page = app.get_login_html().await;
    assert!(!html_page.contains(r#"<p><i>Authentication failed</i></p>"#));
}

#[actix_web::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    let app = spawn_app().await;
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });

    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
