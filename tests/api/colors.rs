use color_mixer_service::RgbColor;

use crate::helpers::spawn_app;

// Test that [252, 211, 0] + [0, 0, 96] = [138, 152, 79]
#[tokio::test]
async fn mixing_colors_works() {
    // Arrange
    let addr = spawn_app();

    // Act
    let response = reqwest::get(format!("{}/mix/FCD300/000060/0.5", &addr))
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let mix: RgbColor = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    assert_eq!(
        mix,
        RgbColor {
            color: [138, 152, 79]
        }
    );
}

#[tokio::test]
async fn invalid_color_fails_with_500() {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/mix/xyzbad/xyzbad/0.5", &addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_server_error());
}
