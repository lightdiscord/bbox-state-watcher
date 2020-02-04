use crate::summary::{Summary, SummaryResponse};
use failure::{Fallible, format_err};

pub struct Bbox;

const URL: &'static str = "https://mabbox.bytel.fr/api/v1/summary";

impl Bbox {
    pub async fn fetch_summary() -> Fallible<Summary> {
        let response = reqwest::get(URL).await?.text().await?;
        let response: SummaryResponse = serde_json::from_str(&response)?;
        let summary = response.0
            .into_iter()
            .next()
            .ok_or(format_err!("should contains at least one element"))?;

        Ok(summary)
    }
}

