use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SummaryResponse(pub Vec<Summary>);

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub internet: Internet
}

#[derive(Debug, Deserialize)]
pub struct Internet {
    pub state: InternetState
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InternetState(u8);
