use dotenv::dotenv;
use serde::Deserialize;
use std::{env, fs::File, io::Write, process::Command};

#[derive(Deserialize, Debug)]
struct JsonResponse {
    entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Entry {
    part_of_speech: String,
    senses: Vec<Sense>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Sense {
    definition: String,
    examples: Vec<String>,
}

fn format_entry(entry: Entry) -> String {
    let senses = entry
        .senses
        .into_iter()
        .enumerate()
        .map(|(index, sense)| format!("{}. {}", index + 1, format_sense(sense)))
        .collect::<Vec<_>>()
        .join("\n");
    let synonyms = if !entry.synonyms.is_empty() {
        format!("\nSynonyms: [{}]", entry.synonyms.join(", "))
    } else {
        "".to_owned()
    };
    let antonyms = if !entry.antonyms.is_empty() {
        format!("\nAntonyms: [{}]", entry.antonyms.join(", "))
    } else {
        "".to_owned()
    };
    format!(
        "\n***{}***\n{}{}{}",
        entry.part_of_speech, senses, synonyms, antonyms,
    )
}

fn format_sense(sense: Sense) -> String {
    let examples = if !sense.examples.is_empty() {
        format!("\n\t- *{}*", sense.examples.join("*\n\t- *"))
    } else {
        "".to_owned()
    };
    format!("{}{}", sense.definition, examples,)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let obsidian_vault_path = env::var("OBSIDIAN_VAULT_PATH")?;
    let args = env::args().skip(1);

    for word in args {
        let request_url = format!(
            "https://freedictionaryapi.com/api/v1/entries/en/{}",
            word.to_ascii_lowercase()
        );

        let response = reqwest::get(request_url)
            .await?
            .json::<JsonResponse>()
            .await?;

        let content = if !response.entries.is_empty() {
            let mut content = format!("## {}\n", word);
            content += &response
                .entries
                .into_iter()
                .map(format_entry)
                .collect::<Vec<_>>()
                .join("\n");
            content
        } else {
            format!("{} not found", word)
        };

        let path = format!("{}{}.md", obsidian_vault_path, word);

        let mut file = File::create(&path)?;
        file.write_fmt(format_args!("{content}"))?;

        Command::new("open")
            .arg(format!("obsidian://{path}"))
            .output()?;
    }
    Ok(())
}
