use anyhow::{Error, Ok, Result};
use env_vars::*;
use serde::{Deserialize, Serialize};

mod env_vars;

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

async fn create_nfs_export(volume: Volume, ontap_host: OntapCluster) -> Result<(), Error> {
    // creating a POST request statement to an ONTAP cluster
    let client = reqwest::Client::new();
    let resource = client
        .post("https://httpbin.org/post")
        .basic_auth(ontap_host.ontap_username, Some(ontap_host.ontap_password))
        .json(&volume)
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // getting env vars
    let ontap_host = read_env_vars();

    let volume = Volume {
        svm: Vserver {
            name: "TestSvm".to_string(),
        },
        name: "TestVol".to_string(),
        aggregates: vec!["TestAggr1".to_string()],
        size: 1234,
    };

    Ok(())
}
