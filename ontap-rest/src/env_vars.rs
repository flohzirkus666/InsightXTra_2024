use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OntapCluster {
    pub ontap_host: String,
    pub ontap_username: String,
    pub ontap_password: String,
}

pub fn read_env_vars() -> OntapCluster {
    // parsing environment vars
    envy::from_env::<OntapCluster>().expect("Please provide all needed environment variables!")
}
