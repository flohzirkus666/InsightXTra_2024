use anyhow::{Error, Ok, Result};
use serde::{Deserialize, Serialize};

// Data structure for creating a POST request
#[derive(Debug, Deserialize, Serialize)]
struct Volume {
    svm: Vserver,
    name: String,
    aggregates: Vec<String>,
    size: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct Vserver {
    name: String,
}

async fn post_request(volume: Volume) -> Result<(), Error> {
    // creating a POST request statement to an ONTAP cluster
    let client = reqwest::Client::new();
    let resource = client
        .post("https://httpbin.org/post")
        .json(&volume)
        .send()
        .await?;
    dbg!("{}", resource);
    dbg!("{}", serde_json::to_string(&volume).unwrap());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let volume = Volume {
        svm: Vserver {
            name: "TestSvm".to_string(),
        },
        name: "TestVol".to_string(),
        aggregates: vec!["TestAggr1".to_string()],
        size: 1234,
    };

    post_request(volume).await?;

    Ok(())
}
