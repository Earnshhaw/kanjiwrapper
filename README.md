# kanjiwrapper

[kanjiwrapper](https://crates.io/crates/kanjiwrapper) is a Rust client library for accessing the [KanjiAPI](https://kanjiapi.dev/), a REST API providing detailed data about Japanese kanji characters and words.

## Features

- Fetch detailed information about single kanji characters
- Retrieve kanji lists by JLPT level, grade, Jouyou, Jinmeiyo, or all kanji
- Get word data including meanings, variants, and readings
- Async API using `reqwest` and `tokio`
- Easy to use enums and result types to handle various response formats

# Example Usage

```rust
use anyhow::Result;
use kanjiwrapper::{KanjiResponse, get};

#[tokio::main]
async fn main() -> Result<()> {
    // Fetch details for a single kanji character
    let kanjichar_details = get(KanjiResponse::SingleKanji('夏')).await?;
    let kanjichar_content = kanjichar_details.into_kanji_detail()?; // Unwrap the KanjiDetail struct

    // Safely print the first meaning if available
    if let Some(first_meaning) = kanjichar_content.meaning.get(0) {
        println!("{}", first_meaning);
    } else {
        println!("No meanings available.");
    }

    println!("Kunyomi readings: {:?}", kanjichar_content.kunyomi);
    println!("Onyomi readings: {:?}", kanjichar_content.onyomi);

    // Fetch a list of kanji for JLPT level 4
    let kanjilist = get(KanjiResponse::Jlpt(4)).await?;
    let kanjilist_content = kanjilist.into_kanji_chars()?; // Unwrap the Vec<String>
    println!("Kanji list for JLPT 4: {:?}", kanjilist_content);

    // Fetch words containing the kanji '日'
    let wordsusingkanji = get(KanjiResponse::Words('日')).await?;
    let wordsusingkanji_content = wordsusingkanji.into_words()?; // Unwrap the Vec<Word>
    println!("Words using '日': {:#?}", wordsusingkanji_content);

    Ok(())
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kanjiwrapper = "0.3"
