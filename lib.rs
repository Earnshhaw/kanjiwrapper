use anyhow::{Result, anyhow};
use reqwest;
use serde::Deserialize;

#[derive(Debug)]
pub enum KanjiResponse {
    SingleKanji(char),
    Jouyou,
    Jinmeiyo,
    All,
    Grade(u8),
    Jlpt(u8),
    Words(char),
}

#[derive(Debug, Deserialize)]
pub struct KanjiDetail {
    #[serde(rename = "freq_mainichi_shinbun")]
    pub frequency: Option<u64>,

    pub grade: Option<u8>,

    #[serde(rename = "heisig_en")]
    pub heisig_en: Option<String>,

    pub jlpt: u8,

    pub kanji: String,

    #[serde(rename = "kun_readings")]
    pub kunyomi: Vec<String>,

    #[serde(rename = "meanings")]
    pub meaning: Vec<String>,

    #[serde(rename = "name_readings")]
    pub name_readings: Vec<String>,

    pub notes: Vec<String>,

    #[serde(rename = "on_readings")]
    pub onyomi: Vec<String>,

    #[serde(rename = "stroke_count")]
    pub strokecount: Option<u8>,

    pub unicode: String,
}

#[derive(Debug, Deserialize)]
pub struct Word {
    pub meanings: Vec<Meaning>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    pub glosses: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Variant {
    pub priorities: Vec<String>,
    pub pronounced: String,
    pub written: String,
}

#[derive(Debug)]
pub enum KanjiResult {
    KanjiDetail(KanjiDetail),
    KanjiChars(Vec<String>),
    Words(Vec<Word>),
}

impl KanjiResult {
    pub fn into_kanji_detail(self) -> Result<KanjiDetail> {
        match self {
            KanjiResult::KanjiDetail(detail) => Ok(detail),
            other => Err(anyhow!("Expected KanjiDetail, got {:?}", other)),
        }
    }

    pub fn into_kanji_chars(self) -> Result<Vec<String>> {
        match self {
            KanjiResult::KanjiChars(chars) => Ok(chars),
            other => Err(anyhow!("Expected KanjiChars, got {:?}", other)),
        }
    }

    pub fn into_words(self) -> Result<Vec<Word>> {
        match self {
            KanjiResult::Words(words) => Ok(words),
            other => Err(anyhow!("Expected Words, got {:?}", other)),
        }
    }
}

pub async fn client_response(calltype: KanjiResponse) -> Result<KanjiResult> {
    let client = reqwest::Client::new();

    let endpoint = match &calltype {
        KanjiResponse::All => "kanji/all".to_string(),
        KanjiResponse::SingleKanji(c) => format!("kanji/{}", c),
        KanjiResponse::Grade(grade) => format!("kanji/grade-{}", grade),
        KanjiResponse::Jlpt(level) => format!("kanji/jlpt-{}", level),
        KanjiResponse::Jinmeiyo => "kanji/jinmeiyou".to_string(),
        KanjiResponse::Jouyou => "kanji/jouyou".to_string(),
        KanjiResponse::Words(c) => format!("words/{}", c),
    };

    let url = format!("https://kanjiapi.dev/v1/{}", endpoint);

    let resp_text = client.get(&url).send().await?.text().await?;

    match calltype {
        KanjiResponse::SingleKanji(_) => {
            let detail: KanjiDetail = serde_json::from_str(&resp_text)?;
            Ok(KanjiResult::KanjiDetail(detail))
        }
        KanjiResponse::All
        | KanjiResponse::Grade(_)
        | KanjiResponse::Jlpt(_)
        | KanjiResponse::Jinmeiyo
        | KanjiResponse::Jouyou => {
            let chars: Vec<String> = serde_json::from_str(&resp_text)?;
            Ok(KanjiResult::KanjiChars(chars))
        }
        KanjiResponse::Words(_) => {
            let words: Vec<Word> = serde_json::from_str(&resp_text)?;
            Ok(KanjiResult::Words(words))
        }
    }
}

pub async fn get(operation: KanjiResponse) -> Result<KanjiResult> {
    client_response(operation).await
}
