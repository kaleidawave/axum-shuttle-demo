mod dictionary;
mod matrix_determinant;
mod random_image;
use std::io::Cursor;

use axum::{
    extract::{Extension, Path},
    response::Html,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dictionary::{Definition, HeadwordInformation};
use hyper::{client::HttpConnector, Body, StatusCode};
use hyper_tls::HttpsConnector;
use matrix_determinant::compute_matrix_determinant;
pub use random_image::get_image;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use sync_wrapper::SyncWrapper;

#[derive(Clone)]
pub struct Client {
    pub hyper_client: hyper::client::Client<HttpsConnector<HttpConnector>, Body>,
    pub merriam_webster_api_key: Option<&'static str>,
}

#[shuttle_service::main]
async fn server(
    pool: PgPool,
) -> Result<SyncWrapper<axum::routing::Router>, shuttle_service::Error> {
    let api_key = sqlx::query("SELECT key FROM Secrets WHERE name = 'MERRIAM_WEBSTER_API_KEY'")
        .fetch_one(&pool)
        .await
        .map_err(|sql_x_error| anyhow::Error::msg(sql_x_error.to_string()))?;

    let key: Option<String> = api_key.get(0);
    let key = key.map(|key| &*Box::leak(key.into_boxed_str()));
    Ok(SyncWrapper::new(get_router(key)))
}

#[derive(Clone)]
struct Data {
    pub index_html: &'static str,
}

const README_CONTENT: &str = include_str!("../README.md");

pub fn get_router(merriam_webster_api_key: Option<&'static str>) -> Router {
    let https = HttpsConnector::new();
    let client = hyper::client::Client::builder().build::<_, hyper::Body>(https);

    let index_html = format!(
        "<!DOCTYPE html>
        <html lang=en>
        {}
        <body>
            {}
        </body>
        </html>",
        HEAD_HTML_TAG,
        markdown::to_html(README_CONTENT),
    );
    let data = Data {
        index_html: Box::leak(index_html.into_boxed_str()),
    };

    Router::new()
        .route("/", get(index))
        .route("/matrix-determinant", post(post_compute_matrix_determinant))
        .route("/random-avatar/:id", get(get_random_avatar))
        .route("/define/:word", get(get_word_definition))
        .layer(Extension(Client {
            hyper_client: client,
            merriam_webster_api_key,
        }))
        .layer(Extension(data))
}

async fn index(Extension(Data { index_html, .. }): Extension<Data>) -> impl IntoResponse {
    Html(index_html)
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum MatrixDeterminantResponse {
    Ok { value: String },
    Err { error_reason: String },
}

#[derive(Deserialize)]
pub struct MatrixDeterminantRequest {
    value: String,
}

async fn post_compute_matrix_determinant(
    Json(payload): Json<MatrixDeterminantRequest>,
) -> Json<MatrixDeterminantResponse> {
    Json(match compute_matrix_determinant(&payload.value) {
        Ok(value) => MatrixDeterminantResponse::Ok { value },
        Err(err) => MatrixDeterminantResponse::Err {
            error_reason: err.to_string(),
        },
    })
}

async fn get_random_avatar(Path(command): Path<String>) -> impl IntoResponse {
    // TODO I think using a cursor here is bad...
    let mut cursor = Cursor::new(vec![]);
    let image = get_image(&command, &mut cursor);
    if image.is_ok() {
        Ok(([("content-type", "image/png")], cursor.into_inner()))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Image construction error",
        ))
    }
}

async fn get_word_definition(
    Extension(client): Extension<Client>,
    Path(word): Path<String>,
) -> impl IntoResponse {
    let result = dictionary::get_definition(client, &word).await;
    match result {
        Ok(result) => {
            let Definition {
                short_definition,
                headword_information: HeadwordInformation { head_word: name },
                ..
            } = result;
            let name = name
                .chars()
                .filter(|char| *char == ' ' || char.is_alphanumeric())
                .collect::<String>();
            Ok(Html(format!(
                "<!DOCTYPE html>
                <html lang=en>
                {}
                <body>
                    <h1>{}</h1>
                    {}
                </body>
                </html>",
                HEAD_HTML_TAG,
                name,
                short_definition
                    .into_iter()
                    .map(|definition| format!("<p>{}</p>", definition))
                    .collect::<String>(),
            )))
        }
        Err(reason) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", reason))),
    }
}

const HEAD_HTML_TAG: &str = "<head>
<meta charset=UTF-8>
<meta name=viewport content=\"width=device-width\", initial-scale=1.0>
<link rel=preconnect href=https://fonts.googleapis.com>
<link rel=preconnect href=https://fonts.gstatic.com crossorigin>
<link href=\"https://fonts.googleapis.com/css2?family=Inter:wght@200&display=swap\" rel=stylesheet>
<style>
    body {
        font-family: 'Inter', sans-serif;
        margin: 100px auto;
        text-align: center;
        max-width: 500px;
        font-size: 12pt;
    }
    img {
        display: block;
        width: 100%;
    }
    li {
        text-align: left;
    }
</style>
<title>Axum Shuttle demo</title>
</head>";
