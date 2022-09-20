use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;

    let body = "name=le%20something&email=something-something%40gmail.com";
    let response = test_app.post_subscription(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscrption");
    
    assert_eq!(saved.email, "something-something@gmail.com");
    assert_eq!(saved.name, "le something");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;
    
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursuluta&email=", "empty email"),
        ("name=fred&email=something-that-is-not-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscription(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.", 
            description
        )
    }
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    
    let test_cases = vec![
        ("name=something", "missing the email"),
        ("email=ursula@something.com", "missing name"),
        ("", "Missing name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscription(invalid_body.into()).await;
        
        assert_eq!(400, response.status().as_u16(),
            "The API did not fail with 400 when the payload was {}", error_message);
    }
}
