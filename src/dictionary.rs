use hyper::{body::to_bytes, Uri};
use serde::Deserialize;

use crate::Client;

/// https://dictionaryapi.com/products/json#sec-2.meta
#[derive(Deserialize)]
pub struct EntryMetadata {
    pub id: String,
    pub offensive: bool,
}

/// https://dictionaryapi.com/products/json#sec-2.hwi
#[derive(Deserialize)]
pub struct HeadwordInformation {
    #[serde(alias = "hw")]
    pub head_word: String,
}

/// https://dictionaryapi.com/products/json
#[derive(Deserialize)]
pub struct Definition {
    pub meta: EntryMetadata,
    #[serde(alias = "hwi")]
    pub headword_information: HeadwordInformation,
    #[serde(alias = "shortdef")]
    pub short_definition: Vec<String>,
    #[serde(alias = "fl")]
    pub family: String,
}

const API_URL: &str = "https://dictionaryapi.com/api/v3/references/collegiate/json/";

#[derive(Debug)]
pub enum GetDefinitionError {
    NoApiKey,
    UriBuilderError,
    ResponseError,
    ToBytesError,
    DeserializeError,
    NoResults,
}

pub async fn get_definition(client: Client, word: &str) -> Result<Definition, GetDefinitionError> {
    let Client {
        hyper_client: client,
        merriam_webster_api_key: api_key,
    } = client;

    let api_key = api_key.ok_or_else(|| GetDefinitionError::NoApiKey)?;

    let uri: Uri = format!("{}{}?key={}", API_URL, word, api_key)
        .try_into()
        .map_err(|_| GetDefinitionError::UriBuilderError)?;

    let response = client
        .get(uri)
        .await
        .map_err(|_| GetDefinitionError::ResponseError)?;

    let bytes = &to_bytes(response.into_body())
        .await
        .map_err(|_| GetDefinitionError::ToBytesError)?;

    let definition = serde_json::from_slice::<Vec<Definition>>(bytes).map_err(|err| {
        eprintln!("DeserializeError: {}", err);
        GetDefinitionError::DeserializeError
    })?;

    definition
        .into_iter()
        .next()
        .ok_or(GetDefinitionError::NoResults)
}
