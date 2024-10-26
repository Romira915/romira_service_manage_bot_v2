mod common;

//noinspection NonAsciiCharacters
#[cfg(test)]
mod tests {
    use crate::common::app_state_for_test;
    use axum::body::Body;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use axum::http::{Method, Request, StatusCode};
    use schema::sdtd::SdtdRequestJson;
    use tower::util::ServiceExt;
    use wakaba_game_service_manage_api::app;
    use wakaba_game_service_manage_api::config::CONFIG;

    #[tokio::test]
    async fn test_sdtd_handler_authorizationヘッダーがないときunauthorizedを返す() {
        let app = app(app_state_for_test());

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/sdtd")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_sdtd_handler_正しいauthorizationヘッダーでokを返す() {
        let app = app(app_state_for_test());

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/sdtd")
                    .header(AUTHORIZATION, format!("Bearer {}", CONFIG.bearer_token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&SdtdRequestJson::default()).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_sdtd_handler_不正なauthorizationヘッダーでunauthorizedを返す() {
        let app = app(app_state_for_test());

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/sdtd")
                    .header("Authorization", "Bearer invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
