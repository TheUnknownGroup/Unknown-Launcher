use std::convert::Into;
use std::process::Command;
use reqwest::blocking::Client;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    username: String,
}

#[tauri::command]
pub fn launch_game() -> Result<String, String> {
    let output = Command::new("java")
        .args(&[
            "-Xmx2G",
            "-jar",
            "version.jar"
        ])
        .output();

    match output {
        Ok(_) => Ok("Game launched successfully".into()),
        Err(e) => Err(format!("Failed to launch game: {}", e)),
    }
}

#[tauri::command]
pub fn authenticate(email: String, password: String) -> Result<String, String> {
    let client = Client::new();
    let body = serde_json::json!({
        "email": email,
        "password": password,
    });

    let response = client
        .post("http://authserver.mojang.com/authenticate")
        .json(&body)
        .send();

    match response {
        Ok(resp) => Ok({
            if resp.status().is_success() {
                let auth: AuthResponse = resp.join().unwrap();
                Ok(format!("Welcome, {}!", auth.username))
            } else {
                Err("Authentication failed. Check the information you gave again.".into())
            }
        }),
        Err(e) => Err(format!("Error: {}", e)),
    }
}