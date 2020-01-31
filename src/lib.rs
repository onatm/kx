use serde::Deserialize;

#[serde(rename_all = "kebab-case")]
#[derive(Debug, Deserialize)]
pub struct KubeConfig {
    //    kind: String,
    //    #[serde(rename = "apiVersion")]
    //    api_version: String,
    //    clusters: Vec<NamedCluster>,
    pub contexts: Vec<NamedContext>,
    pub current_context: Option<String>,
    // #[serde(skip)]
    // raw: String,
}

#[derive(Debug, Deserialize)]
pub struct NamedContext {
    pub name: String,
    //    context: Context,
}

impl KubeConfig {
    // pub fn load(path: &PathBuf) -> Result<KubeConfig, Box<dyn Error>> {
    //     let kube_config_raw = &fs::read_to_string(path)?;

    //     let mut kube_config: KubeConfig = serde_yaml::from_str(kube_config_raw)?;
    //     kube_config.raw = kube_config_raw.to_owned();

    //     Ok(kube_config)
    // }

    pub fn list_contexts(&self) -> String {
        self.contexts
            .iter()
            .map(|c| &c.name)
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }
}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//struct Context {
//    cluster: String,
//    namespace: Option<String>,
//}

//#[derive(Debug, Serialize, Deserialize)]
//struct NamedCluster {
//    name: String,
//    cluster: Cluster,
//}
//
//#[serde(rename_all = "kebab-case")]
//#[derive(Debug, Serialize, Deserialize)]
//struct Cluster {
//    certificate_authority: Option<String>,
//    certificate_authority_data: Option<String>,
//    server: String,
//}
