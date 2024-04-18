//! Run with
//!
//! ```not_rust
//! cargo run -p example-customize-extractor-error
//! ```

// mod custom_extractor;
// mod derive_from_request;
// mod with_rejection;

use axum::{routing::post, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_customize_extractor_error=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with some routes
    let app = Router::new()
        .route("/with-rejection", post(with_rejection::handler))
        .route("/custom-extractor", post(custom_extractor::handler))
        .route("/derive-from-request", post(derive_from_request::handler));

    // Run our application
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
mod with_rejection {
    //! Uses `axum_extra::extract::WithRejection` to transform one rejection into
    //! another
    //!
    //! + Easy learning curve: `WithRejection` acts as a wrapper for another
    //!   already existing extractor. You only need to provide a `From` impl
    //!   between the original rejection type and the target rejection. Crates like
    //!   `thiserror` can provide such conversion using derive macros. See
    //!   [`thiserror`]
    //! - Verbose types: types become much larger, which makes them difficult to
    //!   read. Current limitations on type aliasing makes impossible to destructure
    //!   a type alias. See [#1116]
    //!   
    //! [`thiserror`]: https://crates.io/crates/thiserror
    //! [#1116]: https://github.com/tokio-rs/axum/issues/1116#issuecomment-1186197684

    use axum::{extract::rejection::JsonRejection, response::IntoResponse, Json};
    use axum_extra::extract::WithRejection;
    use serde_json::{json, Value};
    use thiserror::Error;

    pub async fn handler(
        // `WithRejection` will extract `Json<Value>` from the request. If it fails,
        // `JsonRejection` will be transform into `ApiError` and returned as response
        // to the client.
        //
        // The second constructor argument is not meaningful and can be safely ignored
        WithRejection(Json(value), _): WithRejection<Json<Value>, ApiError>,
    ) -> impl IntoResponse {
        Json(dbg!(value))
    }

    // We derive `thiserror::Error`
    #[derive(Debug, Error)]
    pub enum ApiError {
        // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
        // implementation. See `thiserror` docs for more information
        #[error(transparent)]
        JsonExtractorRejection(#[from] JsonRejection),
    }

    // We implement `IntoResponse` so ApiError can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let (status, message) = match self {
                ApiError::JsonExtractorRejection(json_rejection) => {
                    (json_rejection.status(), json_rejection.body_text())
                }
            };

            let payload = json!({
                "message": message,
                "origin": "with_rejection"
            });

            (status, Json(payload)).into_response()
        }
    }
}

mod custom_extractor {
    //! Manual implementation of `FromRequest` that wraps another extractor
    //!
    //! + Powerful API: Implementing `FromRequest` grants access to `RequestParts`
    //!   and `async/await`. This means that you can create more powerful rejections
    //! - Boilerplate: Requires creating a new extractor for every custom rejection
    //! - Complexity: Manually implementing `FromRequest` results on more complex code
    use axum::{
        async_trait,
        extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request},
        http::StatusCode,
        response::IntoResponse,
        RequestPartsExt,
    };
    use serde_json::{json, Value};

    pub async fn handler(Json(value): Json<Value>) -> impl IntoResponse {
        Json(dbg!(value));
    }

    // We define our own `Json` extractor that customizes the error from `axum::Json`
    pub struct Json<T>(pub T);

    #[async_trait]
    impl<S, T> FromRequest<S> for Json<T>
    where
        axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
        S: Send + Sync,
    {
        type Rejection = (StatusCode, axum::Json<Value>);

        async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
            let (mut parts, body) = req.into_parts();

            // We can use other extractors to provide better rejection messages.
            // For example, here we are using `axum::extract::MatchedPath` to
            // provide a better error message.
            //
            // Have to run that first since `Json` extraction consumes the request.
            let path =
                parts.extract::<MatchedPath>().await.map(|path| path.as_str().to_owned()).ok();

            let req = Request::from_parts(parts, body);

            match axum::Json::<T>::from_request(req, state).await {
                Ok(value) => Ok(Self(value.0)),
                // convert the error from `axum::Json` into whatever we want
                Err(rejection) => {
                    let payload = json!({
                        "message": rejection.body_text(),
                        "origin": "custom_extractor",
                        "path": path,
                    });

                    Err((rejection.status(), axum::Json(payload)))
                }
            }
        }
    }
}

mod derive_from_request {
    //! Uses `axum::extract::FromRequest` to wrap another extractor and customize the
    //! rejection
    //!
    //! + Easy learning curve: Deriving `FromRequest` generates a `FromRequest`
    //!   implementation for your type using another extractor. You only need
    //!   to provide a `From` impl between the original rejection type and the
    //!   target rejection. Crates like [`thiserror`] can provide such conversion
    //!   using derive macros.
    //! - Boilerplate: Requires deriving `FromRequest` for every custom rejection
    //! - There are some known limitations: [FromRequest#known-limitations]
    //!
    //! [`thiserror`]: https://crates.io/crates/thiserror
    //! [FromRequest#known-limitations]: https://docs.rs/axum-macros/*/axum_macros/derive.FromRequest.html#known-limitations
    use axum::{
        extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode,
        response::IntoResponse,
    };
    use serde::Serialize;
    use serde_json::{json, Value};

    pub async fn handler(Json(value): Json<Value>) -> impl IntoResponse {
        Json(dbg!(value))
    }

    // create an extractor that internally uses `axum::Json` but has a custom rejection
    #[derive(FromRequest)]
    #[from_request(via(axum::Json), rejection(ApiError))]
    pub struct Json<T>(T);

    // We implement `IntoResponse` for our extractor so it can be used as a response
    impl<T: Serialize> IntoResponse for Json<T> {
        fn into_response(self) -> axum::response::Response {
            let Self(value) = self;
            axum::Json(value).into_response()
        }
    }

    // We create our own rejection type
    #[derive(Debug)]
    pub struct ApiError {
        status: StatusCode,
        message: String,
    }

    // We implement `From<JsonRejection> for ApiError`
    impl From<JsonRejection> for ApiError {
        fn from(rejection: JsonRejection) -> Self {
            Self { status: rejection.status(), message: rejection.body_text() }
        }
    }

    // We implement `IntoResponse` so `ApiError` can be used as a response
    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {
            let payload = json!({
                "message": self.message,
                "origin": "derive_from_request"
            });

            (self.status, axum::Json(payload)).into_response()
        }
    }
}
