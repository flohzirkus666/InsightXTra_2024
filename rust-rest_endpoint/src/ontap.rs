use crate::env_vars::read_env_vars;
use anyhow::{Error, Ok, Result};
use serde::{Deserialize, Serialize};

// Data structure for creating a POST request
#[derive(Debug, Deserialize, Serialize)]
struct Volume {
    svm: Vserver,
    nas: Nas,
    name: String,
    aggregates: Vec<String>,
    size: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Vserver {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Nas {
    security_style: String,
    path: String,
}

pub async fn create_nfs_export(
    volume_name: String,
    size: u16,
    prefix: String,
) -> Result<(), Error> {
    // retrieving env vars
    let ontap_host = read_env_vars();

    // creating an object for our request
    let volume = Volume {
        svm: Vserver {
            name: "nfs_svm".to_string(),
        },
        nas: Nas {
            security_style: "unix".to_string(),
            path: format!("/{}", volume_name),
        },
        name: volume_name,
        aggregates: vec!["cluster1_01".to_string()],
        size: format!("{}{}", size, prefix.to_ascii_uppercase()),
    };
    // creating a POST request statement to an ONTAP cluster
    let client = reqwest::Client::new();

    let url = format!("https://{}/api/storage/volumes", ontap_host.ontap_host);
    // use it for testing purposes :)
    // let url = "https://httpbin.org/post";
    let _resource = client
        .post(url)
        .basic_auth(ontap_host.ontap_username, Some(ontap_host.ontap_password))
        .json(&volume)
        .send()
        .await?;

    Ok(())
}
