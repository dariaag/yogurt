#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{self, StatusCode};
    use tokio;

    #[tokio::test]
    async fn test_user_creation() {
        let client = reqwest::Client::new();
        let user_data = serde_json::json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "password": "password123",
            "role": "standard_user"
        });

        let res = client
            .post("http://localhost:3000/users")
            .json(&user_data)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(res.status(), StatusCode::CREATED);
        let body = res.text().await.expect("Failed to read response text");
        println!("Response: {}", body);
        // Additional assertions can be made here based on response content
    }
}
