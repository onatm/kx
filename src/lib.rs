use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// https://github.com/kubernetes/client-go/blob/master/tools/clientcmd/api/types.go
// https://github.com/clux/kube-rs/blob/master/src/config/apis.rs
#[serde(rename_all = "kebab-case")]
#[derive(Debug, Serialize, Deserialize)]
pub struct KubeConfig {
    kind: Option<String>,
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    preferences: Option<Preferences>,
    clusters: Vec<NamedCluster>,
    users: Vec<NamedUser>,
    pub contexts: Vec<NamedContext>,
    pub current_context: Option<String>,
    extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Preferences {
    colors: Option<bool>,
    extensions: Option<Vec<NamedExtension>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NamedExtension {
    name: String,
    extension: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NamedCluster {
    name: String,
    cluster: Cluster,
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, Serialize, Deserialize)]
struct Cluster {
    server: String,
    insecure_skip_tls_verify: Option<bool>,
    certificate_authority: Option<String>,
    certificate_authority_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedUser {
    name: String,
    user: User,
}

#[serde(rename_all = "kebab-case")]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: Option<String>,
    pub password: Option<String>,

    pub token: Option<String>,
    #[serde(rename = "tokenFile")]
    pub token_file: Option<String>,

    pub client_certificate: Option<String>,
    pub client_certificate_data: Option<String>,

    pub client_key: Option<String>,
    pub client_key_data: Option<String>,

    #[serde(rename = "as")]
    pub impersonate: Option<String>,
    #[serde(rename = "as-groups")]
    pub impersonate_groups: Option<Vec<String>>,

    pub auth_provider: Option<AuthProviderConfig>,

    pub exec: Option<ExecConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProviderConfig {
    pub name: String,
    pub config: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecConfig {
    #[serde(rename = "apiVersion")]
    pub api_version: Option<String>,
    pub args: Option<Vec<String>>,
    pub command: String,
    pub env: Option<Vec<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedContext {
    pub name: String,
    context: Context,
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    cluster: String,
    namespace: Option<String>,
}

impl KubeConfig {
    pub fn load(path: &PathBuf) -> Result<KubeConfig, Box<dyn Error>> {
        let kube_config_raw = &fs::read_to_string(path)?;

        let kube_config: KubeConfig = serde_yaml::from_str(kube_config_raw)?;

        Ok(kube_config)
    }

    pub fn list_contexts(&self) -> String {
        self.contexts
            .iter()
            .map(|c| &c.name)
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }
}
