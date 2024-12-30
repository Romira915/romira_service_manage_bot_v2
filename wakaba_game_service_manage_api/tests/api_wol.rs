mod common;

//noinspection NonAsciiCharacters
#[cfg(test)]
mod tests {
    use crate::common::AppStateTest;
    use axum::body::Body;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use axum::http::{Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use schema::wol::{WolRequestJson, WolResponseJson, WolTarget};
    use tower::util::ServiceExt;
    use wakaba_game_service_manage_api::config::CONFIG;
    use wakaba_game_service_manage_api::{app, AppState};

    #[tokio::test]
    async fn test_wol_targetにamd3900xを指定して正常に起動する() {
        let app = app(AppState::default_for_test());

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/wol")
                    .header(AUTHORIZATION, format!("Bearer {}", CONFIG.bearer_token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&WolRequestJson {
                            target: WolTarget::Amd3900X,
                        })
                        .unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8_lossy(&body[..]);
        let response_json: WolResponseJson = serde_json::from_str(&body).unwrap();

        assert_eq!(response_json.result, "success");
    }
}
