use tokio::net::TcpListener;

// use pilot::config::PilotConfig;
use pilot::controller::hello::HelloResponse;
use pilot::startup::{app, run};
use serde_json::json;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("error binding TCP listener");
    let address = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        let app = app();
        run(listener, app).await.expect("failed to run application");
    });

    address
}

#[tokio::test]
async fn server_alive_check() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/ruok", &address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn get_hello_route_with_path() {
    let address = spawn_app().await;

    let name = "jimbo";

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/hello/{}", &address, name))
        .send()
        .await
        .expect("failed to execute request");

    let response_status = response.status();
    let response_data = response.json::<HelloResponse>().await.unwrap();

    assert!(response_status.is_success());
    assert_eq!(response_data.name, name);
}

#[tokio::test]
async fn get_hello_route_with_params() {
    let address = spawn_app().await;

    let name = "jimbo";

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/hello?name={}", &address, name))
        .send()
        .await
        .expect("failed to execute request");

    let response_status = response.status();
    let response_data = response.json::<HelloResponse>().await.unwrap();

    assert!(response_status.is_success());
    assert_eq!(response_data.name, name);
}

#[tokio::test]
async fn post_hello_route_with_body() {
    let address = spawn_app().await;

    let name = "jimbo";

    let client = reqwest::Client::new();

    let body = json!({
        "name": name
    });

    let response = client
        .post(&format!("http://{}/hello", &address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    let response_status = response.status();
    let response_data = response.json::<HelloResponse>().await.unwrap();

    assert!(response_status.is_success());
    assert_eq!(response_data.name, name);
}
