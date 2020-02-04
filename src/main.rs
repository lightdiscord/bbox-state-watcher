mod summary;
mod bbox;

use bbox::Bbox;
use failure::Fallible;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Fallible<()> {
    let mut interval = time::interval(Duration::from_secs(1));
    let mut last_state = None;

    loop {
        let _ = interval.tick().await;
        let summary = Bbox::fetch_summary().await?;
        let state = summary.internet.state;

        match last_state {
            None => {
                println!("Start the application with a state of: {:?}", state);
                last_state = Some(state);
            },
            Some(ref mut last_state) if *last_state != state => {
                println!("State changed from {:?} to {:?}", last_state, state);
                *last_state = state;
            },
            _ => {}
        }
    }
}
