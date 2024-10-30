mod common;

//noinspection NonAsciiCharacters
#[cfg(test)]
mod tests {
    use crate::common::{AppStateTest, MockSystemd};
    use axum::body::Body;
    use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use axum::http::{Method, Request, StatusCode};
    use http_body_util::BodyExt;
    use schema::sdtd::{SdtdRequestJson, SdtdResponseJson};
    use schema::{SystemdCommand, SystemdStatus};
    use std::sync::Arc;
    use tower::util::ServiceExt;
    use wakaba_game_service_manage_api::config::CONFIG;
    use wakaba_game_service_manage_api::{app, AppState};

    #[tokio::test]
    async fn test_sdtd_handler_authorizationヘッダーがないときunauthorizedを返す() {
        let app = app(AppState::default_for_test());

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
        let app = app(AppState::default_for_test());

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
        let app = app(AppState::default_for_test());

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

    #[tokio::test]
    async fn test_sdtd_handler_startリクエストが成功時にsuccessを返す() {
        let app = app(AppState::default_for_test());

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/sdtd")
                    .header(AUTHORIZATION, format!("Bearer {}", CONFIG.bearer_token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&SdtdRequestJson {
                            command: SystemdCommand::Start,
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
        assert_eq!(
            body,
            serde_json::to_string(&SdtdResponseJson {
                result: "success",
                status: None
            })
            .unwrap()
        );
    }

    #[tokio::test]
    async fn test_sdtd_handler_is_activeリクエストが成功時にsuccessとactiveを返す() {
        let app = app(
            AppState::default_for_test().sdtd_systemd(Arc::new(MockSystemd {
                is_active_return_value: true,
            })),
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/sdtd")
                    .header(AUTHORIZATION, format!("Bearer {}", CONFIG.bearer_token))
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        serde_json::to_string(&SdtdRequestJson {
                            command: SystemdCommand::IsActive,
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
        assert_eq!(
            body,
            serde_json::to_string(&SdtdResponseJson {
                result: "success",
                status: Some(SystemdStatus::Active)
            })
            .unwrap()
        );
    }
}
