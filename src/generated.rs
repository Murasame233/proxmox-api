pub mod cluster {
    pub struct ClusterClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl ClusterClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/cluster".to_string(),
            }
        }
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
    pub struct GETReturnsItems {}
    impl ClusterClient {
        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
    pub mod replication {
        pub struct ReplicationClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl ReplicationClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/replication"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl ReplicationClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub source: Option<String>,
            pub rate: Option<f64>,
            pub id: String,
            pub comment: Option<String>,
            #[serde(rename = "type")]
            pub ty: String,
            pub target: String,
            pub schedule: Option<String>,
            pub disable: Option<bool>,
            pub remove_job: Option<String>,
        }
        impl ReplicationClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        pub mod id {
            pub struct IdClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl IdClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    id: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, id),
                    }
                }
            }
            impl IdClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub rate: Option<f64>,
                pub remove_job: Option<String>,
                pub digest: Option<String>,
                pub schedule: Option<String>,
                pub delete: Option<String>,
                pub disable: Option<bool>,
                pub comment: Option<String>,
                pub source: Option<String>,
            }
            impl IdClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct DELETEParams {
                pub force: Option<bool>,
                pub keep: Option<bool>,
            }
            impl IdClient {
                pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &params)
                }
            }
        }
        impl ReplicationClient {
            pub fn id(&self, id: &str) -> id::IdClient {
                id::IdClient::new(self.client.clone(), &self.path, id)
            }
        }
    }
    impl ClusterClient {
        pub fn replication(&self) -> replication::ReplicationClient {
            replication::ReplicationClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod metrics {
        pub struct MetricsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl MetricsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/metrics"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl MetricsClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod server {
            pub struct ServerClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ServerClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/server"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub server: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub port: u64,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool",
                    deserialize_with = "::proxmox_api::deserialize_bool"
                )]
                pub disable: bool,
                pub id: String,
                #[serde(rename = "type")]
                pub ty: String,
            }
            impl ServerClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod id {
                pub struct IdClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IdClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        id: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, id),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub path: Option<String>,
                    pub bucket: Option<String>,
                    pub digest: Option<String>,
                    pub disable: Option<bool>,
                    pub influxdbproto: Option<String>,
                    #[serde(rename = "verify-certificate")]
                    pub verify_certificate: Option<bool>,
                    pub token: Option<String>,
                    #[serde(rename = "api-path-prefix")]
                    pub api_path_prefix: Option<String>,
                    pub mtu: Option<u64>,
                    pub server: String,
                    #[serde(rename = "max-body-size")]
                    pub max_body_size: Option<u64>,
                    pub port: u64,
                    pub delete: Option<String>,
                    pub proto: Option<String>,
                    pub organization: Option<String>,
                    pub timeout: Option<u64>,
                }
                impl IdClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub server: String,
                    pub organization: Option<String>,
                    pub mtu: Option<u64>,
                    pub timeout: Option<u64>,
                    pub bucket: Option<String>,
                    pub port: u64,
                    #[serde(rename = "verify-certificate")]
                    pub verify_certificate: Option<bool>,
                    #[serde(rename = "api-path-prefix")]
                    pub api_path_prefix: Option<String>,
                    pub proto: Option<String>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(rename = "max-body-size")]
                    pub max_body_size: Option<u64>,
                    pub disable: Option<bool>,
                    pub token: Option<String>,
                    pub path: Option<String>,
                    pub influxdbproto: Option<String>,
                }
                impl IdClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                impl IdClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
                impl IdClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl ServerClient {
                pub fn id(&self, id: &str) -> id::IdClient {
                    id::IdClient::new(self.client.clone(), &self.path, id)
                }
            }
        }
        impl MetricsClient {
            pub fn server(&self) -> server::ServerClient {
                server::ServerClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn metrics(&self) -> metrics::MetricsClient {
            metrics::MetricsClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod config {
        pub struct ConfigClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl ConfigClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/config"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub nodeid: Option<u64>,
            #[serde(rename = "link[n]")]
            pub link_n: Option<String>,
            pub votes: Option<u64>,
            pub clustername: String,
        }
        impl ConfigClient {
            pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl ConfigClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod apiversion {
            pub struct ApiversionClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ApiversionClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/apiversion"),
                    }
                }
            }
            impl ApiversionClient {
                pub fn get(&self) -> Result<u64, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    Ok(self
                        .client
                        .get::<_, _, ::proxmox_api::Integer>(&path, &())?
                        .get())
                }
            }
        }
        impl ConfigClient {
            pub fn apiversion(&self) -> apiversion::ApiversionClient {
                apiversion::ApiversionClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod nodes {
            pub struct NodesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl NodesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/nodes"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub node: String,
            }
            impl NodesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod node {
                pub struct NodeClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl NodeClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        node: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, node),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub new_node_ip: Option<String>,
                    pub nodeid: Option<u64>,
                    pub votes: Option<u64>,
                    #[serde(rename = "link[n]")]
                    pub link_n: Option<String>,
                    pub apiversion: Option<u64>,
                    pub force: Option<bool>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct POSTReturns {
                    pub corosync_conf: String,
                    pub warnings: Vec<String>,
                    pub corosync_authkey: String,
                }
                impl NodeClient {
                    pub fn post(
                        &self,
                        params: POSTParams,
                    ) -> Result<POSTReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                impl NodeClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
            }
            impl NodesClient {
                pub fn node(&self, node: &str) -> node::NodeClient {
                    node::NodeClient::new(self.client.clone(), &self.path, node)
                }
            }
        }
        impl ConfigClient {
            pub fn nodes(&self) -> nodes::NodesClient {
                nodes::NodesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod join {
            pub struct JoinClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl JoinClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/join"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub fingerprint: String,
                pub force: Option<bool>,
                pub hostname: String,
                #[serde(rename = "link[n]")]
                pub link_n: Option<String>,
                pub nodeid: Option<u64>,
                pub password: String,
                pub votes: Option<u64>,
            }
            impl JoinClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                pub node: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsNodelistItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub quorum_votes: u64,
                pub name: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub nodeid: Option<u64>,
                pub pve_fp: String,
                pub pve_addr: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub ring0_addr: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub nodelist: Vec<GETReturnsNodelistItems>,
                pub config_digest: String,
                pub preferred_node: String,
                pub totem: (),
            }
            impl JoinClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl ConfigClient {
            pub fn join(&self) -> join::JoinClient {
                join::JoinClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod totem {
            pub struct TotemClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl TotemClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/totem"),
                    }
                }
            }
            impl TotemClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl ConfigClient {
            pub fn totem(&self) -> totem::TotemClient {
                totem::TotemClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod qdevice {
            pub struct QdeviceClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl QdeviceClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/qdevice"),
                    }
                }
            }
            impl QdeviceClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl ConfigClient {
            pub fn qdevice(&self) -> qdevice::QdeviceClient {
                qdevice::QdeviceClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn config(&self) -> config::ConfigClient {
            config::ConfigClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod firewall {
        pub struct FirewallClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl FirewallClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/firewall"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl FirewallClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod groups {
            pub struct GroupsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl GroupsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/groups"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub rename: Option<String>,
                pub digest: Option<String>,
                pub group: String,
                pub comment: Option<String>,
            }
            impl GroupsClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
                pub digest: String,
                pub group: String,
            }
            impl GroupsClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod group {
                pub struct GroupClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl GroupClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        group: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, group),
                        }
                    }
                }
                impl GroupClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pos: u64,
                }
                impl GroupClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    #[serde(rename = "type")]
                    pub ty: String,
                    pub dport: Option<String>,
                    pub log: Option<String>,
                    pub dest: Option<String>,
                    pub iface: Option<String>,
                    #[serde(rename = "icmp-type")]
                    pub icmp_type: Option<String>,
                    pub comment: Option<String>,
                    pub action: String,
                    pub enable: Option<u64>,
                    pub proto: Option<String>,
                    pub source: Option<String>,
                    pub digest: Option<String>,
                    #[serde(rename = "macro")]
                    pub macro_def: Option<String>,
                    pub pos: Option<u64>,
                    pub sport: Option<String>,
                }
                impl GroupClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                pub mod pos {
                    pub struct PosClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PosClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            pos: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, pos),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub dest: Option<String>,
                        pub log: Option<String>,
                        pub source: Option<String>,
                        #[serde(rename = "icmp-type")]
                        pub icmp_type: Option<String>,
                        pub iface: Option<String>,
                        #[serde(rename = "macro")]
                        pub macro_def: Option<String>,
                        pub dport: Option<String>,
                        pub sport: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: Option<String>,
                        pub digest: Option<String>,
                        pub action: Option<String>,
                        pub proto: Option<String>,
                        pub comment: Option<String>,
                        pub delete: Option<String>,
                        pub enable: Option<u64>,
                        pub moveto: Option<u64>,
                    }
                    impl PosClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub digest: Option<String>,
                    }
                    impl PosClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub log: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub sport: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub proto: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub source: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub enable: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub comment: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub dest: Option<String>,
                        pub action: String,
                        #[serde(rename = "macro")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub macro_def: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub iface: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub pos: u64,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ipversion: Option<u64>,
                        #[serde(rename = "icmp-type")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub icmp_type: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub dport: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: String,
                    }
                    impl PosClient {
                        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl GroupClient {
                    pub fn pos(&self, pos: &str) -> pos::PosClient {
                        pos::PosClient::new(self.client.clone(), &self.path, pos)
                    }
                }
            }
            impl GroupsClient {
                pub fn group(&self, group: &str) -> group::GroupClient {
                    group::GroupClient::new(self.client.clone(), &self.path, group)
                }
            }
        }
        impl FirewallClient {
            pub fn groups(&self) -> groups::GroupsClient {
                groups::GroupsClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod rules {
            pub struct RulesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RulesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/rules"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                #[serde(rename = "type")]
                pub ty: String,
                pub enable: Option<u64>,
                pub source: Option<String>,
                pub iface: Option<String>,
                #[serde(rename = "macro")]
                pub macro_def: Option<String>,
                pub log: Option<String>,
                pub pos: Option<u64>,
                pub proto: Option<String>,
                #[serde(rename = "icmp-type")]
                pub icmp_type: Option<String>,
                pub action: String,
                pub dport: Option<String>,
                pub digest: Option<String>,
                pub sport: Option<String>,
                pub dest: Option<String>,
                pub comment: Option<String>,
            }
            impl RulesClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub pos: u64,
            }
            impl RulesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod pos {
                pub struct PosClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl PosClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        pos: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, pos),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
                pub struct DELETEParams {
                    pub digest: Option<String>,
                }
                impl PosClient {
                    pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
                pub struct PUTParams {
                    pub comment: Option<String>,
                    pub source: Option<String>,
                    pub iface: Option<String>,
                    pub action: Option<String>,
                    pub dest: Option<String>,
                    pub proto: Option<String>,
                    pub delete: Option<String>,
                    #[serde(rename = "macro")]
                    pub macro_def: Option<String>,
                    pub log: Option<String>,
                    pub moveto: Option<u64>,
                    #[serde(rename = "type")]
                    pub ty: Option<String>,
                    pub dport: Option<String>,
                    pub sport: Option<String>,
                    pub digest: Option<String>,
                    #[serde(rename = "icmp-type")]
                    pub icmp_type: Option<String>,
                    pub enable: Option<u64>,
                }
                impl PosClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(rename = "icmp-type")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub icmp_type: Option<String>,
                    pub action: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub log: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pos: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub dest: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub proto: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub iface: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub source: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ipversion: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub sport: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub dport: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub enable: Option<u64>,
                    #[serde(rename = "macro")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub macro_def: Option<String>,
                    #[serde(rename = "type")]
                    pub ty: String,
                }
                impl PosClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl RulesClient {
                pub fn pos(&self, pos: &str) -> pos::PosClient {
                    pos::PosClient::new(self.client.clone(), &self.path, pos)
                }
            }
        }
        impl FirewallClient {
            pub fn rules(&self) -> rules::RulesClient {
                rules::RulesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod ipset {
            pub struct IpsetClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl IpsetClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/ipset"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub name: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
                pub digest: String,
            }
            impl IpsetClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub name: String,
                pub rename: Option<String>,
                pub digest: Option<String>,
                pub comment: Option<String>,
            }
            impl IpsetClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod name {
                pub struct NameClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl NameClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        name: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, name),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    pub force: Option<bool>,
                }
                impl NameClient {
                    pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                    pub cidr: String,
                    pub digest: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nomatch: Option<bool>,
                }
                impl NameClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub cidr: String,
                    pub comment: Option<String>,
                    pub nomatch: Option<bool>,
                }
                impl NameClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                pub mod cidr {
                    pub struct CidrClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CidrClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            cidr: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, cidr),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub digest: Option<String>,
                    }
                    impl CidrClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    impl CidrClient {
                        pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub digest: Option<String>,
                        pub nomatch: Option<bool>,
                        pub comment: Option<String>,
                    }
                    impl CidrClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl NameClient {
                    pub fn cidr(&self, cidr: &str) -> cidr::CidrClient {
                        cidr::CidrClient::new(self.client.clone(), &self.path, cidr)
                    }
                }
            }
            impl IpsetClient {
                pub fn name(&self, name: &str) -> name::NameClient {
                    name::NameClient::new(self.client.clone(), &self.path, name)
                }
            }
        }
        impl FirewallClient {
            pub fn ipset(&self) -> ipset::IpsetClient {
                ipset::IpsetClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod aliases {
            pub struct AliasesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl AliasesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/aliases"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub cidr: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
                pub digest: String,
                pub name: String,
            }
            impl AliasesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub cidr: String,
                pub comment: Option<String>,
                pub name: String,
            }
            impl AliasesClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod name {
                pub struct NameClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl NameClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        name: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, name),
                        }
                    }
                }
                impl NameClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub comment: Option<String>,
                    pub cidr: String,
                    pub digest: Option<String>,
                    pub rename: Option<String>,
                }
                impl NameClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    pub digest: Option<String>,
                }
                impl NameClient {
                    pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
            }
            impl AliasesClient {
                pub fn name(&self, name: &str) -> name::NameClient {
                    name::NameClient::new(self.client.clone(), &self.path, name)
                }
            }
        }
        impl FirewallClient {
            pub fn aliases(&self) -> aliases::AliasesClient {
                aliases::AliasesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod options {
            pub struct OptionsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl OptionsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/options"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub enable: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub policy_in: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub ebtables: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub log_ratelimit: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub policy_out: Option<String>,
            }
            impl OptionsClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct PUTParams {
                pub policy_in: Option<String>,
                pub policy_out: Option<String>,
                pub digest: Option<String>,
                pub ebtables: Option<bool>,
                pub delete: Option<String>,
                pub enable: Option<u64>,
                pub log_ratelimit: Option<String>,
            }
            impl OptionsClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
        }
        impl FirewallClient {
            pub fn options(&self) -> options::OptionsClient {
                options::OptionsClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod macros {
            pub struct MacrosClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl MacrosClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/macros"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(rename = "macro")]
                pub macro_def: String,
                pub descr: String,
            }
            impl MacrosClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl FirewallClient {
            pub fn macros(&self) -> macros::MacrosClient {
                macros::MacrosClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod refs {
            pub struct RefsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RefsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/refs"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                #[serde(rename = "type")]
                pub ty: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(rename = "ref")]
                pub reference: String,
                pub name: String,
                #[serde(rename = "type")]
                pub ty: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
            }
            impl RefsClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl FirewallClient {
            pub fn refs(&self) -> refs::RefsClient {
                refs::RefsClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn firewall(&self) -> firewall::FirewallClient {
            firewall::FirewallClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod backup {
        pub struct BackupClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl BackupClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/backup"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub id: String,
        }
        impl BackupClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct POSTParams {
            pub vmid: Option<String>,
            pub tmpdir: Option<String>,
            pub zstd: Option<u64>,
            pub performance: Option<String>,
            pub protected: Option<bool>,
            pub bwlimit: Option<u64>,
            #[serde(rename = "prune-backups")]
            pub prune_backups: Option<String>,
            pub maxfiles: Option<u64>,
            pub pool: Option<String>,
            pub remove: Option<bool>,
            #[serde(rename = "repeat-missed")]
            pub repeat_missed: Option<bool>,
            pub dow: Option<String>,
            pub mailto: Option<String>,
            pub stdexcludes: Option<bool>,
            pub pigz: Option<u64>,
            #[serde(rename = "notes-template")]
            pub notes_template: Option<String>,
            pub starttime: Option<String>,
            pub dumpdir: Option<String>,
            pub compress: Option<String>,
            pub stop: Option<bool>,
            pub ionice: Option<u64>,
            pub lockwait: Option<u64>,
            pub comment: Option<String>,
            pub mailnotification: Option<String>,
            pub script: Option<String>,
            pub node: Option<String>,
            pub storage: Option<String>,
            pub id: Option<String>,
            pub schedule: Option<String>,
            pub exclude: Option<String>,
            pub all: Option<bool>,
            pub enabled: Option<bool>,
            pub mode: Option<String>,
            pub stopwait: Option<u64>,
            pub quiet: Option<bool>,
            #[serde(rename = "exclude-path")]
            pub exclude_path: Option<String>,
        }
        impl BackupClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        pub mod id {
            pub struct IdClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl IdClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    id: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, id),
                    }
                }
            }
            impl IdClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            impl IdClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub all: Option<bool>,
                pub maxfiles: Option<u64>,
                pub vmid: Option<String>,
                pub node: Option<String>,
                pub remove: Option<bool>,
                pub stop: Option<bool>,
                pub performance: Option<String>,
                #[serde(rename = "notes-template")]
                pub notes_template: Option<String>,
                pub comment: Option<String>,
                pub script: Option<String>,
                pub starttime: Option<String>,
                pub tmpdir: Option<String>,
                pub enabled: Option<bool>,
                pub lockwait: Option<u64>,
                pub protected: Option<bool>,
                pub bwlimit: Option<u64>,
                pub delete: Option<String>,
                pub quiet: Option<bool>,
                pub compress: Option<String>,
                pub mailto: Option<String>,
                pub pigz: Option<u64>,
                pub schedule: Option<String>,
                #[serde(rename = "exclude-path")]
                pub exclude_path: Option<String>,
                #[serde(rename = "prune-backups")]
                pub prune_backups: Option<String>,
                pub stopwait: Option<u64>,
                pub mailnotification: Option<String>,
                pub stdexcludes: Option<bool>,
                pub ionice: Option<u64>,
                pub pool: Option<String>,
                pub zstd: Option<u64>,
                pub dow: Option<String>,
                pub mode: Option<String>,
                pub storage: Option<String>,
                pub exclude: Option<String>,
                pub dumpdir: Option<String>,
                #[serde(rename = "repeat-missed")]
                pub repeat_missed: Option<bool>,
            }
            impl IdClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            pub mod included_volumes {
                pub struct IncludedVolumesClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IncludedVolumesClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/included_volumes"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsChildrenItemsChildrenItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub included: bool,
                    pub id: String,
                    pub name: String,
                    pub reason: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsChildrenItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub id: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub name: Option<String>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub children: Option<Vec<Option<GETReturnsChildrenItemsChildrenItems>>>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    pub children: Vec<GETReturnsChildrenItems>,
                }
                impl IncludedVolumesClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl IdClient {
                pub fn included_volumes(&self) -> included_volumes::IncludedVolumesClient {
                    included_volumes::IncludedVolumesClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl BackupClient {
            pub fn id(&self, id: &str) -> id::IdClient {
                id::IdClient::new(self.client.clone(), &self.path, id)
            }
        }
    }
    impl ClusterClient {
        pub fn backup(&self) -> backup::BackupClient {
            backup::BackupClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod backup_info {
        pub struct BackupInfoClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl BackupInfoClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/backup-info"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub subdir: String,
        }
        impl BackupInfoClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod not_backed_up {
            pub struct NotBackedUpClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl NotBackedUpClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/not-backed-up"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub name: Option<String>,
                #[serde(rename = "type")]
                pub ty: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub vmid: u64,
            }
            impl NotBackedUpClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl BackupInfoClient {
            pub fn not_backed_up(&self) -> not_backed_up::NotBackedUpClient {
                not_backed_up::NotBackedUpClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn backup_info(&self) -> backup_info::BackupInfoClient {
            backup_info::BackupInfoClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod ha {
        pub struct HaClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl HaClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/ha"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub id: String,
        }
        impl HaClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod resources {
            pub struct ResourcesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ResourcesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/resources"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                #[serde(rename = "type")]
                pub ty: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub sid: String,
            }
            impl ResourcesClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub sid: String,
                pub max_restart: Option<u64>,
                pub state: Option<String>,
                #[serde(rename = "type")]
                pub ty: Option<String>,
                pub max_relocate: Option<u64>,
                pub group: Option<String>,
                pub comment: Option<String>,
            }
            impl ResourcesClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod sid {
                pub struct SidClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl SidClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        sid: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, sid),
                        }
                    }
                }
                impl SidClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub delete: Option<String>,
                    pub digest: Option<String>,
                    pub max_relocate: Option<u64>,
                    pub state: Option<String>,
                    pub group: Option<String>,
                    pub max_restart: Option<u64>,
                    pub comment: Option<String>,
                }
                impl SidClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub max_restart: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub state: Option<String>,
                    pub digest: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub group: Option<String>,
                    pub sid: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub max_relocate: Option<u64>,
                }
                impl SidClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod migrate {
                    pub struct MigrateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MigrateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/migrate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub node: String,
                    }
                    impl MigrateClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl SidClient {
                    pub fn migrate(&self) -> migrate::MigrateClient {
                        migrate::MigrateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod relocate {
                    pub struct RelocateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RelocateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/relocate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub node: String,
                    }
                    impl RelocateClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl SidClient {
                    pub fn relocate(&self) -> relocate::RelocateClient {
                        relocate::RelocateClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl ResourcesClient {
                pub fn sid(&self, sid: &str) -> sid::SidClient {
                    sid::SidClient::new(self.client.clone(), &self.path, sid)
                }
            }
        }
        impl HaClient {
            pub fn resources(&self) -> resources::ResourcesClient {
                resources::ResourcesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod groups {
            pub struct GroupsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl GroupsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/groups"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub nofailback: Option<bool>,
                pub restricted: Option<bool>,
                #[serde(rename = "type")]
                pub ty: Option<String>,
                pub group: String,
                pub nodes: String,
                pub comment: Option<String>,
            }
            impl GroupsClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub group: String,
            }
            impl GroupsClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod group {
                pub struct GroupClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl GroupClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        group: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, group),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub nodes: Option<String>,
                    pub nofailback: Option<bool>,
                    pub digest: Option<String>,
                    pub restricted: Option<bool>,
                    pub delete: Option<String>,
                    pub comment: Option<String>,
                }
                impl GroupClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                impl GroupClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                impl GroupClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
            }
            impl GroupsClient {
                pub fn group(&self, group: &str) -> group::GroupClient {
                    group::GroupClient::new(self.client.clone(), &self.path, group)
                }
            }
        }
        impl HaClient {
            pub fn groups(&self) -> groups::GroupsClient {
                groups::GroupsClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod status {
            pub struct StatusClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StatusClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/status"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl StatusClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod current {
                pub struct CurrentClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CurrentClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/current"),
                        }
                    }
                }
                impl CurrentClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl StatusClient {
                pub fn current(&self) -> current::CurrentClient {
                    current::CurrentClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod manager_status {
                pub struct ManagerStatusClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ManagerStatusClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/manager_status"),
                        }
                    }
                }
                impl ManagerStatusClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl StatusClient {
                pub fn manager_status(&self) -> manager_status::ManagerStatusClient {
                    manager_status::ManagerStatusClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl HaClient {
            pub fn status(&self) -> status::StatusClient {
                status::StatusClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn ha(&self) -> ha::HaClient {
            ha::HaClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod acme {
        pub struct AcmeClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl AcmeClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/acme"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl AcmeClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod plugins {
            pub struct PluginsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl PluginsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/plugins"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                #[serde(rename = "type")]
                pub ty: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub plugin: String,
            }
            impl PluginsClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub api: Option<String>,
                pub data: Option<String>,
                pub id: String,
                pub nodes: Option<String>,
                #[serde(rename = "type")]
                pub ty: String,
                pub disable: Option<bool>,
                #[serde(rename = "validation-delay")]
                pub validation_delay: Option<u64>,
            }
            impl PluginsClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod id {
                pub struct IdClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IdClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        id: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, id),
                        }
                    }
                }
                impl IdClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub data: Option<String>,
                    pub disable: Option<bool>,
                    pub delete: Option<String>,
                    pub digest: Option<String>,
                    pub api: Option<String>,
                    pub nodes: Option<String>,
                    #[serde(rename = "validation-delay")]
                    pub validation_delay: Option<u64>,
                }
                impl IdClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                impl IdClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
            }
            impl PluginsClient {
                pub fn id(&self, id: &str) -> id::IdClient {
                    id::IdClient::new(self.client.clone(), &self.path, id)
                }
            }
        }
        impl AcmeClient {
            pub fn plugins(&self) -> plugins::PluginsClient {
                plugins::PluginsClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod account {
            pub struct AccountClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl AccountClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/account"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub directory: Option<String>,
                pub tos_url: Option<String>,
                pub contact: String,
                pub name: Option<String>,
            }
            impl AccountClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl AccountClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod name {
                pub struct NameClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl NameClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        name: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, name),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturns {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub directory: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub account: Option<()>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub location: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub tos: Option<String>,
                }
                impl NameClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                impl NameClient {
                    pub fn delete(&self) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
                pub struct PUTParams {
                    pub contact: Option<String>,
                }
                impl NameClient {
                    pub fn put(&self, params: PUTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
            }
            impl AccountClient {
                pub fn name(&self, name: &str) -> name::NameClient {
                    name::NameClient::new(self.client.clone(), &self.path, name)
                }
            }
        }
        impl AcmeClient {
            pub fn account(&self) -> account::AccountClient {
                account::AccountClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod tos {
            pub struct TosClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl TosClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/tos"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                pub directory: Option<String>,
            }
            impl TosClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Option<String>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl AcmeClient {
            pub fn tos(&self) -> tos::TosClient {
                tos::TosClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod directories {
            pub struct DirectoriesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl DirectoriesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/directories"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub name: String,
                pub url: String,
            }
            impl DirectoriesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl AcmeClient {
            pub fn directories(&self) -> directories::DirectoriesClient {
                directories::DirectoriesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod challenge_schema {
            pub struct ChallengeSchemaClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ChallengeSchemaClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/challenge-schema"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub id: String,
                #[serde(rename = "type")]
                pub ty: String,
                pub schema: (),
                pub name: String,
            }
            impl ChallengeSchemaClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl AcmeClient {
            pub fn challenge_schema(&self) -> challenge_schema::ChallengeSchemaClient {
                challenge_schema::ChallengeSchemaClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn acme(&self) -> acme::AcmeClient {
            acme::AcmeClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod ceph {
        pub struct CephClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl CephClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/ceph"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl CephClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod metadata {
            pub struct MetadataClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl MetadataClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/metadata"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct GETParams {
                pub scope: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMgrId {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_swap_kb: u64,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_total_kb: u64,
                pub ceph_version: String,
                pub addr: String,
                pub hostname: String,
                pub name: String,
                pub ceph_release: String,
                pub ceph_version_short: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMgr {
                #[serde(rename = "{id}")]
                pub _id: GETReturnsMgrId,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMonId {
                pub ceph_release: String,
                pub addrs: String,
                pub ceph_version: String,
                pub hostname: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_swap_kb: u64,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_total_kb: u64,
                pub ceph_version_short: String,
                pub name: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMon {
                #[serde(rename = "{id}")]
                pub _id: GETReturnsMonId,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMdsId {
                pub name: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_total_kb: u64,
                pub hostname: String,
                pub addr: String,
                pub ceph_release: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub mem_swap_kb: u64,
                pub ceph_version_short: String,
                pub ceph_version: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsMds {
                #[serde(rename = "{id}")]
                pub _id: GETReturnsMdsId,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsNodeNodeVersion {
                pub str: String,
                pub parts: (),
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsNodeNode {
                pub buildcommit: String,
                pub version: GETReturnsNodeNodeVersion,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsNode {
                #[serde(rename = "{node}")]
                pub _node: GETReturnsNodeNode,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub mgr: GETReturnsMgr,
                pub osd: (),
                pub mon: GETReturnsMon,
                pub mds: GETReturnsMds,
                pub node: GETReturnsNode,
            }
            impl MetadataClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl CephClient {
            pub fn metadata(&self) -> metadata::MetadataClient {
                metadata::MetadataClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod status {
            pub struct StatusClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StatusClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/status"),
                    }
                }
            }
            impl StatusClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl CephClient {
            pub fn status(&self) -> status::StatusClient {
                status::StatusClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod flags {
            pub struct FlagsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl FlagsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/flags"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool",
                    deserialize_with = "::proxmox_api::deserialize_bool"
                )]
                pub value: bool,
                pub name: String,
                pub description: String,
            }
            impl FlagsClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct PUTParams {
                pub norecover: Option<bool>,
                pub noup: Option<bool>,
                pub norebalance: Option<bool>,
                #[serde(rename = "nodeep-scrub")]
                pub nodeep_scrub: Option<bool>,
                pub noscrub: Option<bool>,
                pub pause: Option<bool>,
                pub nobackfill: Option<bool>,
                pub notieragent: Option<bool>,
                pub nodown: Option<bool>,
                pub noout: Option<bool>,
                pub noin: Option<bool>,
            }
            impl FlagsClient {
                pub fn put(&self, params: PUTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            pub mod flag {
                pub struct FlagClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl FlagClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        flag: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, flag),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub value: bool,
                }
                impl FlagClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                impl FlagClient {
                    pub fn get(&self) -> Result<bool, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        Ok(self
                            .client
                            .get::<_, _, ::proxmox_api::Bool>(&path, &())?
                            .get())
                    }
                }
            }
            impl FlagsClient {
                pub fn flag(&self, flag: &str) -> flag::FlagClient {
                    flag::FlagClient::new(self.client.clone(), &self.path, flag)
                }
            }
        }
        impl CephClient {
            pub fn flags(&self) -> flags::FlagsClient {
                flags::FlagsClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn ceph(&self) -> ceph::CephClient {
            ceph::CephClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod jobs {
        pub struct JobsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl JobsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/jobs"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub subdir: String,
        }
        impl JobsClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod schedule_analyze {
            pub struct ScheduleAnalyzeClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ScheduleAnalyzeClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/schedule-analyze"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub schedule: String,
                pub starttime: Option<u64>,
                pub iterations: Option<u64>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub utc: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub timestamp: u64,
            }
            impl ScheduleAnalyzeClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl JobsClient {
            pub fn schedule_analyze(&self) -> schedule_analyze::ScheduleAnalyzeClient {
                schedule_analyze::ScheduleAnalyzeClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl ClusterClient {
        pub fn jobs(&self) -> jobs::JobsClient {
            jobs::JobsClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod log {
        pub struct LogClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl LogClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/log"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct GETParams {
            pub max: Option<u64>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl LogClient {
            pub fn get(
                &self,
                params: GETParams,
            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &params)
            }
        }
    }
    impl ClusterClient {
        pub fn log(&self) -> log::LogClient {
            log::LogClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod resources {
        pub struct ResourcesClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl ResourcesClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/resources"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct GETParams {
            #[serde(rename = "type")]
            pub ty: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(rename = "type")]
            pub ty: String,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub pool: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub maxdisk: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub storage: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub maxmem: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub node: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_number_optional",
                deserialize_with = "::proxmox_api::deserialize_number_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub maxcpu: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub status: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub vmid: Option<u64>,
            #[serde(rename = "cgroup-mode")]
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub cgroup_mode: Option<u64>,
            pub id: String,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub content: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub level: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_number_optional",
                deserialize_with = "::proxmox_api::deserialize_number_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub cpu: Option<f64>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub disk: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub plugintype: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub uptime: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub hastate: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub mem: Option<u64>,
        }
        impl ResourcesClient {
            pub fn get(
                &self,
                params: GETParams,
            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &params)
            }
        }
    }
    impl ClusterClient {
        pub fn resources(&self) -> resources::ResourcesClient {
            resources::ResourcesClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod tasks {
        pub struct TasksClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl TasksClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/tasks"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub upid: String,
        }
        impl TasksClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
    }
    impl ClusterClient {
        pub fn tasks(&self) -> tasks::TasksClient {
            tasks::TasksClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod options {
        pub struct OptionsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl OptionsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/options"),
                }
            }
        }
        impl OptionsClient {
            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct PUTParams {
            pub migration_unsecure: Option<bool>,
            pub keyboard: Option<String>,
            pub bwlimit: Option<String>,
            pub webauthn: Option<String>,
            #[serde(rename = "registered-tags")]
            pub registered_tags: Option<String>,
            pub fencing: Option<String>,
            #[serde(rename = "tag-style")]
            pub tag_style: Option<String>,
            pub ha: Option<String>,
            pub email_from: Option<String>,
            pub language: Option<String>,
            pub description: Option<String>,
            pub max_workers: Option<u64>,
            pub crs: Option<String>,
            pub delete: Option<String>,
            pub mac_prefix: Option<String>,
            pub console: Option<String>,
            pub migration: Option<String>,
            #[serde(rename = "next-id")]
            pub next_id: Option<String>,
            pub notify: Option<String>,
            #[serde(rename = "user-tag-access")]
            pub user_tag_access: Option<String>,
            pub http_proxy: Option<String>,
            pub u2f: Option<String>,
        }
        impl OptionsClient {
            pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl ClusterClient {
        pub fn options(&self) -> options::OptionsClient {
            options::OptionsClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod status {
        pub struct StatusClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl StatusClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/status"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub quorate: Option<bool>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub version: Option<u64>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub nodes: Option<u64>,
            #[serde(rename = "type")]
            pub ty: String,
            pub id: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub local: Option<bool>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub online: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub ip: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub level: Option<String>,
            pub name: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub nodeid: Option<u64>,
        }
        impl StatusClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
    }
    impl ClusterClient {
        pub fn status(&self) -> status::StatusClient {
            status::StatusClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod nextid {
        pub struct NextidClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl NextidClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/nextid"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct GETParams {
            pub vmid: Option<u64>,
        }
        impl NextidClient {
            pub fn get(&self, params: GETParams) -> Result<u64, ::proxmox_api::Error> {
                let path = self.path.to_string();
                Ok(self
                    .client
                    .get::<_, _, ::proxmox_api::Integer>(&path, &params)?
                    .get())
            }
        }
    }
    impl ClusterClient {
        pub fn nextid(&self) -> nextid::NextidClient {
            nextid::NextidClient::new(self.client.clone(), &self.path)
        }
    }
}
pub mod nodes {
    pub struct NodesClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl NodesClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/nodes".to_string(),
            }
        }
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct GETReturnsItems {
        #[serde(
            serialize_with = "::proxmox_api::serialize_int_optional",
            deserialize_with = "::proxmox_api::deserialize_int_optional"
        )]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub uptime: Option<u64>,
        #[serde(
            serialize_with = "::proxmox_api::serialize_int_optional",
            deserialize_with = "::proxmox_api::deserialize_int_optional"
        )]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub maxmem: Option<u64>,
        #[serde(
            serialize_with = "::proxmox_api::serialize_int_optional",
            deserialize_with = "::proxmox_api::deserialize_int_optional"
        )]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub mem: Option<u64>,
        #[serde(
            serialize_with = "::proxmox_api::serialize_number_optional",
            deserialize_with = "::proxmox_api::deserialize_number_optional"
        )]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub cpu: Option<f64>,
        pub node: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub ssl_fingerprint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub level: Option<String>,
        #[serde(
            serialize_with = "::proxmox_api::serialize_int_optional",
            deserialize_with = "::proxmox_api::deserialize_int_optional"
        )]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub maxcpu: Option<u64>,
        pub status: String,
    }
    impl NodesClient {
        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
    pub mod node {
        pub struct NodeClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl NodeClient {
            pub fn new(
                client: ::std::sync::Arc<::proxmox_api::Client>,
                parent_path: &str,
                node: &str,
            ) -> Self {
                Self {
                    client,
                    path: format!("{}/{}", parent_path, node),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct GETReturnsItems {}
        impl NodeClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod qemu {
            pub struct QemuClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl QemuClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/qemu"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub start: Option<bool>,
                pub bwlimit: Option<()>,
                pub vcpus: Option<u64>,
                pub cpulimit: Option<f64>,
                pub hookscript: Option<String>,
                pub hotplug: Option<String>,
                #[serde(rename = "sata[n]")]
                pub sata_n: Option<String>,
                #[serde(rename = "live-restore")]
                pub live_restore: Option<bool>,
                pub balloon: Option<u64>,
                pub args: Option<String>,
                pub scsihw: Option<String>,
                pub rng0: Option<String>,
                #[serde(rename = "serial[n]")]
                pub serial_n: Option<String>,
                pub shares: Option<u64>,
                pub pool: Option<String>,
                #[serde(rename = "unused[n]")]
                pub unused_n: Option<String>,
                pub sshkeys: Option<String>,
                pub watchdog: Option<String>,
                pub efidisk0: Option<String>,
                pub force: Option<bool>,
                pub tdf: Option<bool>,
                pub sockets: Option<u64>,
                pub template: Option<bool>,
                pub smp: Option<u64>,
                pub cipassword: Option<String>,
                pub spice_enhancements: Option<String>,
                pub storage: Option<String>,
                pub tpmstate0: Option<String>,
                pub reboot: Option<bool>,
                #[serde(rename = "scsi[n]")]
                pub scsi_n: Option<String>,
                pub vmstatestorage: Option<String>,
                pub cores: Option<u64>,
                pub cicustom: Option<String>,
                #[serde(rename = "ide[n]")]
                pub ide_n: Option<String>,
                pub bootdisk: Option<String>,
                pub ivshmem: Option<String>,
                pub tablet: Option<bool>,
                pub description: Option<String>,
                pub machine: Option<String>,
                pub migrate_downtime: Option<f64>,
                pub startup: Option<String>,
                #[serde(rename = "usb[n]")]
                pub usb_n: Option<String>,
                pub kvm: Option<bool>,
                pub audio0: Option<String>,
                pub keyboard: Option<String>,
                pub migrate_speed: Option<u64>,
                #[serde(rename = "numa[n]")]
                pub numa_n: Option<String>,
                #[serde(rename = "hostpci[n]")]
                pub hostpci_n: Option<String>,
                pub vmgenid: Option<String>,
                pub citype: Option<String>,
                pub memory: Option<u64>,
                pub nameserver: Option<String>,
                pub protection: Option<bool>,
                pub tags: Option<String>,
                pub searchdomain: Option<String>,
                pub keephugepages: Option<bool>,
                pub agent: Option<String>,
                pub boot: Option<String>,
                pub cpuunits: Option<u64>,
                pub ciuser: Option<String>,
                pub unique: Option<bool>,
                pub smbios1: Option<String>,
                pub archive: Option<String>,
                pub vga: Option<String>,
                pub arch: Option<String>,
                #[serde(rename = "ipconfig[n]")]
                pub ipconfig_n: Option<String>,
                pub acpi: Option<bool>,
                pub cdrom: Option<String>,
                pub autostart: Option<bool>,
                pub bios: Option<String>,
                pub numa: Option<bool>,
                pub ostype: Option<String>,
                pub startdate: Option<String>,
                #[serde(rename = "parallel[n]")]
                pub parallel_n: Option<String>,
                pub onboot: Option<bool>,
                #[serde(rename = "net[n]")]
                pub net_n: Option<String>,
                pub cpu: Option<String>,
                pub vmid: u64,
                pub lock: Option<String>,
                #[serde(rename = "virtio[n]")]
                pub virtio_n: Option<String>,
                pub affinity: Option<String>,
                pub freeze: Option<bool>,
                pub hugepages: Option<String>,
                pub name: Option<String>,
                pub localtime: Option<bool>,
            }
            impl QemuClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub full: Option<bool>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub pid: Option<u64>,
                #[serde(rename = "running-machine")]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub running_machine: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_number_optional",
                    deserialize_with = "::proxmox_api::deserialize_number_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub cpus: Option<f64>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub maxdisk: Option<u64>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub maxmem: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub name: Option<String>,
                #[serde(rename = "running-qemu")]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub running_qemu: Option<String>,
                pub status: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub uptime: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub qmpstatus: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub tags: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub vmid: u64,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub lock: Option<String>,
            }
            impl QemuClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            pub mod vmid {
                pub struct VmidClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl VmidClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        vmid: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, vmid),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub subdir: String,
                }
                impl VmidClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    pub skiplock: Option<bool>,
                    #[serde(rename = "destroy-unreferenced-disks")]
                    pub destroy_unreferenced_disks: Option<bool>,
                    pub purge: Option<bool>,
                }
                impl VmidClient {
                    pub fn delete(
                        &self,
                        params: DELETEParams,
                    ) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                pub mod firewall {
                    pub struct FirewallClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl FirewallClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/firewall"),
                            }
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl FirewallClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod rules {
                        pub struct RulesClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RulesClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/rules"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub pos: u64,
                        }
                        impl RulesClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            #[serde(rename = "icmp-type")]
                            pub icmp_type: Option<String>,
                            #[serde(rename = "type")]
                            pub ty: String,
                            pub log: Option<String>,
                            #[serde(rename = "macro")]
                            pub macro_def: Option<String>,
                            pub digest: Option<String>,
                            pub source: Option<String>,
                            pub proto: Option<String>,
                            pub enable: Option<u64>,
                            pub dest: Option<String>,
                            pub pos: Option<u64>,
                            pub sport: Option<String>,
                            pub action: String,
                            pub comment: Option<String>,
                            pub iface: Option<String>,
                            pub dport: Option<String>,
                        }
                        impl RulesClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        pub mod pos {
                            pub struct PosClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl PosClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    pos: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, pos),
                                    }
                                }
                            }
                            #[derive(
                                Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize,
                            )]
                            pub struct GETReturns {
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub sport: Option<String>,
                                pub action: String,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub dest: Option<String>,
                                #[serde(rename = "icmp-type")]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub icmp_type: Option<String>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int",
                                    deserialize_with = "::proxmox_api::deserialize_int"
                                )]
                                pub pos: u64,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub dport: Option<String>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int_optional",
                                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub enable: Option<u64>,
                                #[serde(rename = "macro")]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub macro_def: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub comment: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub log: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub proto: Option<String>,
                                #[serde(rename = "type")]
                                pub ty: String,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub iface: Option<String>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int_optional",
                                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub ipversion: Option<u64>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub source: Option<String>,
                            }
                            impl PosClient {
                                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub digest: Option<String>,
                            }
                            impl PosClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub digest: Option<String>,
                                pub comment: Option<String>,
                                pub delete: Option<String>,
                                pub dport: Option<String>,
                                pub iface: Option<String>,
                                pub proto: Option<String>,
                                pub enable: Option<u64>,
                                #[serde(rename = "macro")]
                                pub macro_def: Option<String>,
                                pub moveto: Option<u64>,
                                #[serde(rename = "type")]
                                pub ty: Option<String>,
                                pub action: Option<String>,
                                pub sport: Option<String>,
                                pub log: Option<String>,
                                pub source: Option<String>,
                                pub dest: Option<String>,
                                #[serde(rename = "icmp-type")]
                                pub icmp_type: Option<String>,
                            }
                            impl PosClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                        }
                        impl RulesClient {
                            pub fn pos(&self, pos: &str) -> pos::PosClient {
                                pos::PosClient::new(self.client.clone(), &self.path, pos)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn rules(&self) -> rules::RulesClient {
                            rules::RulesClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod aliases {
                        pub struct AliasesClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl AliasesClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/aliases"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub cidr: String,
                            pub name: String,
                            pub comment: Option<String>,
                        }
                        impl AliasesClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub cidr: String,
                            pub digest: String,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                            pub name: String,
                        }
                        impl AliasesClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod name {
                            pub struct NameClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl NameClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    name: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, name),
                                    }
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub comment: Option<String>,
                                pub digest: Option<String>,
                                pub rename: Option<String>,
                                pub cidr: String,
                            }
                            impl NameClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                            impl NameClient {
                                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub digest: Option<String>,
                            }
                            impl NameClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                        }
                        impl AliasesClient {
                            pub fn name(&self, name: &str) -> name::NameClient {
                                name::NameClient::new(self.client.clone(), &self.path, name)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn aliases(&self) -> aliases::AliasesClient {
                            aliases::AliasesClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod ipset {
                        pub struct IpsetClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl IpsetClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/ipset"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub comment: Option<String>,
                            pub digest: Option<String>,
                            pub name: String,
                            pub rename: Option<String>,
                        }
                        impl IpsetClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub digest: String,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                            pub name: String,
                        }
                        impl IpsetClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod name {
                            pub struct NameClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl NameClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    name: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, name),
                                    }
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct POSTParams {
                                pub cidr: String,
                                pub comment: Option<String>,
                                pub nomatch: Option<bool>,
                            }
                            impl NameClient {
                                pub fn post(
                                    &self,
                                    params: POSTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.post(&path, &params)
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub force: Option<bool>,
                            }
                            impl NameClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                            #[derive(
                                Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize,
                            )]
                            pub struct GETReturnsItems {
                                pub cidr: String,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_bool_optional",
                                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub nomatch: Option<bool>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub comment: Option<String>,
                                pub digest: String,
                            }
                            impl NameClient {
                                pub fn get(
                                    &self,
                                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            pub mod cidr {
                                pub struct CidrClient {
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    path: String,
                                }
                                impl CidrClient {
                                    pub fn new(
                                        client: ::std::sync::Arc<::proxmox_api::Client>,
                                        parent_path: &str,
                                        cidr: &str,
                                    ) -> Self {
                                        Self {
                                            client,
                                            path: format!("{}/{}", parent_path, cidr),
                                        }
                                    }
                                }
                                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                                pub struct DELETEParams {
                                    pub digest: Option<String>,
                                }
                                impl CidrClient {
                                    pub fn delete(
                                        &self,
                                        params: DELETEParams,
                                    ) -> Result<(), ::proxmox_api::Error>
                                    {
                                        let path = self.path.to_string();
                                        self.client.delete(&path, &params)
                                    }
                                }
                                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                                pub struct PUTParams {
                                    pub comment: Option<String>,
                                    pub nomatch: Option<bool>,
                                    pub digest: Option<String>,
                                }
                                impl CidrClient {
                                    pub fn put(
                                        &self,
                                        params: PUTParams,
                                    ) -> Result<(), ::proxmox_api::Error>
                                    {
                                        let path = self.path.to_string();
                                        self.client.put(&path, &params)
                                    }
                                }
                                impl CidrClient {
                                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                        let path = self.path.to_string();
                                        self.client.get(&path, &())
                                    }
                                }
                            }
                            impl NameClient {
                                pub fn cidr(&self, cidr: &str) -> cidr::CidrClient {
                                    cidr::CidrClient::new(self.client.clone(), &self.path, cidr)
                                }
                            }
                        }
                        impl IpsetClient {
                            pub fn name(&self, name: &str) -> name::NameClient {
                                name::NameClient::new(self.client.clone(), &self.path, name)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn ipset(&self) -> ipset::IpsetClient {
                            ipset::IpsetClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod options {
                        pub struct OptionsClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl OptionsClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/options"),
                                }
                            }
                        }
                        #[derive(
                            Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                        )]
                        pub struct GETReturns {
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub policy_out: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub macfilter: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub dhcp: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub radv: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub enable: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub ndp: Option<bool>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub log_level_out: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub policy_in: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub log_level_in: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub ipfilter: Option<bool>,
                        }
                        impl OptionsClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct PUTParams {
                            pub macfilter: Option<bool>,
                            pub ndp: Option<bool>,
                            pub policy_in: Option<String>,
                            pub ipfilter: Option<bool>,
                            pub enable: Option<bool>,
                            pub radv: Option<bool>,
                            pub digest: Option<String>,
                            pub dhcp: Option<bool>,
                            pub policy_out: Option<String>,
                            pub delete: Option<String>,
                            pub log_level_in: Option<String>,
                            pub log_level_out: Option<String>,
                        }
                        impl OptionsClient {
                            pub fn put(
                                &self,
                                params: PUTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.put(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn options(&self) -> options::OptionsClient {
                            options::OptionsClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod log {
                        pub struct LogClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl LogClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/log"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub start: Option<u64>,
                            pub until: Option<u64>,
                            pub limit: Option<u64>,
                            pub since: Option<u64>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub t: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub n: u64,
                        }
                        impl LogClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn log(&self) -> log::LogClient {
                            log::LogClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod refs {
                        pub struct RefsClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RefsClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/refs"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            #[serde(rename = "type")]
                            pub ty: Option<String>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                            #[serde(rename = "type")]
                            pub ty: String,
                            pub name: String,
                        }
                        impl RefsClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn refs(&self) -> refs::RefsClient {
                            refs::RefsClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn firewall(&self) -> firewall::FirewallClient {
                        firewall::FirewallClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod agent {
                    pub struct AgentClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl AgentClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/agent"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub command: String,
                    }
                    impl AgentClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl AgentClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod fsfreeze_freeze {
                        pub struct FsfreezeFreezeClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FsfreezeFreezeClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/fsfreeze-freeze"),
                                }
                            }
                        }
                        impl FsfreezeFreezeClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn fsfreeze_freeze(&self) -> fsfreeze_freeze::FsfreezeFreezeClient {
                            fsfreeze_freeze::FsfreezeFreezeClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod fsfreeze_status {
                        pub struct FsfreezeStatusClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FsfreezeStatusClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/fsfreeze-status"),
                                }
                            }
                        }
                        impl FsfreezeStatusClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn fsfreeze_status(&self) -> fsfreeze_status::FsfreezeStatusClient {
                            fsfreeze_status::FsfreezeStatusClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod fsfreeze_thaw {
                        pub struct FsfreezeThawClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FsfreezeThawClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/fsfreeze-thaw"),
                                }
                            }
                        }
                        impl FsfreezeThawClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn fsfreeze_thaw(&self) -> fsfreeze_thaw::FsfreezeThawClient {
                            fsfreeze_thaw::FsfreezeThawClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod fstrim {
                        pub struct FstrimClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FstrimClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/fstrim"),
                                }
                            }
                        }
                        impl FstrimClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn fstrim(&self) -> fstrim::FstrimClient {
                            fstrim::FstrimClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_fsinfo {
                        pub struct GetFsinfoClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetFsinfoClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-fsinfo"),
                                }
                            }
                        }
                        impl GetFsinfoClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_fsinfo(&self) -> get_fsinfo::GetFsinfoClient {
                            get_fsinfo::GetFsinfoClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_host_name {
                        pub struct GetHostNameClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetHostNameClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-host-name"),
                                }
                            }
                        }
                        impl GetHostNameClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_host_name(&self) -> get_host_name::GetHostNameClient {
                            get_host_name::GetHostNameClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_memory_block_info {
                        pub struct GetMemoryBlockInfoClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetMemoryBlockInfoClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-memory-block-info"),
                                }
                            }
                        }
                        impl GetMemoryBlockInfoClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_memory_block_info(
                            &self,
                        ) -> get_memory_block_info::GetMemoryBlockInfoClient
                        {
                            get_memory_block_info::GetMemoryBlockInfoClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod get_memory_blocks {
                        pub struct GetMemoryBlocksClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetMemoryBlocksClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-memory-blocks"),
                                }
                            }
                        }
                        impl GetMemoryBlocksClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_memory_blocks(
                            &self,
                        ) -> get_memory_blocks::GetMemoryBlocksClient {
                            get_memory_blocks::GetMemoryBlocksClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod get_osinfo {
                        pub struct GetOsinfoClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetOsinfoClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-osinfo"),
                                }
                            }
                        }
                        impl GetOsinfoClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_osinfo(&self) -> get_osinfo::GetOsinfoClient {
                            get_osinfo::GetOsinfoClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_time {
                        pub struct GetTimeClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetTimeClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-time"),
                                }
                            }
                        }
                        impl GetTimeClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_time(&self) -> get_time::GetTimeClient {
                            get_time::GetTimeClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_timezone {
                        pub struct GetTimezoneClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetTimezoneClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-timezone"),
                                }
                            }
                        }
                        impl GetTimezoneClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_timezone(&self) -> get_timezone::GetTimezoneClient {
                            get_timezone::GetTimezoneClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_users {
                        pub struct GetUsersClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetUsersClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-users"),
                                }
                            }
                        }
                        impl GetUsersClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_users(&self) -> get_users::GetUsersClient {
                            get_users::GetUsersClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod get_vcpus {
                        pub struct GetVcpusClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl GetVcpusClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/get-vcpus"),
                                }
                            }
                        }
                        impl GetVcpusClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn get_vcpus(&self) -> get_vcpus::GetVcpusClient {
                            get_vcpus::GetVcpusClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod info {
                        pub struct InfoClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl InfoClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/info"),
                                }
                            }
                        }
                        impl InfoClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn info(&self) -> info::InfoClient {
                            info::InfoClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod network_get_interfaces {
                        pub struct NetworkGetInterfacesClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl NetworkGetInterfacesClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/network-get-interfaces"),
                                }
                            }
                        }
                        impl NetworkGetInterfacesClient {
                            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn network_get_interfaces(
                            &self,
                        ) -> network_get_interfaces::NetworkGetInterfacesClient
                        {
                            network_get_interfaces::NetworkGetInterfacesClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod ping {
                        pub struct PingClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl PingClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/ping"),
                                }
                            }
                        }
                        impl PingClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn ping(&self) -> ping::PingClient {
                            ping::PingClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod shutdown {
                        pub struct ShutdownClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ShutdownClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/shutdown"),
                                }
                            }
                        }
                        impl ShutdownClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn shutdown(&self) -> shutdown::ShutdownClient {
                            shutdown::ShutdownClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod suspend_disk {
                        pub struct SuspendDiskClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SuspendDiskClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/suspend-disk"),
                                }
                            }
                        }
                        impl SuspendDiskClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn suspend_disk(&self) -> suspend_disk::SuspendDiskClient {
                            suspend_disk::SuspendDiskClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod suspend_hybrid {
                        pub struct SuspendHybridClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SuspendHybridClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/suspend-hybrid"),
                                }
                            }
                        }
                        impl SuspendHybridClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn suspend_hybrid(&self) -> suspend_hybrid::SuspendHybridClient {
                            suspend_hybrid::SuspendHybridClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod suspend_ram {
                        pub struct SuspendRamClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SuspendRamClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/suspend-ram"),
                                }
                            }
                        }
                        impl SuspendRamClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn suspend_ram(&self) -> suspend_ram::SuspendRamClient {
                            suspend_ram::SuspendRamClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod set_user_password {
                        pub struct SetUserPasswordClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SetUserPasswordClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/set-user-password"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub password: String,
                            pub crypted: Option<bool>,
                            pub username: String,
                        }
                        impl SetUserPasswordClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn set_user_password(
                            &self,
                        ) -> set_user_password::SetUserPasswordClient {
                            set_user_password::SetUserPasswordClient::new(
                                self.client.clone(),
                                &self.path,
                            )
                        }
                    }
                    pub mod exec {
                        pub struct ExecClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ExecClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/exec"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            #[serde(rename = "input-data")]
                            pub input_data: Option<String>,
                            pub command: Option<String>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct POSTReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub pid: u64,
                        }
                        impl ExecClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<POSTReturns, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn exec(&self) -> exec::ExecClient {
                            exec::ExecClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod exec_status {
                        pub struct ExecStatusClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ExecStatusClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/exec-status"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub pid: u64,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub signal: Option<u64>,
                            #[serde(rename = "err-data")]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub err_data: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub exited: bool,
                            #[serde(rename = "out-truncated")]
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub out_truncated: Option<bool>,
                            #[serde(rename = "err-truncated")]
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub err_truncated: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub exitcode: Option<u64>,
                            #[serde(rename = "out-data")]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub out_data: Option<String>,
                        }
                        impl ExecStatusClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<GETReturns, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn exec_status(&self) -> exec_status::ExecStatusClient {
                            exec_status::ExecStatusClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod file_read {
                        pub struct FileReadClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FileReadClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/file-read"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub file: String,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            pub content: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub truncated: Option<bool>,
                        }
                        impl FileReadClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<GETReturns, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn file_read(&self) -> file_read::FileReadClient {
                            file_read::FileReadClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod file_write {
                        pub struct FileWriteClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl FileWriteClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/file-write"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub content: String,
                            pub encode: Option<bool>,
                            pub file: String,
                        }
                        impl FileWriteClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl AgentClient {
                        pub fn file_write(&self) -> file_write::FileWriteClient {
                            file_write::FileWriteClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn agent(&self) -> agent::AgentClient {
                        agent::AgentClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrd {
                    pub struct RrdClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrdClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrd"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub cf: Option<String>,
                        pub ds: String,
                        pub timeframe: String,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub filename: String,
                    }
                    impl RrdClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn rrd(&self) -> rrd::RrdClient {
                        rrd::RrdClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrddata {
                    pub struct RrddataClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrddataClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrddata"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub cf: Option<String>,
                        pub timeframe: String,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl RrddataClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn rrddata(&self) -> rrddata::RrddataClient {
                        rrddata::RrddataClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod config {
                    pub struct ConfigClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ConfigClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/config"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub searchdomain: Option<String>,
                        pub vmstatestorage: Option<String>,
                        pub sockets: Option<u64>,
                        pub autostart: Option<bool>,
                        pub ivshmem: Option<String>,
                        pub name: Option<String>,
                        pub tablet: Option<bool>,
                        pub ciuser: Option<String>,
                        #[serde(rename = "sata[n]")]
                        pub sata_n: Option<String>,
                        pub machine: Option<String>,
                        pub bios: Option<String>,
                        pub protection: Option<bool>,
                        pub vga: Option<String>,
                        pub watchdog: Option<String>,
                        pub keephugepages: Option<bool>,
                        pub memory: Option<u64>,
                        pub kvm: Option<bool>,
                        pub smbios1: Option<String>,
                        pub agent: Option<String>,
                        pub spice_enhancements: Option<String>,
                        pub keyboard: Option<String>,
                        pub boot: Option<String>,
                        pub cdrom: Option<String>,
                        pub args: Option<String>,
                        pub cpuunits: Option<u64>,
                        #[serde(rename = "virtio[n]")]
                        pub virtio_n: Option<String>,
                        pub efidisk0: Option<String>,
                        #[serde(rename = "ide[n]")]
                        pub ide_n: Option<String>,
                        pub migrate_downtime: Option<f64>,
                        pub smp: Option<u64>,
                        pub audio0: Option<String>,
                        pub citype: Option<String>,
                        pub lock: Option<String>,
                        pub description: Option<String>,
                        pub nameserver: Option<String>,
                        pub affinity: Option<String>,
                        pub revert: Option<String>,
                        pub rng0: Option<String>,
                        pub reboot: Option<bool>,
                        pub hotplug: Option<String>,
                        #[serde(rename = "scsi[n]")]
                        pub scsi_n: Option<String>,
                        pub delete: Option<String>,
                        pub bootdisk: Option<String>,
                        pub numa: Option<bool>,
                        pub sshkeys: Option<String>,
                        pub scsihw: Option<String>,
                        pub migrate_speed: Option<u64>,
                        pub startup: Option<String>,
                        #[serde(rename = "hostpci[n]")]
                        pub hostpci_n: Option<String>,
                        pub skiplock: Option<bool>,
                        #[serde(rename = "usb[n]")]
                        pub usb_n: Option<String>,
                        pub digest: Option<String>,
                        #[serde(rename = "serial[n]")]
                        pub serial_n: Option<String>,
                        pub vmgenid: Option<String>,
                        pub cipassword: Option<String>,
                        pub cores: Option<u64>,
                        pub tpmstate0: Option<String>,
                        pub hugepages: Option<String>,
                        pub template: Option<bool>,
                        pub cpu: Option<String>,
                        pub tags: Option<String>,
                        pub onboot: Option<bool>,
                        pub localtime: Option<bool>,
                        pub freeze: Option<bool>,
                        pub shares: Option<u64>,
                        pub force: Option<bool>,
                        pub hookscript: Option<String>,
                        #[serde(rename = "net[n]")]
                        pub net_n: Option<String>,
                        pub cicustom: Option<String>,
                        pub tdf: Option<bool>,
                        #[serde(rename = "unused[n]")]
                        pub unused_n: Option<String>,
                        pub arch: Option<String>,
                        pub balloon: Option<u64>,
                        pub cpulimit: Option<f64>,
                        pub ostype: Option<String>,
                        #[serde(rename = "parallel[n]")]
                        pub parallel_n: Option<String>,
                        pub acpi: Option<bool>,
                        #[serde(rename = "ipconfig[n]")]
                        pub ipconfig_n: Option<String>,
                        pub startdate: Option<String>,
                        pub vcpus: Option<u64>,
                        #[serde(rename = "numa[n]")]
                        pub numa_n: Option<String>,
                    }
                    impl ConfigClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub snapshot: Option<String>,
                        pub current: Option<bool>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub rng0: Option<String>,
                        #[serde(rename = "hostpci[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hostpci_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub sshkeys: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub localtime: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub keephugepages: Option<bool>,
                        #[serde(rename = "ipconfig[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ipconfig_n: Option<String>,
                        pub digest: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub kvm: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vmgenid: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub smp: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tablet: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ciuser: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cipassword: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub template: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vcpus: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vmstatestorage: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub autostart: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub machine: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub smbios1: Option<String>,
                        #[serde(rename = "virtio[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub virtio_n: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub memory: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tdf: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub acpi: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub args: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub migrate_downtime: Option<f64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cores: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub nameserver: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub affinity: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub reboot: Option<bool>,
                        #[serde(rename = "scsi[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub scsi_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub spice_enhancements: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cdrom: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub startdate: Option<String>,
                        #[serde(rename = "unused[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub unused_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub bootdisk: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub watchdog: Option<String>,
                        #[serde(rename = "net[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub net_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub arch: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cpu: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub keyboard: Option<String>,
                        #[serde(rename = "parallel[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub parallel_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub audio0: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub protection: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cpuunits: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub searchdomain: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub sockets: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub lock: Option<String>,
                        #[serde(rename = "sata[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub sata_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub name: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub bios: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub onboot: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ostype: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hugepages: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tags: Option<String>,
                        #[serde(rename = "numa[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub numa_n: Option<String>,
                        #[serde(rename = "usb[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub usb_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vga: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cpulimit: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub citype: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cicustom: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hookscript: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub agent: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub boot: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hotplug: Option<String>,
                        #[serde(rename = "serial[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub serial_n: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub startup: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub scsihw: Option<String>,
                        #[serde(rename = "ide[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ide_n: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub numa: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tpmstate0: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub migrate_speed: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub shares: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub efidisk0: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub balloon: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub freeze: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ivshmem: Option<String>,
                    }
                    impl ConfigClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub startup: Option<String>,
                        pub tdf: Option<bool>,
                        pub localtime: Option<bool>,
                        pub cores: Option<u64>,
                        pub force: Option<bool>,
                        pub freeze: Option<bool>,
                        pub rng0: Option<String>,
                        pub citype: Option<String>,
                        pub migrate_downtime: Option<f64>,
                        pub hookscript: Option<String>,
                        pub hugepages: Option<String>,
                        #[serde(rename = "sata[n]")]
                        pub sata_n: Option<String>,
                        pub revert: Option<String>,
                        pub smp: Option<u64>,
                        #[serde(rename = "ipconfig[n]")]
                        pub ipconfig_n: Option<String>,
                        pub cicustom: Option<String>,
                        pub efidisk0: Option<String>,
                        pub searchdomain: Option<String>,
                        pub skiplock: Option<bool>,
                        pub name: Option<String>,
                        pub migrate_speed: Option<u64>,
                        pub affinity: Option<String>,
                        pub ciuser: Option<String>,
                        pub protection: Option<bool>,
                        pub autostart: Option<bool>,
                        pub bios: Option<String>,
                        #[serde(rename = "ide[n]")]
                        pub ide_n: Option<String>,
                        pub keyboard: Option<String>,
                        #[serde(rename = "hostpci[n]")]
                        pub hostpci_n: Option<String>,
                        pub kvm: Option<bool>,
                        pub lock: Option<String>,
                        #[serde(rename = "numa[n]")]
                        pub numa_n: Option<String>,
                        #[serde(rename = "parallel[n]")]
                        pub parallel_n: Option<String>,
                        pub spice_enhancements: Option<String>,
                        pub bootdisk: Option<String>,
                        pub boot: Option<String>,
                        pub scsihw: Option<String>,
                        pub description: Option<String>,
                        pub audio0: Option<String>,
                        pub tablet: Option<bool>,
                        pub balloon: Option<u64>,
                        pub background_delay: Option<u64>,
                        pub onboot: Option<bool>,
                        #[serde(rename = "unused[n]")]
                        pub unused_n: Option<String>,
                        pub sshkeys: Option<String>,
                        pub vmgenid: Option<String>,
                        pub smbios1: Option<String>,
                        pub vmstatestorage: Option<String>,
                        pub hotplug: Option<String>,
                        pub cpulimit: Option<f64>,
                        pub delete: Option<String>,
                        pub vga: Option<String>,
                        pub shares: Option<u64>,
                        pub ostype: Option<String>,
                        #[serde(rename = "net[n]")]
                        pub net_n: Option<String>,
                        pub digest: Option<String>,
                        #[serde(rename = "usb[n]")]
                        pub usb_n: Option<String>,
                        pub arch: Option<String>,
                        pub sockets: Option<u64>,
                        pub cpuunits: Option<u64>,
                        pub numa: Option<bool>,
                        pub acpi: Option<bool>,
                        pub ivshmem: Option<String>,
                        pub args: Option<String>,
                        #[serde(rename = "virtio[n]")]
                        pub virtio_n: Option<String>,
                        pub cipassword: Option<String>,
                        pub keephugepages: Option<bool>,
                        pub reboot: Option<bool>,
                        pub machine: Option<String>,
                        pub watchdog: Option<String>,
                        pub startdate: Option<String>,
                        pub nameserver: Option<String>,
                        pub tags: Option<String>,
                        pub tpmstate0: Option<String>,
                        pub cpu: Option<String>,
                        pub vcpus: Option<u64>,
                        pub agent: Option<String>,
                        pub memory: Option<u64>,
                        pub template: Option<bool>,
                        pub cdrom: Option<String>,
                        #[serde(rename = "scsi[n]")]
                        pub scsi_n: Option<String>,
                        #[serde(rename = "serial[n]")]
                        pub serial_n: Option<String>,
                    }
                    impl ConfigClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<Option<String>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn config(&self) -> config::ConfigClient {
                        config::ConfigClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod pending {
                    pub struct PendingClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PendingClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/pending"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub delete: Option<u64>,
                        pub key: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub pending: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub value: Option<String>,
                    }
                    impl PendingClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl VmidClient {
                    pub fn pending(&self) -> pending::PendingClient {
                        pending::PendingClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod cloudinit {
                    pub struct CloudinitClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CloudinitClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/cloudinit"),
                            }
                        }
                    }
                    impl CloudinitClient {
                        pub fn put(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &())
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub old: Option<String>,
                        pub key: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub new: Option<String>,
                    }
                    impl CloudinitClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod dump {
                        pub struct DumpClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl DumpClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/dump"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            #[serde(rename = "type")]
                            pub ty: String,
                        }
                        impl DumpClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl CloudinitClient {
                        pub fn dump(&self) -> dump::DumpClient {
                            dump::DumpClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn cloudinit(&self) -> cloudinit::CloudinitClient {
                        cloudinit::CloudinitClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod unlink {
                    pub struct UnlinkClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl UnlinkClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/unlink"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub idlist: String,
                        pub force: Option<bool>,
                    }
                    impl UnlinkClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn unlink(&self) -> unlink::UnlinkClient {
                        unlink::UnlinkClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod vncproxy {
                    pub struct VncproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl VncproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/vncproxy"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub websocket: Option<bool>,
                        #[serde(rename = "generate-password")]
                        pub generate_password: Option<bool>,
                    }
                    impl VncproxyClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn vncproxy(&self) -> vncproxy::VncproxyClient {
                        vncproxy::VncproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod termproxy {
                    pub struct TermproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl TermproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/termproxy"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub serial: Option<String>,
                    }
                    impl TermproxyClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn termproxy(&self) -> termproxy::TermproxyClient {
                        termproxy::TermproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod vncwebsocket {
                    pub struct VncwebsocketClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl VncwebsocketClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/vncwebsocket"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub vncticket: String,
                        pub port: u64,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub port: String,
                    }
                    impl VncwebsocketClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient {
                        vncwebsocket::VncwebsocketClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod spiceproxy {
                    pub struct SpiceproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl SpiceproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/spiceproxy"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub proxy: Option<String>,
                    }
                    impl SpiceproxyClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn spiceproxy(&self) -> spiceproxy::SpiceproxyClient {
                        spiceproxy::SpiceproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod status {
                    pub struct StatusClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StatusClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/status"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub subdir: String,
                    }
                    impl StatusClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod current {
                        pub struct CurrentClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl CurrentClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/current"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_number_optional",
                                deserialize_with = "::proxmox_api::deserialize_number_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub cpus: Option<f64>,
                            pub status: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub vmid: u64,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub name: Option<String>,
                            pub ha: (),
                            #[serde(rename = "running-machine")]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub running_machine: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub maxdisk: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub maxmem: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub agent: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub pid: Option<u64>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub tags: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub qmpstatus: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub spice: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub uptime: Option<u64>,
                            #[serde(rename = "running-qemu")]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub running_qemu: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub lock: Option<String>,
                        }
                        impl CurrentClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn current(&self) -> current::CurrentClient {
                            current::CurrentClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod start {
                        pub struct StartClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl StartClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/start"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub migratedfrom: Option<String>,
                            pub skiplock: Option<bool>,
                            pub machine: Option<String>,
                            pub migration_network: Option<String>,
                            pub migration_type: Option<String>,
                            pub targetstorage: Option<String>,
                            #[serde(rename = "force-cpu")]
                            pub force_cpu: Option<String>,
                            pub stateuri: Option<String>,
                            pub timeout: Option<u64>,
                        }
                        impl StartClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn start(&self) -> start::StartClient {
                            start::StartClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod stop {
                        pub struct StopClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl StopClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/stop"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub skiplock: Option<bool>,
                            pub migratedfrom: Option<String>,
                            pub timeout: Option<u64>,
                            #[serde(rename = "keepActive")]
                            pub keepactive: Option<bool>,
                        }
                        impl StopClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn stop(&self) -> stop::StopClient {
                            stop::StopClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod reset {
                        pub struct ResetClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ResetClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/reset"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub skiplock: Option<bool>,
                        }
                        impl ResetClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn reset(&self) -> reset::ResetClient {
                            reset::ResetClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod shutdown {
                        pub struct ShutdownClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ShutdownClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/shutdown"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub skiplock: Option<bool>,
                            pub timeout: Option<u64>,
                            #[serde(rename = "forceStop")]
                            pub forcestop: Option<bool>,
                            #[serde(rename = "keepActive")]
                            pub keepactive: Option<bool>,
                        }
                        impl ShutdownClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn shutdown(&self) -> shutdown::ShutdownClient {
                            shutdown::ShutdownClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod reboot {
                        pub struct RebootClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RebootClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/reboot"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub timeout: Option<u64>,
                        }
                        impl RebootClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn reboot(&self) -> reboot::RebootClient {
                            reboot::RebootClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod suspend {
                        pub struct SuspendClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SuspendClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/suspend"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub statestorage: Option<String>,
                            pub skiplock: Option<bool>,
                            pub todisk: Option<bool>,
                        }
                        impl SuspendClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn suspend(&self) -> suspend::SuspendClient {
                            suspend::SuspendClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod resume {
                        pub struct ResumeClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ResumeClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/resume"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub nocheck: Option<bool>,
                            pub skiplock: Option<bool>,
                        }
                        impl ResumeClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn resume(&self) -> resume::ResumeClient {
                            resume::ResumeClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn status(&self) -> status::StatusClient {
                        status::StatusClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod sendkey {
                    pub struct SendkeyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl SendkeyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/sendkey"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub skiplock: Option<bool>,
                        pub key: String,
                    }
                    impl SendkeyClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn sendkey(&self) -> sendkey::SendkeyClient {
                        sendkey::SendkeyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod feature {
                    pub struct FeatureClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl FeatureClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/feature"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub snapname: Option<String>,
                        pub feature: String,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(rename = "hasFeature")]
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub hasfeature: bool,
                        pub nodes: Vec<String>,
                    }
                    impl FeatureClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn feature(&self) -> feature::FeatureClient {
                        feature::FeatureClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod clone {
                    pub struct CloneClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CloneClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/clone"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub target: Option<String>,
                        pub snapname: Option<String>,
                        pub bwlimit: Option<()>,
                        pub full: Option<bool>,
                        pub description: Option<String>,
                        pub name: Option<String>,
                        pub newid: u64,
                        pub pool: Option<String>,
                        pub format: Option<String>,
                        pub storage: Option<String>,
                    }
                    impl CloneClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn clone(&self) -> clone::CloneClient {
                        clone::CloneClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod move_disk {
                    pub struct MoveDiskClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MoveDiskClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/move_disk"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub digest: Option<String>,
                        pub storage: Option<String>,
                        pub format: Option<String>,
                        #[serde(rename = "target-digest")]
                        pub target_digest: Option<String>,
                        #[serde(rename = "target-disk")]
                        pub target_disk: Option<String>,
                        #[serde(rename = "target-vmid")]
                        pub target_vmid: Option<u64>,
                        pub bwlimit: Option<()>,
                        pub delete: Option<bool>,
                        pub disk: String,
                    }
                    impl MoveDiskClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn move_disk(&self) -> move_disk::MoveDiskClient {
                        move_disk::MoveDiskClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod migrate {
                    pub struct MigrateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MigrateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/migrate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub target: Option<String>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub allowed_nodes: Option<()>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub not_allowed_nodes: Option<()>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub running: bool,
                        pub local_disks: (),
                        pub local_resources: (),
                    }
                    impl MigrateClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub online: Option<bool>,
                        pub migration_network: Option<String>,
                        pub migration_type: Option<String>,
                        pub bwlimit: Option<()>,
                        pub target: String,
                        pub targetstorage: Option<String>,
                        #[serde(rename = "with-local-disks")]
                        pub with_local_disks: Option<bool>,
                        pub force: Option<bool>,
                    }
                    impl MigrateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn migrate(&self) -> migrate::MigrateClient {
                        migrate::MigrateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod remote_migrate {
                    pub struct RemoteMigrateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RemoteMigrateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/remote_migrate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub online: Option<bool>,
                        #[serde(rename = "target-storage")]
                        pub target_storage: String,
                        pub delete: Option<bool>,
                        #[serde(rename = "target-bridge")]
                        pub target_bridge: String,
                        #[serde(rename = "target-endpoint")]
                        pub target_endpoint: String,
                        pub bwlimit: Option<()>,
                        #[serde(rename = "target-vmid")]
                        pub target_vmid: Option<u64>,
                    }
                    impl RemoteMigrateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn remote_migrate(&self) -> remote_migrate::RemoteMigrateClient {
                        remote_migrate::RemoteMigrateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod monitor {
                    pub struct MonitorClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MonitorClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/monitor"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub command: String,
                    }
                    impl MonitorClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn monitor(&self) -> monitor::MonitorClient {
                        monitor::MonitorClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod resize {
                    pub struct ResizeClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ResizeClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/resize"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub disk: String,
                        pub skiplock: Option<bool>,
                        pub size: String,
                        pub digest: Option<String>,
                    }
                    impl ResizeClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn resize(&self) -> resize::ResizeClient {
                        resize::ResizeClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod snapshot {
                    pub struct SnapshotClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl SnapshotClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/snapshot"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub snaptime: Option<u64>,
                        pub name: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub parent: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vmstate: Option<bool>,
                        pub description: String,
                    }
                    impl SnapshotClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub snapname: String,
                        pub vmstate: Option<bool>,
                        pub description: Option<String>,
                    }
                    impl SnapshotClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    pub mod snapname {
                        pub struct SnapnameClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SnapnameClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                                snapname: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}/{}", parent_path, snapname),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct DELETEParams {
                            pub force: Option<bool>,
                        }
                        impl SnapnameClient {
                            pub fn delete(
                                &self,
                                params: DELETEParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.delete(&path, &params)
                            }
                        }
                        #[derive(
                            Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                        )]
                        pub struct GETReturnsItems {}
                        impl SnapnameClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod config {
                            pub struct ConfigClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl ConfigClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}{}", parent_path, "/config"),
                                    }
                                }
                            }
                            impl ConfigClient {
                                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub description: Option<String>,
                            }
                            impl ConfigClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                        }
                        impl SnapnameClient {
                            pub fn config(&self) -> config::ConfigClient {
                                config::ConfigClient::new(self.client.clone(), &self.path)
                            }
                        }
                        pub mod rollback {
                            pub struct RollbackClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl RollbackClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}{}", parent_path, "/rollback"),
                                    }
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct POSTParams {
                                pub start: Option<bool>,
                            }
                            impl RollbackClient {
                                pub fn post(
                                    &self,
                                    params: POSTParams,
                                ) -> Result<String, ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.post(&path, &params)
                                }
                            }
                        }
                        impl SnapnameClient {
                            pub fn rollback(&self) -> rollback::RollbackClient {
                                rollback::RollbackClient::new(self.client.clone(), &self.path)
                            }
                        }
                    }
                    impl SnapshotClient {
                        pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient {
                            snapname::SnapnameClient::new(self.client.clone(), &self.path, snapname)
                        }
                    }
                }
                impl VmidClient {
                    pub fn snapshot(&self) -> snapshot::SnapshotClient {
                        snapshot::SnapshotClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod template {
                    pub struct TemplateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl TemplateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/template"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub disk: Option<String>,
                    }
                    impl TemplateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn template(&self) -> template::TemplateClient {
                        template::TemplateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod mtunnel {
                    pub struct MtunnelClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MtunnelClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/mtunnel"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub bridges: Option<String>,
                        pub storages: Option<String>,
                    }
                    impl MtunnelClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn mtunnel(&self) -> mtunnel::MtunnelClient {
                        mtunnel::MtunnelClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod mtunnelwebsocket {
                    pub struct MtunnelwebsocketClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MtunnelwebsocketClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/mtunnelwebsocket"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub socket: String,
                        pub ticket: String,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub port: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub socket: Option<String>,
                    }
                    impl MtunnelwebsocketClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn mtunnelwebsocket(&self) -> mtunnelwebsocket::MtunnelwebsocketClient {
                        mtunnelwebsocket::MtunnelwebsocketClient::new(
                            self.client.clone(),
                            &self.path,
                        )
                    }
                }
            }
            impl QemuClient {
                pub fn vmid(&self, vmid: &str) -> vmid::VmidClient {
                    vmid::VmidClient::new(self.client.clone(), &self.path, vmid)
                }
            }
        }
        impl NodeClient {
            pub fn qemu(&self) -> qemu::QemuClient {
                qemu::QemuClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod lxc {
            pub struct LxcClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl LxcClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/lxc"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                #[serde(rename = "unused[n]")]
                pub unused_n: Option<String>,
                pub console: Option<bool>,
                pub nameserver: Option<String>,
                pub ostemplate: String,
                pub vmid: u64,
                pub pool: Option<String>,
                pub template: Option<bool>,
                pub cores: Option<u64>,
                pub timezone: Option<String>,
                pub hostname: Option<String>,
                #[serde(rename = "net[n]")]
                pub net_n: Option<String>,
                pub ostype: Option<String>,
                pub restore: Option<bool>,
                pub force: Option<bool>,
                pub swap: Option<u64>,
                pub cpuunits: Option<u64>,
                #[serde(rename = "ignore-unpack-errors")]
                pub ignore_unpack_errors: Option<bool>,
                #[serde(rename = "mp[n]")]
                pub mp_n: Option<String>,
                pub password: Option<String>,
                pub rootfs: Option<String>,
                pub start: Option<bool>,
                pub unique: Option<bool>,
                pub description: Option<String>,
                pub hookscript: Option<String>,
                pub onboot: Option<bool>,
                pub debug: Option<bool>,
                pub arch: Option<String>,
                pub cpulimit: Option<f64>,
                #[serde(rename = "ssh-public-keys")]
                pub ssh_public_keys: Option<String>,
                pub searchdomain: Option<String>,
                pub lock: Option<String>,
                pub tty: Option<u64>,
                pub unprivileged: Option<bool>,
                pub bwlimit: Option<f64>,
                pub memory: Option<u64>,
                pub storage: Option<String>,
                pub cmode: Option<String>,
                pub startup: Option<String>,
                pub tags: Option<String>,
                pub protection: Option<bool>,
                pub features: Option<String>,
            }
            impl LxcClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_number_optional",
                    deserialize_with = "::proxmox_api::deserialize_number_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub cpus: Option<f64>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub maxswap: Option<u64>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub vmid: u64,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub maxdisk: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub name: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub maxmem: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub lock: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub tags: Option<String>,
                pub status: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub uptime: Option<u64>,
            }
            impl LxcClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod vmid {
                pub struct VmidClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl VmidClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        vmid: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, vmid),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    #[serde(rename = "destroy-unreferenced-disks")]
                    pub destroy_unreferenced_disks: Option<bool>,
                    pub purge: Option<bool>,
                    pub force: Option<bool>,
                }
                impl VmidClient {
                    pub fn delete(
                        &self,
                        params: DELETEParams,
                    ) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub subdir: String,
                }
                impl VmidClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod config {
                    pub struct ConfigClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ConfigClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/config"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub snapshot: Option<String>,
                        pub current: Option<bool>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cmode: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tty: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cores: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub nameserver: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub searchdomain: Option<String>,
                        #[serde(rename = "net[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub net_n: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cpulimit: Option<f64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub arch: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hookscript: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub lock: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub tags: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub timezone: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub unprivileged: Option<bool>,
                        #[serde(rename = "mp[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub mp_n: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub template: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub description: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub rootfs: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub startup: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub console: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub hostname: Option<String>,
                        pub digest: String,
                        #[serde(rename = "unused[n]")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub unused_n: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub onboot: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ostype: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub protection: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub features: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub lxc: Option<Vec<Option<Vec<Option<String>>>>>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub debug: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub memory: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cpuunits: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub swap: Option<u64>,
                    }
                    impl ConfigClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub protection: Option<bool>,
                        pub nameserver: Option<String>,
                        pub memory: Option<u64>,
                        pub template: Option<bool>,
                        pub timezone: Option<String>,
                        pub description: Option<String>,
                        pub hostname: Option<String>,
                        pub debug: Option<bool>,
                        pub cores: Option<u64>,
                        pub hookscript: Option<String>,
                        pub delete: Option<String>,
                        pub cmode: Option<String>,
                        pub ostype: Option<String>,
                        pub swap: Option<u64>,
                        pub digest: Option<String>,
                        pub arch: Option<String>,
                        pub tty: Option<u64>,
                        #[serde(rename = "net[n]")]
                        pub net_n: Option<String>,
                        #[serde(rename = "mp[n]")]
                        pub mp_n: Option<String>,
                        pub onboot: Option<bool>,
                        pub searchdomain: Option<String>,
                        pub unprivileged: Option<bool>,
                        pub startup: Option<String>,
                        pub features: Option<String>,
                        pub revert: Option<String>,
                        pub rootfs: Option<String>,
                        #[serde(rename = "unused[n]")]
                        pub unused_n: Option<String>,
                        pub console: Option<bool>,
                        pub lock: Option<String>,
                        pub cpuunits: Option<u64>,
                        pub tags: Option<String>,
                        pub cpulimit: Option<f64>,
                    }
                    impl ConfigClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn config(&self) -> config::ConfigClient {
                        config::ConfigClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod status {
                    pub struct StatusClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StatusClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/status"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub subdir: String,
                    }
                    impl StatusClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod current {
                        pub struct CurrentClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl CurrentClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/current"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub maxmem: Option<u64>,
                            pub ha: (),
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub uptime: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub vmid: u64,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub lock: Option<String>,
                            pub status: String,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub tags: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub maxswap: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_number_optional",
                                deserialize_with = "::proxmox_api::deserialize_number_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub cpus: Option<f64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub maxdisk: Option<u64>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub name: Option<String>,
                        }
                        impl CurrentClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn current(&self) -> current::CurrentClient {
                            current::CurrentClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod start {
                        pub struct StartClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl StartClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/start"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub debug: Option<bool>,
                            pub skiplock: Option<bool>,
                        }
                        impl StartClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn start(&self) -> start::StartClient {
                            start::StartClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod stop {
                        pub struct StopClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl StopClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/stop"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub skiplock: Option<bool>,
                        }
                        impl StopClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn stop(&self) -> stop::StopClient {
                            stop::StopClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod shutdown {
                        pub struct ShutdownClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ShutdownClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/shutdown"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            #[serde(rename = "forceStop")]
                            pub forcestop: Option<bool>,
                            pub timeout: Option<u64>,
                        }
                        impl ShutdownClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn shutdown(&self) -> shutdown::ShutdownClient {
                            shutdown::ShutdownClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod suspend {
                        pub struct SuspendClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SuspendClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/suspend"),
                                }
                            }
                        }
                        impl SuspendClient {
                            pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn suspend(&self) -> suspend::SuspendClient {
                            suspend::SuspendClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod resume {
                        pub struct ResumeClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ResumeClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/resume"),
                                }
                            }
                        }
                        impl ResumeClient {
                            pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn resume(&self) -> resume::ResumeClient {
                            resume::ResumeClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod reboot {
                        pub struct RebootClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RebootClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/reboot"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub timeout: Option<u64>,
                        }
                        impl RebootClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn reboot(&self) -> reboot::RebootClient {
                            reboot::RebootClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn status(&self) -> status::StatusClient {
                        status::StatusClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod snapshot {
                    pub struct SnapshotClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl SnapshotClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/snapshot"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub snaptime: Option<u64>,
                        pub description: String,
                        pub name: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub parent: Option<String>,
                    }
                    impl SnapshotClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub description: Option<String>,
                        pub snapname: String,
                    }
                    impl SnapshotClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    pub mod snapname {
                        pub struct SnapnameClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl SnapnameClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                                snapname: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}/{}", parent_path, snapname),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct DELETEParams {
                            pub force: Option<bool>,
                        }
                        impl SnapnameClient {
                            pub fn delete(
                                &self,
                                params: DELETEParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.delete(&path, &params)
                            }
                        }
                        #[derive(
                            Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                        )]
                        pub struct GETReturnsItems {}
                        impl SnapnameClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod rollback {
                            pub struct RollbackClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl RollbackClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}{}", parent_path, "/rollback"),
                                    }
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct POSTParams {
                                pub start: Option<bool>,
                            }
                            impl RollbackClient {
                                pub fn post(
                                    &self,
                                    params: POSTParams,
                                ) -> Result<String, ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.post(&path, &params)
                                }
                            }
                        }
                        impl SnapnameClient {
                            pub fn rollback(&self) -> rollback::RollbackClient {
                                rollback::RollbackClient::new(self.client.clone(), &self.path)
                            }
                        }
                        pub mod config {
                            pub struct ConfigClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl ConfigClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}{}", parent_path, "/config"),
                                    }
                                }
                            }
                            impl ConfigClient {
                                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub description: Option<String>,
                            }
                            impl ConfigClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                        }
                        impl SnapnameClient {
                            pub fn config(&self) -> config::ConfigClient {
                                config::ConfigClient::new(self.client.clone(), &self.path)
                            }
                        }
                    }
                    impl SnapshotClient {
                        pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient {
                            snapname::SnapnameClient::new(self.client.clone(), &self.path, snapname)
                        }
                    }
                }
                impl VmidClient {
                    pub fn snapshot(&self) -> snapshot::SnapshotClient {
                        snapshot::SnapshotClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod firewall {
                    pub struct FirewallClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl FirewallClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/firewall"),
                            }
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl FirewallClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod rules {
                        pub struct RulesClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RulesClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/rules"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub dest: Option<String>,
                            pub digest: Option<String>,
                            pub dport: Option<String>,
                            pub log: Option<String>,
                            #[serde(rename = "macro")]
                            pub macro_def: Option<String>,
                            pub proto: Option<String>,
                            pub sport: Option<String>,
                            pub pos: Option<u64>,
                            pub source: Option<String>,
                            pub comment: Option<String>,
                            #[serde(rename = "icmp-type")]
                            pub icmp_type: Option<String>,
                            pub action: String,
                            #[serde(rename = "type")]
                            pub ty: String,
                            pub enable: Option<u64>,
                            pub iface: Option<String>,
                        }
                        impl RulesClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub pos: u64,
                        }
                        impl RulesClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod pos {
                            pub struct PosClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl PosClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    pos: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, pos),
                                    }
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub digest: Option<String>,
                            }
                            impl PosClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                            #[derive(
                                Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize,
                            )]
                            pub struct GETReturns {
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub dest: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub comment: Option<String>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int_optional",
                                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub ipversion: Option<u64>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int_optional",
                                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub enable: Option<u64>,
                                #[serde(rename = "macro")]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub macro_def: Option<String>,
                                #[serde(rename = "icmp-type")]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub icmp_type: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub sport: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub iface: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub proto: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub source: Option<String>,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub log: Option<String>,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_int",
                                    deserialize_with = "::proxmox_api::deserialize_int"
                                )]
                                pub pos: u64,
                                pub action: String,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub dport: Option<String>,
                                #[serde(rename = "type")]
                                pub ty: String,
                            }
                            impl PosClient {
                                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub enable: Option<u64>,
                                #[serde(rename = "macro")]
                                pub macro_def: Option<String>,
                                pub moveto: Option<u64>,
                                pub action: Option<String>,
                                pub dport: Option<String>,
                                pub digest: Option<String>,
                                pub sport: Option<String>,
                                #[serde(rename = "type")]
                                pub ty: Option<String>,
                                pub log: Option<String>,
                                pub iface: Option<String>,
                                pub comment: Option<String>,
                                pub dest: Option<String>,
                                #[serde(rename = "icmp-type")]
                                pub icmp_type: Option<String>,
                                pub source: Option<String>,
                                pub proto: Option<String>,
                                pub delete: Option<String>,
                            }
                            impl PosClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                        }
                        impl RulesClient {
                            pub fn pos(&self, pos: &str) -> pos::PosClient {
                                pos::PosClient::new(self.client.clone(), &self.path, pos)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn rules(&self) -> rules::RulesClient {
                            rules::RulesClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod aliases {
                        pub struct AliasesClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl AliasesClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/aliases"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub digest: String,
                            pub name: String,
                            pub cidr: String,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                        }
                        impl AliasesClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub comment: Option<String>,
                            pub name: String,
                            pub cidr: String,
                        }
                        impl AliasesClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        pub mod name {
                            pub struct NameClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl NameClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    name: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, name),
                                    }
                                }
                            }
                            impl NameClient {
                                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct PUTParams {
                                pub cidr: String,
                                pub digest: Option<String>,
                                pub comment: Option<String>,
                                pub rename: Option<String>,
                            }
                            impl NameClient {
                                pub fn put(
                                    &self,
                                    params: PUTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.put(&path, &params)
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub digest: Option<String>,
                            }
                            impl NameClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                        }
                        impl AliasesClient {
                            pub fn name(&self, name: &str) -> name::NameClient {
                                name::NameClient::new(self.client.clone(), &self.path, name)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn aliases(&self) -> aliases::AliasesClient {
                            aliases::AliasesClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod ipset {
                        pub struct IpsetClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl IpsetClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/ipset"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub rename: Option<String>,
                            pub comment: Option<String>,
                            pub digest: Option<String>,
                            pub name: String,
                        }
                        impl IpsetClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                            pub digest: String,
                            pub name: String,
                        }
                        impl IpsetClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        pub mod name {
                            pub struct NameClient {
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                path: String,
                            }
                            impl NameClient {
                                pub fn new(
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    parent_path: &str,
                                    name: &str,
                                ) -> Self {
                                    Self {
                                        client,
                                        path: format!("{}/{}", parent_path, name),
                                    }
                                }
                            }
                            #[derive(
                                Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize,
                            )]
                            pub struct GETReturnsItems {
                                pub digest: String,
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub comment: Option<String>,
                                pub cidr: String,
                                #[serde(
                                    serialize_with = "::proxmox_api::serialize_bool_optional",
                                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                                )]
                                #[serde(skip_serializing_if = "Option::is_none", default)]
                                pub nomatch: Option<bool>,
                            }
                            impl NameClient {
                                pub fn get(
                                    &self,
                                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.get(&path, &())
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct POSTParams {
                                pub nomatch: Option<bool>,
                                pub comment: Option<String>,
                                pub cidr: String,
                            }
                            impl NameClient {
                                pub fn post(
                                    &self,
                                    params: POSTParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.post(&path, &params)
                                }
                            }
                            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                            pub struct DELETEParams {
                                pub force: Option<bool>,
                            }
                            impl NameClient {
                                pub fn delete(
                                    &self,
                                    params: DELETEParams,
                                ) -> Result<(), ::proxmox_api::Error>
                                {
                                    let path = self.path.to_string();
                                    self.client.delete(&path, &params)
                                }
                            }
                            pub mod cidr {
                                pub struct CidrClient {
                                    client: ::std::sync::Arc<::proxmox_api::Client>,
                                    path: String,
                                }
                                impl CidrClient {
                                    pub fn new(
                                        client: ::std::sync::Arc<::proxmox_api::Client>,
                                        parent_path: &str,
                                        cidr: &str,
                                    ) -> Self {
                                        Self {
                                            client,
                                            path: format!("{}/{}", parent_path, cidr),
                                        }
                                    }
                                }
                                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                                pub struct DELETEParams {
                                    pub digest: Option<String>,
                                }
                                impl CidrClient {
                                    pub fn delete(
                                        &self,
                                        params: DELETEParams,
                                    ) -> Result<(), ::proxmox_api::Error>
                                    {
                                        let path = self.path.to_string();
                                        self.client.delete(&path, &params)
                                    }
                                }
                                impl CidrClient {
                                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                                        let path = self.path.to_string();
                                        self.client.get(&path, &())
                                    }
                                }
                                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                                pub struct PUTParams {
                                    pub nomatch: Option<bool>,
                                    pub comment: Option<String>,
                                    pub digest: Option<String>,
                                }
                                impl CidrClient {
                                    pub fn put(
                                        &self,
                                        params: PUTParams,
                                    ) -> Result<(), ::proxmox_api::Error>
                                    {
                                        let path = self.path.to_string();
                                        self.client.put(&path, &params)
                                    }
                                }
                            }
                            impl NameClient {
                                pub fn cidr(&self, cidr: &str) -> cidr::CidrClient {
                                    cidr::CidrClient::new(self.client.clone(), &self.path, cidr)
                                }
                            }
                        }
                        impl IpsetClient {
                            pub fn name(&self, name: &str) -> name::NameClient {
                                name::NameClient::new(self.client.clone(), &self.path, name)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn ipset(&self) -> ipset::IpsetClient {
                            ipset::IpsetClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod options {
                        pub struct OptionsClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl OptionsClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/options"),
                                }
                            }
                        }
                        #[derive(
                            Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                        )]
                        pub struct GETReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub macfilter: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub enable: Option<bool>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub log_level_in: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub ndp: Option<bool>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub policy_in: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub policy_out: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub dhcp: Option<bool>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub log_level_out: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub radv: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub ipfilter: Option<bool>,
                        }
                        impl OptionsClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct PUTParams {
                            pub ndp: Option<bool>,
                            pub dhcp: Option<bool>,
                            pub radv: Option<bool>,
                            pub delete: Option<String>,
                            pub policy_in: Option<String>,
                            pub policy_out: Option<String>,
                            pub ipfilter: Option<bool>,
                            pub macfilter: Option<bool>,
                            pub log_level_in: Option<String>,
                            pub log_level_out: Option<String>,
                            pub digest: Option<String>,
                            pub enable: Option<bool>,
                        }
                        impl OptionsClient {
                            pub fn put(
                                &self,
                                params: PUTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.put(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn options(&self) -> options::OptionsClient {
                            options::OptionsClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod log {
                        pub struct LogClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl LogClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/log"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub since: Option<u64>,
                            pub until: Option<u64>,
                            pub limit: Option<u64>,
                            pub start: Option<u64>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub t: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub n: u64,
                        }
                        impl LogClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn log(&self) -> log::LogClient {
                            log::LogClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod refs {
                        pub struct RefsClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl RefsClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/refs"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            #[serde(rename = "type")]
                            pub ty: Option<String>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub comment: Option<String>,
                            #[serde(rename = "type")]
                            pub ty: String,
                            pub name: String,
                        }
                        impl RefsClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FirewallClient {
                        pub fn refs(&self) -> refs::RefsClient {
                            refs::RefsClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl VmidClient {
                    pub fn firewall(&self) -> firewall::FirewallClient {
                        firewall::FirewallClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrd {
                    pub struct RrdClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrdClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrd"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub ds: String,
                        pub timeframe: String,
                        pub cf: Option<String>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub filename: String,
                    }
                    impl RrdClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn rrd(&self) -> rrd::RrdClient {
                        rrd::RrdClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrddata {
                    pub struct RrddataClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrddataClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrddata"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub timeframe: String,
                        pub cf: Option<String>,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl RrddataClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn rrddata(&self) -> rrddata::RrddataClient {
                        rrddata::RrddataClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod vncproxy {
                    pub struct VncproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl VncproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/vncproxy"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub width: Option<u64>,
                        pub websocket: Option<bool>,
                        pub height: Option<u64>,
                    }
                    impl VncproxyClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn vncproxy(&self) -> vncproxy::VncproxyClient {
                        vncproxy::VncproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod termproxy {
                    pub struct TermproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl TermproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/termproxy"),
                            }
                        }
                    }
                    impl TermproxyClient {
                        pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl VmidClient {
                    pub fn termproxy(&self) -> termproxy::TermproxyClient {
                        termproxy::TermproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod vncwebsocket {
                    pub struct VncwebsocketClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl VncwebsocketClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/vncwebsocket"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub port: u64,
                        pub vncticket: String,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub port: String,
                    }
                    impl VncwebsocketClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient {
                        vncwebsocket::VncwebsocketClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod spiceproxy {
                    pub struct SpiceproxyClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl SpiceproxyClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/spiceproxy"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub proxy: Option<String>,
                    }
                    impl SpiceproxyClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn spiceproxy(&self) -> spiceproxy::SpiceproxyClient {
                        spiceproxy::SpiceproxyClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod remote_migrate {
                    pub struct RemoteMigrateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RemoteMigrateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/remote_migrate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub bwlimit: Option<f64>,
                        pub delete: Option<bool>,
                        pub online: Option<bool>,
                        #[serde(rename = "target-bridge")]
                        pub target_bridge: String,
                        #[serde(rename = "target-storage")]
                        pub target_storage: String,
                        #[serde(rename = "target-vmid")]
                        pub target_vmid: Option<u64>,
                        #[serde(rename = "target-endpoint")]
                        pub target_endpoint: String,
                        pub timeout: Option<u64>,
                        pub restart: Option<bool>,
                    }
                    impl RemoteMigrateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn remote_migrate(&self) -> remote_migrate::RemoteMigrateClient {
                        remote_migrate::RemoteMigrateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod migrate {
                    pub struct MigrateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MigrateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/migrate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub target: String,
                        pub timeout: Option<u64>,
                        pub online: Option<bool>,
                        #[serde(rename = "target-storage")]
                        pub target_storage: Option<String>,
                        pub bwlimit: Option<f64>,
                        pub restart: Option<bool>,
                    }
                    impl MigrateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn migrate(&self) -> migrate::MigrateClient {
                        migrate::MigrateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod feature {
                    pub struct FeatureClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl FeatureClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/feature"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub feature: String,
                        pub snapname: Option<String>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(rename = "hasFeature")]
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub hasfeature: bool,
                    }
                    impl FeatureClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn feature(&self) -> feature::FeatureClient {
                        feature::FeatureClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod template {
                    pub struct TemplateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl TemplateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/template"),
                            }
                        }
                    }
                    impl TemplateClient {
                        pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl VmidClient {
                    pub fn template(&self) -> template::TemplateClient {
                        template::TemplateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod clone {
                    pub struct CloneClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CloneClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/clone"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub bwlimit: Option<f64>,
                        pub hostname: Option<String>,
                        pub full: Option<bool>,
                        pub storage: Option<String>,
                        pub snapname: Option<String>,
                        pub target: Option<String>,
                        pub pool: Option<String>,
                        pub newid: u64,
                        pub description: Option<String>,
                    }
                    impl CloneClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn clone(&self) -> clone::CloneClient {
                        clone::CloneClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod resize {
                    pub struct ResizeClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ResizeClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/resize"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub digest: Option<String>,
                        pub size: String,
                        pub disk: String,
                    }
                    impl ResizeClient {
                        pub fn put(
                            &self,
                            params: PUTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn resize(&self) -> resize::ResizeClient {
                        resize::ResizeClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod move_volume {
                    pub struct MoveVolumeClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MoveVolumeClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/move_volume"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        #[serde(rename = "target-volume")]
                        pub target_volume: Option<String>,
                        pub delete: Option<bool>,
                        pub volume: String,
                        #[serde(rename = "target-vmid")]
                        pub target_vmid: Option<u64>,
                        pub digest: Option<String>,
                        #[serde(rename = "target-digest")]
                        pub target_digest: Option<String>,
                        pub bwlimit: Option<f64>,
                        pub storage: Option<String>,
                    }
                    impl MoveVolumeClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn move_volume(&self) -> move_volume::MoveVolumeClient {
                        move_volume::MoveVolumeClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod pending {
                    pub struct PendingClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PendingClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/pending"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub delete: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub value: Option<String>,
                        pub key: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub pending: Option<String>,
                    }
                    impl PendingClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl VmidClient {
                    pub fn pending(&self) -> pending::PendingClient {
                        pending::PendingClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod mtunnel {
                    pub struct MtunnelClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MtunnelClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/mtunnel"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub bridges: Option<String>,
                        pub storages: Option<String>,
                    }
                    impl MtunnelClient {
                        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn mtunnel(&self) -> mtunnel::MtunnelClient {
                        mtunnel::MtunnelClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod mtunnelwebsocket {
                    pub struct MtunnelwebsocketClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MtunnelwebsocketClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/mtunnelwebsocket"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub socket: String,
                        pub ticket: String,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub socket: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub port: Option<String>,
                    }
                    impl MtunnelwebsocketClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl VmidClient {
                    pub fn mtunnelwebsocket(&self) -> mtunnelwebsocket::MtunnelwebsocketClient {
                        mtunnelwebsocket::MtunnelwebsocketClient::new(
                            self.client.clone(),
                            &self.path,
                        )
                    }
                }
            }
            impl LxcClient {
                pub fn vmid(&self, vmid: &str) -> vmid::VmidClient {
                    vmid::VmidClient::new(self.client.clone(), &self.path, vmid)
                }
            }
        }
        impl NodeClient {
            pub fn lxc(&self) -> lxc::LxcClient {
                lxc::LxcClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod ceph {
            pub struct CephClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl CephClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/ceph"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl CephClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod cfg {
                pub struct CfgClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CfgClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/cfg"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl CfgClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod raw {
                    pub struct RawClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RawClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/raw"),
                            }
                        }
                    }
                    impl RawClient {
                        pub fn get(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl CfgClient {
                    pub fn raw(&self) -> raw::RawClient {
                        raw::RawClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod db {
                    pub struct DbClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl DbClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/db"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub can_update_at_runtime: bool,
                        pub name: String,
                        pub value: String,
                        pub section: String,
                        pub level: String,
                        pub mask: String,
                    }
                    impl DbClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl CfgClient {
                    pub fn db(&self) -> db::DbClient {
                        db::DbClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl CephClient {
                pub fn cfg(&self) -> cfg::CfgClient {
                    cfg::CfgClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod osd {
                pub struct OsdClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl OsdClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/osd"),
                        }
                    }
                }
                impl OsdClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub dev: String,
                    pub wal_dev_size: Option<f64>,
                    pub wal_dev: Option<String>,
                    #[serde(rename = "crush-device-class")]
                    pub crush_device_class: Option<String>,
                    pub encrypted: Option<bool>,
                    pub db_dev: Option<String>,
                    pub db_dev_size: Option<f64>,
                }
                impl OsdClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                pub mod osdid {
                    pub struct OsdidClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl OsdidClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            osdid: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, osdid),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub cleanup: Option<bool>,
                    }
                    impl OsdidClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl OsdidClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod metadata {
                        pub struct MetadataClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl MetadataClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/metadata"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsOsd {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub id: u64,
                            pub osd_data: String,
                            pub version: String,
                            pub hb_front_addr: String,
                            pub hostname: String,
                            pub back_addr: String,
                            pub front_addr: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub pid: u64,
                            pub hb_back_addr: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub mem_usage: u64,
                            pub osd_objectstore: String,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsDevicesItems {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub size: u64,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub support_discard: bool,
                            pub devices: String,
                            #[serde(rename = "type")]
                            pub ty: String,
                            pub dev_node: String,
                            pub device: String,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            pub osd: GETReturnsOsd,
                            pub devices: Vec<GETReturnsDevicesItems>,
                        }
                        impl MetadataClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl OsdidClient {
                        pub fn metadata(&self) -> metadata::MetadataClient {
                            metadata::MetadataClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod lv_info {
                        pub struct LvInfoClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl LvInfoClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/lv-info"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            #[serde(rename = "type")]
                            pub ty: Option<String>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            pub lv_name: String,
                            pub lv_path: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub lv_size: u64,
                            pub lv_uuid: String,
                            pub vg_name: String,
                            pub creation_time: String,
                        }
                        impl LvInfoClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<GETReturns, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl OsdidClient {
                        pub fn lv_info(&self) -> lv_info::LvInfoClient {
                            lv_info::LvInfoClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod in_name {
                        pub struct InClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl InClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/in"),
                                }
                            }
                        }
                        impl InClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl OsdidClient {
                        pub fn in_name(&self) -> in_name::InClient {
                            in_name::InClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod out {
                        pub struct OutClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl OutClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/out"),
                                }
                            }
                        }
                        impl OutClient {
                            pub fn post(&self) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &())
                            }
                        }
                    }
                    impl OsdidClient {
                        pub fn out(&self) -> out::OutClient {
                            out::OutClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod scrub {
                        pub struct ScrubClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ScrubClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/scrub"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub deep: Option<bool>,
                        }
                        impl ScrubClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                    }
                    impl OsdidClient {
                        pub fn scrub(&self) -> scrub::ScrubClient {
                            scrub::ScrubClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl OsdClient {
                    pub fn osdid(&self, osdid: &str) -> osdid::OsdidClient {
                        osdid::OsdidClient::new(self.client.clone(), &self.path, osdid)
                    }
                }
            }
            impl CephClient {
                pub fn osd(&self) -> osd::OsdClient {
                    osd::OsdClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod mds {
                pub struct MdsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl MdsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/mds"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub name: (),
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub addr: Option<String>,
                    pub state: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub standby_replay: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub rank: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub host: Option<String>,
                }
                impl MdsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    impl NameClient {
                        pub fn delete(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub hotstandby: Option<bool>,
                    }
                    impl NameClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl MdsClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl CephClient {
                pub fn mds(&self) -> mds::MdsClient {
                    mds::MdsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod mgr {
                pub struct MgrClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl MgrClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/mgr"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub state: String,
                    pub name: (),
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub host: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub addr: Option<String>,
                }
                impl MgrClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod id {
                    pub struct IdClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl IdClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            id: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, id),
                            }
                        }
                    }
                    impl IdClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                    impl IdClient {
                        pub fn delete(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &())
                        }
                    }
                }
                impl MgrClient {
                    pub fn id(&self, id: &str) -> id::IdClient {
                        id::IdClient::new(self.client.clone(), &self.path, id)
                    }
                }
            }
            impl CephClient {
                pub fn mgr(&self) -> mgr::MgrClient {
                    mgr::MgrClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod mon {
                pub struct MonClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl MonClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/mon"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ceph_version_short: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub state: Option<String>,
                    pub name: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub direxists: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub host: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub addr: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub quorum: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub rank: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub service: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ceph_version: Option<String>,
                }
                impl MonClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod monid {
                    pub struct MonidClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MonidClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            monid: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, monid),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        #[serde(rename = "mon-address")]
                        pub mon_address: Option<String>,
                    }
                    impl MonidClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    impl MonidClient {
                        pub fn delete(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &())
                        }
                    }
                }
                impl MonClient {
                    pub fn monid(&self, monid: &str) -> monid::MonidClient {
                        monid::MonidClient::new(self.client.clone(), &self.path, monid)
                    }
                }
            }
            impl CephClient {
                pub fn mon(&self) -> mon::MonClient {
                    mon::MonClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod fs {
                pub struct FsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl FsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/fs"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub metadata_pool: String,
                    pub data_pool: String,
                    pub name: String,
                }
                impl FsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub pg_num: Option<u64>,
                        #[serde(rename = "add-storage")]
                        pub add_storage: Option<bool>,
                    }
                    impl NameClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl FsClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl CephClient {
                pub fn fs(&self) -> fs::FsClient {
                    fs::FsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod pool {
                pub struct PoolClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl PoolClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/pool"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub application: Option<String>,
                    #[serde(rename = "erasure-coding")]
                    pub erasure_coding: Option<String>,
                    pub pg_autoscale_mode: Option<String>,
                    pub min_size: Option<u64>,
                    pub pg_num: Option<u64>,
                    pub size: Option<u64>,
                    pub name: String,
                    pub target_size: Option<String>,
                    pub crush_rule: Option<String>,
                    pub target_size_ratio: Option<f64>,
                    pub pg_num_min: Option<u64>,
                    pub add_storages: Option<bool>,
                }
                impl PoolClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_number_optional",
                        deserialize_with = "::proxmox_api::deserialize_number_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub target_size_ratio: Option<f64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub crush_rule: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_num_final: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pg_num: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_num_min: Option<u64>,
                    pub crush_rule_name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub target_size: Option<u64>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub min_size: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub autoscale_status: Option<()>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_number",
                        deserialize_with = "::proxmox_api::deserialize_number"
                    )]
                    pub percent_used: f64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_autoscale_mode: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub bytes_used: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub application_metadata: Option<()>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pool: u64,
                    pub pool_name: String,
                }
                impl PoolClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub force: Option<bool>,
                        pub remove_ecprofile: Option<bool>,
                        pub remove_storages: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl NameClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub pg_num_min: Option<u64>,
                        pub size: Option<u64>,
                        pub target_size_ratio: Option<f64>,
                        pub pg_num: Option<u64>,
                        pub target_size: Option<String>,
                        pub pg_autoscale_mode: Option<String>,
                        pub application: Option<String>,
                        pub crush_rule: Option<String>,
                        pub min_size: Option<u64>,
                    }
                    impl NameClient {
                        pub fn put(
                            &self,
                            params: PUTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    pub mod status {
                        pub struct StatusClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl StatusClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/status"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub verbose: Option<bool>,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub application_list: Option<()>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub crush_rule: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub pg_num_min: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_number_optional",
                                deserialize_with = "::proxmox_api::deserialize_number_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub target_size_ratio: Option<f64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub write_fadvise_dontneed: bool,
                            #[serde(rename = "nodeep-scrub")]
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub nodeep_scrub: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub noscrub: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub hashpspool: bool,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub statistics: Option<()>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub id: u64,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub pg_autoscale_mode: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub nosizechange: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub nodelete: bool,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub application: Option<String>,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub autoscale_status: Option<()>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub min_size: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub pg_num: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub pgp_num: u64,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub target_size: Option<String>,
                            pub name: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub size: Option<u64>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub fast_read: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub nopgchange: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub use_gmt_hitset: bool,
                        }
                        impl StatusClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<GETReturns, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl NameClient {
                        pub fn status(&self) -> status::StatusClient {
                            status::StatusClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl PoolClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl CephClient {
                pub fn pool(&self) -> pool::PoolClient {
                    pool::PoolClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod pools {
                pub struct PoolsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl PoolsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/pools"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub pg_num: Option<u64>,
                    pub pg_num_min: Option<u64>,
                    pub pg_autoscale_mode: Option<String>,
                    pub size: Option<u64>,
                    pub crush_rule: Option<String>,
                    #[serde(rename = "erasure-coding")]
                    pub erasure_coding: Option<String>,
                    pub target_size: Option<String>,
                    pub target_size_ratio: Option<f64>,
                    pub application: Option<String>,
                    pub add_storages: Option<bool>,
                    pub min_size: Option<u64>,
                    pub name: String,
                }
                impl PoolsClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub crush_rule_name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub bytes_used: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub min_size: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_number_optional",
                        deserialize_with = "::proxmox_api::deserialize_number_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub target_size_ratio: Option<f64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub application_metadata: Option<()>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub autoscale_status: Option<()>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_number",
                        deserialize_with = "::proxmox_api::deserialize_number"
                    )]
                    pub percent_used: f64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pool: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_num_final: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub crush_rule: u64,
                    pub pool_name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub target_size: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pg_num: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_num_min: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pg_autoscale_mode: Option<String>,
                }
                impl PoolsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub force: Option<bool>,
                        pub remove_ecprofile: Option<bool>,
                        pub remove_storages: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub application: Option<String>,
                        pub crush_rule: Option<String>,
                        pub min_size: Option<u64>,
                        pub pg_autoscale_mode: Option<String>,
                        pub target_size: Option<String>,
                        pub target_size_ratio: Option<f64>,
                        pub size: Option<u64>,
                        pub pg_num: Option<u64>,
                        pub pg_num_min: Option<u64>,
                    }
                    impl NameClient {
                        pub fn put(
                            &self,
                            params: PUTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub verbose: Option<bool>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub id: u64,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub nopgchange: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub noscrub: bool,
                        pub name: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub pg_autoscale_mode: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub write_fadvise_dontneed: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub fast_read: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub nodelete: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub min_size: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub pg_num_min: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub target_size_ratio: Option<f64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub pg_num: Option<u64>,
                        #[serde(rename = "nodeep-scrub")]
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub nodeep_scrub: bool,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub crush_rule: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub application: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub autoscale_status: Option<()>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub hashpspool: bool,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub statistics: Option<()>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub target_size: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub application_list: Option<()>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub nosizechange: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub size: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub use_gmt_hitset: bool,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub pgp_num: u64,
                    }
                    impl NameClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl PoolsClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl CephClient {
                pub fn pools(&self) -> pools::PoolsClient {
                    pools::PoolsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod config {
                pub struct ConfigClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ConfigClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/config"),
                        }
                    }
                }
                impl ConfigClient {
                    pub fn get(&self) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CephClient {
                pub fn config(&self) -> config::ConfigClient {
                    config::ConfigClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod configdb {
                pub struct ConfigdbClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ConfigdbClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/configdb"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub value: String,
                    pub mask: String,
                    pub section: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub can_update_at_runtime: bool,
                    pub name: String,
                    pub level: String,
                }
                impl ConfigdbClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CephClient {
                pub fn configdb(&self) -> configdb::ConfigdbClient {
                    configdb::ConfigdbClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod init {
                pub struct InitClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl InitClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/init"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub min_size: Option<u64>,
                    pub size: Option<u64>,
                    #[serde(rename = "cluster-network")]
                    pub cluster_network: Option<String>,
                    pub disable_cephx: Option<bool>,
                    pub network: Option<String>,
                    pub pg_bits: Option<u64>,
                }
                impl InitClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn init(&self) -> init::InitClient {
                    init::InitClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod stop {
                pub struct StopClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl StopClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/stop"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub service: Option<String>,
                }
                impl StopClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn stop(&self) -> stop::StopClient {
                    stop::StopClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod start {
                pub struct StartClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl StartClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/start"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub service: Option<String>,
                }
                impl StartClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn start(&self) -> start::StartClient {
                    start::StartClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod restart {
                pub struct RestartClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl RestartClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/restart"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub service: Option<String>,
                }
                impl RestartClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn restart(&self) -> restart::RestartClient {
                    restart::RestartClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod status {
                pub struct StatusClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl StatusClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/status"),
                        }
                    }
                }
                impl StatusClient {
                    pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CephClient {
                pub fn status(&self) -> status::StatusClient {
                    status::StatusClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod crush {
                pub struct CrushClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CrushClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/crush"),
                        }
                    }
                }
                impl CrushClient {
                    pub fn get(&self) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CephClient {
                pub fn crush(&self) -> crush::CrushClient {
                    crush::CrushClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod log {
                pub struct LogClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LogClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/log"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub limit: Option<u64>,
                    pub start: Option<u64>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub n: u64,
                    pub t: String,
                }
                impl LogClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn log(&self) -> log::LogClient {
                    log::LogClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod rules {
                pub struct RulesClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl RulesClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/rules"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub name: String,
                }
                impl RulesClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CephClient {
                pub fn rules(&self) -> rules::RulesClient {
                    rules::RulesClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod cmd_safety {
                pub struct CmdSafetyClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CmdSafetyClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/cmd-safety"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub id: String,
                    pub service: String,
                    pub action: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub status: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub safe: bool,
                }
                impl CmdSafetyClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl CephClient {
                pub fn cmd_safety(&self) -> cmd_safety::CmdSafetyClient {
                    cmd_safety::CmdSafetyClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn ceph(&self) -> ceph::CephClient {
                ceph::CephClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod vzdump {
            pub struct VzdumpClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl VzdumpClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/vzdump"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
            pub struct POSTParams {
                pub mode: Option<String>,
                pub pool: Option<String>,
                pub ionice: Option<u64>,
                pub quiet: Option<bool>,
                pub stdexcludes: Option<bool>,
                pub tmpdir: Option<String>,
                pub compress: Option<String>,
                pub bwlimit: Option<u64>,
                pub exclude: Option<String>,
                pub storage: Option<String>,
                pub stopwait: Option<u64>,
                #[serde(rename = "exclude-path")]
                pub exclude_path: Option<String>,
                pub all: Option<bool>,
                pub stop: Option<bool>,
                pub stdout: Option<bool>,
                pub vmid: Option<String>,
                #[serde(rename = "notes-template")]
                pub notes_template: Option<String>,
                pub pigz: Option<u64>,
                pub mailto: Option<String>,
                pub maxfiles: Option<u64>,
                #[serde(rename = "prune-backups")]
                pub prune_backups: Option<String>,
                pub script: Option<String>,
                pub zstd: Option<u64>,
                pub protected: Option<bool>,
                pub remove: Option<bool>,
                pub performance: Option<String>,
                pub mailnotification: Option<String>,
                pub lockwait: Option<u64>,
                pub dumpdir: Option<String>,
            }
            impl VzdumpClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod defaults {
                pub struct DefaultsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl DefaultsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/defaults"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub storage: Option<String>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturns {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub remove: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub script: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub mode: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub stopwait: Option<u64>,
                    #[serde(rename = "prune-backups")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub prune_backups: Option<String>,
                    #[serde(rename = "notes-template")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub notes_template: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub compress: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub storage: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub stdexcludes: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub vmid: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub mailto: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub quiet: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub performance: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub tmpdir: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub dumpdir: Option<String>,
                    #[serde(rename = "exclude-path")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub exclude_path: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub mailnotification: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub zstd: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub stop: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub maxfiles: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ionice: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub node: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub exclude: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pool: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub all: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub lockwait: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub protected: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub bwlimit: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pigz: Option<u64>,
                }
                impl DefaultsClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl VzdumpClient {
                pub fn defaults(&self) -> defaults::DefaultsClient {
                    defaults::DefaultsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod extractconfig {
                pub struct ExtractconfigClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ExtractconfigClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/extractconfig"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub volume: String,
                }
                impl ExtractconfigClient {
                    pub fn get(&self, params: GETParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl VzdumpClient {
                pub fn extractconfig(&self) -> extractconfig::ExtractconfigClient {
                    extractconfig::ExtractconfigClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn vzdump(&self) -> vzdump::VzdumpClient {
                vzdump::VzdumpClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod services {
            pub struct ServicesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ServicesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/services"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl ServicesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod service {
                pub struct ServiceClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ServiceClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        service: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, service),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub subdir: String,
                }
                impl ServiceClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod state {
                    pub struct StateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/state"),
                            }
                        }
                    }
                    impl StateClient {
                        pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl ServiceClient {
                    pub fn state(&self) -> state::StateClient {
                        state::StateClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod start {
                    pub struct StartClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StartClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/start"),
                            }
                        }
                    }
                    impl StartClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl ServiceClient {
                    pub fn start(&self) -> start::StartClient {
                        start::StartClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod stop {
                    pub struct StopClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StopClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/stop"),
                            }
                        }
                    }
                    impl StopClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl ServiceClient {
                    pub fn stop(&self) -> stop::StopClient {
                        stop::StopClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod restart {
                    pub struct RestartClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RestartClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/restart"),
                            }
                        }
                    }
                    impl RestartClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl ServiceClient {
                    pub fn restart(&self) -> restart::RestartClient {
                        restart::RestartClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod reload {
                    pub struct ReloadClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ReloadClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/reload"),
                            }
                        }
                    }
                    impl ReloadClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl ServiceClient {
                    pub fn reload(&self) -> reload::ReloadClient {
                        reload::ReloadClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl ServicesClient {
                pub fn service(&self, service: &str) -> service::ServiceClient {
                    service::ServiceClient::new(self.client.clone(), &self.path, service)
                }
            }
        }
        impl NodeClient {
            pub fn services(&self) -> services::ServicesClient {
                services::ServicesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod subscription {
            pub struct SubscriptionClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl SubscriptionClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/subscription"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub force: Option<bool>,
            }
            impl SubscriptionClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub key: String,
            }
            impl SubscriptionClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            impl SubscriptionClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            impl SubscriptionClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn subscription(&self) -> subscription::SubscriptionClient {
                subscription::SubscriptionClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod network {
            pub struct NetworkClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl NetworkClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/network"),
                    }
                }
            }
            impl NetworkClient {
                pub fn put(&self) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &())
                }
            }
            impl NetworkClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                #[serde(rename = "type")]
                pub ty: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl NetworkClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                #[serde(rename = "vlan-id")]
                pub vlan_id: Option<u64>,
                pub cidr6: Option<String>,
                #[serde(rename = "vlan-raw-device")]
                pub vlan_raw_device: Option<String>,
                pub ovs_bonds: Option<String>,
                pub ovs_tag: Option<u64>,
                pub bridge_ports: Option<String>,
                pub address6: Option<String>,
                pub address: Option<String>,
                #[serde(rename = "bond-primary")]
                pub bond_primary: Option<String>,
                pub bond_mode: Option<String>,
                pub gateway6: Option<String>,
                pub gateway: Option<String>,
                pub iface: String,
                pub netmask: Option<String>,
                pub netmask6: Option<u64>,
                pub autostart: Option<bool>,
                pub comments: Option<String>,
                pub comments6: Option<String>,
                pub mtu: Option<u64>,
                pub bond_xmit_hash_policy: Option<String>,
                pub cidr: Option<String>,
                pub ovs_options: Option<String>,
                pub ovs_ports: Option<String>,
                pub bridge_vlan_aware: Option<bool>,
                pub slaves: Option<String>,
                #[serde(rename = "type")]
                pub ty: String,
                pub ovs_bridge: Option<String>,
            }
            impl NetworkClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            pub mod iface {
                pub struct IfaceClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IfaceClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        iface: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, iface),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(rename = "type")]
                    pub ty: String,
                    pub method: String,
                }
                impl IfaceClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub slaves: Option<String>,
                    pub bridge_vlan_aware: Option<bool>,
                    pub netmask6: Option<u64>,
                    pub ovs_ports: Option<String>,
                    pub cidr6: Option<String>,
                    pub netmask: Option<String>,
                    #[serde(rename = "vlan-id")]
                    pub vlan_id: Option<u64>,
                    pub ovs_bonds: Option<String>,
                    pub bond_xmit_hash_policy: Option<String>,
                    pub gateway: Option<String>,
                    pub address: Option<String>,
                    pub ovs_options: Option<String>,
                    pub bridge_ports: Option<String>,
                    pub comments6: Option<String>,
                    pub comments: Option<String>,
                    pub delete: Option<String>,
                    pub gateway6: Option<String>,
                    #[serde(rename = "vlan-raw-device")]
                    pub vlan_raw_device: Option<String>,
                    pub ovs_bridge: Option<String>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(rename = "bond-primary")]
                    pub bond_primary: Option<String>,
                    pub mtu: Option<u64>,
                    pub cidr: Option<String>,
                    pub address6: Option<String>,
                    pub autostart: Option<bool>,
                    pub bond_mode: Option<String>,
                    pub ovs_tag: Option<u64>,
                }
                impl IfaceClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
                impl IfaceClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
            }
            impl NetworkClient {
                pub fn iface(&self, iface: &str) -> iface::IfaceClient {
                    iface::IfaceClient::new(self.client.clone(), &self.path, iface)
                }
            }
        }
        impl NodeClient {
            pub fn network(&self) -> network::NetworkClient {
                network::NetworkClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod tasks {
            pub struct TasksClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl TasksClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/tasks"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub start: Option<u64>,
                pub typefilter: Option<String>,
                pub userfilter: Option<String>,
                pub statusfilter: Option<String>,
                pub limit: Option<u64>,
                pub until: Option<u64>,
                pub vmid: Option<u64>,
                pub since: Option<u64>,
                pub source: Option<String>,
                pub errors: Option<bool>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub starttime: u64,
                #[serde(rename = "type")]
                pub ty: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub status: Option<String>,
                pub upid: String,
                pub node: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub pid: u64,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub pstart: u64,
                pub id: String,
                pub user: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub endtime: Option<u64>,
            }
            impl TasksClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            pub mod upid {
                pub struct UpidClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl UpidClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        upid: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, upid),
                        }
                    }
                }
                impl UpidClient {
                    pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &())
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl UpidClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod log {
                    pub struct LogClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl LogClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/log"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub limit: Option<u64>,
                        pub start: Option<u64>,
                        pub download: Option<bool>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub t: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub n: u64,
                    }
                    impl LogClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl UpidClient {
                    pub fn log(&self) -> log::LogClient {
                        log::LogClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod status {
                    pub struct StatusClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StatusClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/status"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub exitstatus: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub pid: u64,
                        pub node: String,
                        #[serde(rename = "type")]
                        pub ty: String,
                        pub upid: String,
                        pub id: String,
                        pub user: String,
                        pub status: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number",
                            deserialize_with = "::proxmox_api::deserialize_number"
                        )]
                        pub starttime: f64,
                    }
                    impl StatusClient {
                        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl UpidClient {
                    pub fn status(&self) -> status::StatusClient {
                        status::StatusClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl TasksClient {
                pub fn upid(&self, upid: &str) -> upid::UpidClient {
                    upid::UpidClient::new(self.client.clone(), &self.path, upid)
                }
            }
        }
        impl NodeClient {
            pub fn tasks(&self) -> tasks::TasksClient {
                tasks::TasksClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod scan {
            pub struct ScanClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ScanClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/scan"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub method: String,
            }
            impl ScanClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod nfs {
                pub struct NfsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl NfsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/nfs"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub server: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub path: String,
                    pub options: String,
                }
                impl NfsClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn nfs(&self) -> nfs::NfsClient {
                    nfs::NfsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod cifs {
                pub struct CifsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CifsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/cifs"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub domain: Option<String>,
                    pub password: Option<String>,
                    pub server: String,
                    pub username: Option<String>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub description: String,
                    pub share: String,
                }
                impl CifsClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn cifs(&self) -> cifs::CifsClient {
                    cifs::CifsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod pbs {
                pub struct PbsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl PbsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/pbs"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub fingerprint: Option<String>,
                    pub password: String,
                    pub username: String,
                    pub server: String,
                    pub port: Option<u64>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub store: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                }
                impl PbsClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn pbs(&self) -> pbs::PbsClient {
                    pbs::PbsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod glusterfs {
                pub struct GlusterfsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl GlusterfsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/glusterfs"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub server: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub volname: String,
                }
                impl GlusterfsClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn glusterfs(&self) -> glusterfs::GlusterfsClient {
                    glusterfs::GlusterfsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod iscsi {
                pub struct IscsiClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IscsiClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/iscsi"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub portal: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub portal: String,
                    pub target: String,
                }
                impl IscsiClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn iscsi(&self) -> iscsi::IscsiClient {
                    iscsi::IscsiClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod lvm {
                pub struct LvmClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LvmClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/lvm"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub vg: String,
                }
                impl LvmClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl ScanClient {
                pub fn lvm(&self) -> lvm::LvmClient {
                    lvm::LvmClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod lvmthin {
                pub struct LvmthinClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LvmthinClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/lvmthin"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub vg: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub lv: String,
                }
                impl LvmthinClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl ScanClient {
                pub fn lvmthin(&self) -> lvmthin::LvmthinClient {
                    lvmthin::LvmthinClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod zfs {
                pub struct ZfsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ZfsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/zfs"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub pool: String,
                }
                impl ZfsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl ScanClient {
                pub fn zfs(&self) -> zfs::ZfsClient {
                    zfs::ZfsClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn scan(&self) -> scan::ScanClient {
                scan::ScanClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod hardware {
            pub struct HardwareClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl HardwareClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/hardware"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(rename = "type")]
                pub ty: String,
            }
            impl HardwareClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod pci {
                pub struct PciClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl PciClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/pci"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub verbose: Option<bool>,
                    #[serde(rename = "pci-class-blacklist")]
                    pub pci_class_blacklist: Option<String>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subsystem_device: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subsystem_device_name: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subsystem_vendor_name: Option<String>,
                    pub vendor: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub vendor_name: Option<String>,
                    pub class: String,
                    pub device: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subsystem_vendor: Option<String>,
                    pub id: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub mdev: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub device_name: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub iommugroup: u64,
                }
                impl PciClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
                pub mod pciid {
                    pub struct PciidClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PciidClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            pciid: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, pciid),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub method: String,
                    }
                    impl PciidClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    pub mod mdev {
                        pub struct MdevClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl MdevClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/mdev"),
                                }
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub description: String,
                            #[serde(rename = "type")]
                            pub ty: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub available: u64,
                        }
                        impl MdevClient {
                            pub fn get(
                                &self,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                    }
                    impl PciidClient {
                        pub fn mdev(&self) -> mdev::MdevClient {
                            mdev::MdevClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl PciClient {
                    pub fn pciid(&self, pciid: &str) -> pciid::PciidClient {
                        pciid::PciidClient::new(self.client.clone(), &self.path, pciid)
                    }
                }
            }
            impl HardwareClient {
                pub fn pci(&self) -> pci::PciClient {
                    pci::PciClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod usb {
                pub struct UsbClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl UsbClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/usb"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub prodid: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub product: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub devnum: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub serial: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub port: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub class: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub usbpath: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub busnum: u64,
                    pub vendid: String,
                    pub speed: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub level: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub manufacturer: Option<String>,
                }
                impl UsbClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl HardwareClient {
                pub fn usb(&self) -> usb::UsbClient {
                    usb::UsbClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn hardware(&self) -> hardware::HardwareClient {
                hardware::HardwareClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod capabilities {
            pub struct CapabilitiesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl CapabilitiesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/capabilities"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl CapabilitiesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod qemu {
                pub struct QemuClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl QemuClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/qemu"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl QemuClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod cpu {
                    pub struct CpuClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CpuClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/cpu"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub name: String,
                        pub vendor: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool",
                            deserialize_with = "::proxmox_api::deserialize_bool"
                        )]
                        pub custom: bool,
                    }
                    impl CpuClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl QemuClient {
                    pub fn cpu(&self) -> cpu::CpuClient {
                        cpu::CpuClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod machines {
                    pub struct MachinesClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl MachinesClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/machines"),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub id: String,
                        pub version: String,
                        #[serde(rename = "type")]
                        pub ty: String,
                    }
                    impl MachinesClient {
                        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl QemuClient {
                    pub fn machines(&self) -> machines::MachinesClient {
                        machines::MachinesClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl CapabilitiesClient {
                pub fn qemu(&self) -> qemu::QemuClient {
                    qemu::QemuClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn capabilities(&self) -> capabilities::CapabilitiesClient {
                capabilities::CapabilitiesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod storage {
            pub struct StorageClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StorageClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/storage"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub content: Option<String>,
                pub enabled: Option<bool>,
                pub format: Option<bool>,
                pub storage: Option<String>,
                pub target: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub total: Option<u64>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub active: Option<bool>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub enabled: Option<bool>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub used: Option<u64>,
                pub content: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub avail: Option<u64>,
                #[serde(rename = "type")]
                pub ty: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub shared: Option<bool>,
                pub storage: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_number_optional",
                    deserialize_with = "::proxmox_api::deserialize_number_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub used_fraction: Option<f64>,
            }
            impl StorageClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            pub mod storage {
                pub struct StorageClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl StorageClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        storage: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, storage),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub subdir: String,
                }
                impl StorageClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod prunebackups {
                    pub struct PrunebackupsClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PrunebackupsClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/prunebackups"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        #[serde(rename = "prune-backups")]
                        pub prune_backups: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: Option<String>,
                        pub vmid: Option<u64>,
                    }
                    impl PrunebackupsClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub vmid: Option<u64>,
                        #[serde(rename = "prune-backups")]
                        pub prune_backups: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: Option<String>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub ctime: u64,
                        pub mark: String,
                        #[serde(rename = "type")]
                        pub ty: String,
                        pub volid: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vmid: Option<u64>,
                    }
                    impl PrunebackupsClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl StorageClient {
                    pub fn prunebackups(&self) -> prunebackups::PrunebackupsClient {
                        prunebackups::PrunebackupsClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod content {
                    pub struct ContentClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ContentClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/content"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub format: Option<String>,
                        pub size: String,
                        pub vmid: u64,
                        pub filename: String,
                    }
                    impl ContentClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub content: Option<String>,
                        pub vmid: Option<u64>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItemsVerification {
                        pub state: String,
                        pub upid: String,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub parent: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub size: u64,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub notes: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ctime: Option<u64>,
                        pub format: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub protected: Option<bool>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub used: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub verification: Option<GETReturnsItemsVerification>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub vmid: Option<u64>,
                        pub volid: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub encrypted: Option<String>,
                    }
                    impl ContentClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                    pub mod volume {
                        pub struct VolumeClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl VolumeClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                                volume: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}/{}", parent_path, volume),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct DELETEParams {
                            pub delay: Option<u64>,
                        }
                        impl VolumeClient {
                            pub fn delete(
                                &self,
                                params: DELETEParams,
                            ) -> Result<Option<String>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.delete(&path, &params)
                            }
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturns {
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub used: u64,
                            pub format: String,
                            pub path: String,
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub notes: Option<String>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool_optional",
                                deserialize_with = "::proxmox_api::deserialize_bool_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub protected: Option<bool>,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int",
                                deserialize_with = "::proxmox_api::deserialize_int"
                            )]
                            pub size: u64,
                        }
                        impl VolumeClient {
                            pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &())
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct POSTParams {
                            pub target_node: Option<String>,
                            pub target: String,
                        }
                        impl VolumeClient {
                            pub fn post(
                                &self,
                                params: POSTParams,
                            ) -> Result<String, ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.post(&path, &params)
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct PUTParams {
                            pub protected: Option<bool>,
                            pub notes: Option<String>,
                        }
                        impl VolumeClient {
                            pub fn put(
                                &self,
                                params: PUTParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.put(&path, &params)
                            }
                        }
                    }
                    impl ContentClient {
                        pub fn volume(&self, volume: &str) -> volume::VolumeClient {
                            volume::VolumeClient::new(self.client.clone(), &self.path, volume)
                        }
                    }
                }
                impl StorageClient {
                    pub fn content(&self) -> content::ContentClient {
                        content::ContentClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod file_restore {
                    pub struct FileRestoreClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl FileRestoreClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/file-restore"),
                            }
                        }
                    }
                    pub mod list {
                        pub struct ListClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl ListClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/list"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub filepath: String,
                            pub volume: String,
                        }
                        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                        pub struct GETReturnsItems {
                            pub filepath: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_bool",
                                deserialize_with = "::proxmox_api::deserialize_bool"
                            )]
                            pub leaf: bool,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub size: Option<u64>,
                            pub text: String,
                            #[serde(
                                serialize_with = "::proxmox_api::serialize_int_optional",
                                deserialize_with = "::proxmox_api::deserialize_int_optional"
                            )]
                            #[serde(skip_serializing_if = "Option::is_none", default)]
                            pub mtime: Option<u64>,
                            #[serde(rename = "type")]
                            pub ty: String,
                        }
                        impl ListClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                            {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FileRestoreClient {
                        pub fn list(&self) -> list::ListClient {
                            list::ListClient::new(self.client.clone(), &self.path)
                        }
                    }
                    pub mod download {
                        pub struct DownloadClient {
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            path: String,
                        }
                        impl DownloadClient {
                            pub fn new(
                                client: ::std::sync::Arc<::proxmox_api::Client>,
                                parent_path: &str,
                            ) -> Self {
                                Self {
                                    client,
                                    path: format!("{}{}", parent_path, "/download"),
                                }
                            }
                        }
                        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                        pub struct GETParams {
                            pub volume: String,
                            pub filepath: String,
                        }
                        impl DownloadClient {
                            pub fn get(
                                &self,
                                params: GETParams,
                            ) -> Result<(), ::proxmox_api::Error> {
                                let path = self.path.to_string();
                                self.client.get(&path, &params)
                            }
                        }
                    }
                    impl FileRestoreClient {
                        pub fn download(&self) -> download::DownloadClient {
                            download::DownloadClient::new(self.client.clone(), &self.path)
                        }
                    }
                }
                impl StorageClient {
                    pub fn file_restore(&self) -> file_restore::FileRestoreClient {
                        file_restore::FileRestoreClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod status {
                    pub struct StatusClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StatusClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/status"),
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl StorageClient {
                    pub fn status(&self) -> status::StatusClient {
                        status::StatusClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrd {
                    pub struct RrdClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrdClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrd"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub timeframe: String,
                        pub cf: Option<String>,
                        pub ds: String,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub filename: String,
                    }
                    impl RrdClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl StorageClient {
                    pub fn rrd(&self) -> rrd::RrdClient {
                        rrd::RrdClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod rrddata {
                    pub struct RrddataClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl RrddataClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/rrddata"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub cf: Option<String>,
                        pub timeframe: String,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturnsItems {}
                    impl RrddataClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl StorageClient {
                    pub fn rrddata(&self) -> rrddata::RrddataClient {
                        rrddata::RrddataClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod upload {
                    pub struct UploadClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl UploadClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/upload"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        #[serde(rename = "checksum-algorithm")]
                        pub checksum_algorithm: Option<String>,
                        pub tmpfilename: Option<String>,
                        pub filename: String,
                        pub content: String,
                        pub checksum: Option<String>,
                    }
                    impl UploadClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl StorageClient {
                    pub fn upload(&self) -> upload::UploadClient {
                        upload::UploadClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod download_url {
                    pub struct DownloadUrlClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl DownloadUrlClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/download-url"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub url: String,
                        #[serde(rename = "checksum-algorithm")]
                        pub checksum_algorithm: Option<String>,
                        pub checksum: Option<String>,
                        #[serde(rename = "verify-certificates")]
                        pub verify_certificates: Option<bool>,
                        pub content: String,
                        pub filename: String,
                    }
                    impl DownloadUrlClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl StorageClient {
                    pub fn download_url(&self) -> download_url::DownloadUrlClient {
                        download_url::DownloadUrlClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl StorageClient {
                pub fn storage(&self, storage: &str) -> storage::StorageClient {
                    storage::StorageClient::new(self.client.clone(), &self.path, storage)
                }
            }
        }
        impl NodeClient {
            pub fn storage(&self) -> storage::StorageClient {
                storage::StorageClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod disks {
            pub struct DisksClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl DisksClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/disks"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl DisksClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod lvm {
                pub struct LvmClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LvmClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/lvm"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub name: String,
                    pub add_storage: Option<bool>,
                    pub device: String,
                }
                impl LvmClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsChildrenItemsChildrenItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub free: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub leaf: bool,
                    pub name: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsChildrenItems {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub children: Option<Vec<Option<GETReturnsChildrenItemsChildrenItems>>>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub free: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub leaf: bool,
                    pub name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    pub children: Vec<GETReturnsChildrenItems>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub leaf: bool,
                }
                impl LvmClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        #[serde(rename = "cleanup-config")]
                        pub cleanup_config: Option<bool>,
                        #[serde(rename = "cleanup-disks")]
                        pub cleanup_disks: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                }
                impl LvmClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl DisksClient {
                pub fn lvm(&self) -> lvm::LvmClient {
                    lvm::LvmClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod lvmthin {
                pub struct LvmthinClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LvmthinClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/lvmthin"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub add_storage: Option<bool>,
                    pub name: String,
                    pub device: String,
                }
                impl LvmthinClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub metadata_used: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub used: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub metadata_size: u64,
                    pub vg: String,
                    pub lv: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub lv_size: u64,
                }
                impl LvmthinClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        #[serde(rename = "volume-group")]
                        pub volume_group: String,
                        #[serde(rename = "cleanup-config")]
                        pub cleanup_config: Option<bool>,
                        #[serde(rename = "cleanup-disks")]
                        pub cleanup_disks: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                }
                impl LvmthinClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl DisksClient {
                pub fn lvmthin(&self) -> lvmthin::LvmthinClient {
                    lvmthin::LvmthinClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod directory {
                pub struct DirectoryClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl DirectoryClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/directory"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub filesystem: Option<String>,
                    pub add_storage: Option<bool>,
                    pub name: String,
                    pub device: String,
                }
                impl DirectoryClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub options: String,
                    pub unitfile: String,
                    pub device: String,
                    pub path: String,
                    #[serde(rename = "type")]
                    pub ty: String,
                }
                impl DirectoryClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        #[serde(rename = "cleanup-disks")]
                        pub cleanup_disks: Option<bool>,
                        #[serde(rename = "cleanup-config")]
                        pub cleanup_config: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                }
                impl DirectoryClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl DisksClient {
                pub fn directory(&self) -> directory::DirectoryClient {
                    directory::DirectoryClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod zfs {
                pub struct ZfsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ZfsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/zfs"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub ashift: Option<u64>,
                    pub name: String,
                    pub compression: Option<String>,
                    pub devices: String,
                    pub add_storage: Option<bool>,
                    #[serde(rename = "draid-config")]
                    pub draid_config: Option<String>,
                    pub raidlevel: String,
                }
                impl ZfsClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub alloc: u64,
                    pub health: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub frag: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_number",
                        deserialize_with = "::proxmox_api::deserialize_number"
                    )]
                    pub dedup: f64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub free: u64,
                }
                impl ZfsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod name {
                    pub struct NameClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl NameClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            name: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, name),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsChildrenItems {
                        pub msg: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub read: Option<f64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub cksum: Option<f64>,
                        pub name: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub state: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_number_optional",
                            deserialize_with = "::proxmox_api::deserialize_number_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub write: Option<f64>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        pub children: Vec<GETReturnsChildrenItems>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub action: Option<String>,
                        pub errors: String,
                        pub name: String,
                        pub state: String,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub status: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub scan: Option<String>,
                    }
                    impl NameClient {
                        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        #[serde(rename = "cleanup-config")]
                        pub cleanup_config: Option<bool>,
                        #[serde(rename = "cleanup-disks")]
                        pub cleanup_disks: Option<bool>,
                    }
                    impl NameClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                }
                impl ZfsClient {
                    pub fn name(&self, name: &str) -> name::NameClient {
                        name::NameClient::new(self.client.clone(), &self.path, name)
                    }
                }
            }
            impl DisksClient {
                pub fn zfs(&self) -> zfs::ZfsClient {
                    zfs::ZfsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod list {
                pub struct ListClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ListClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/list"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    #[serde(rename = "include-partitions")]
                    pub include_partitions: Option<bool>,
                    pub skipsmart: Option<bool>,
                    #[serde(rename = "type")]
                    pub ty: Option<String>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub gpt: bool,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub model: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub size: u64,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub health: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub serial: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub osdid: u64,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub mounted: bool,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub parent: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub used: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub vendor: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub wwn: Option<String>,
                    pub devpath: String,
                }
                impl ListClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl DisksClient {
                pub fn list(&self) -> list::ListClient {
                    list::ListClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod smart {
                pub struct SmartClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl SmartClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/smart"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub disk: String,
                    pub healthonly: Option<bool>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub attributes: Option<()>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub text: Option<String>,
                    pub health: String,
                    #[serde(rename = "type")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ty: Option<String>,
                }
                impl SmartClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl DisksClient {
                pub fn smart(&self) -> smart::SmartClient {
                    smart::SmartClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod initgpt {
                pub struct InitgptClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl InitgptClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/initgpt"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub disk: String,
                    pub uuid: Option<String>,
                }
                impl InitgptClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl DisksClient {
                pub fn initgpt(&self) -> initgpt::InitgptClient {
                    initgpt::InitgptClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod wipedisk {
                pub struct WipediskClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl WipediskClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/wipedisk"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub disk: String,
                }
                impl WipediskClient {
                    pub fn put(&self, params: PUTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
            }
            impl DisksClient {
                pub fn wipedisk(&self) -> wipedisk::WipediskClient {
                    wipedisk::WipediskClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn disks(&self) -> disks::DisksClient {
                disks::DisksClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod apt {
            pub struct AptClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl AptClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/apt"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub id: String,
            }
            impl AptClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod update {
                pub struct UpdateClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl UpdateClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/update"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub notify: Option<bool>,
                    pub quiet: Option<bool>,
                }
                impl UpdateClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl UpdateClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl AptClient {
                pub fn update(&self) -> update::UpdateClient {
                    update::UpdateClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod changelog {
                pub struct ChangelogClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl ChangelogClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/changelog"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub name: String,
                    pub version: Option<String>,
                }
                impl ChangelogClient {
                    pub fn get(&self, params: GETParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl AptClient {
                pub fn changelog(&self) -> changelog::ChangelogClient {
                    changelog::ChangelogClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod repositories {
                pub struct RepositoriesClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl RepositoriesClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/repositories"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsErrorsItems {
                    pub path: String,
                    pub error: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsInfosItems {
                    pub index: String,
                    pub kind: String,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub property: Option<String>,
                    pub path: String,
                    pub message: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsStandardReposItems {
                    pub name: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub status: Option<bool>,
                    pub handle: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsFilesItemsRepositoriesItemsOptionsItems {
                    #[serde(rename = "Key")]
                    pub key: String,
                    #[serde(rename = "Values")]
                    pub values: Vec<String>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsFilesItemsRepositoriesItems {
                    #[serde(rename = "Options")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub options:
                        Option<Vec<Option<GETReturnsFilesItemsRepositoriesItemsOptionsItems>>>,
                    #[serde(rename = "Enabled")]
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool",
                        deserialize_with = "::proxmox_api::deserialize_bool"
                    )]
                    pub enabled: bool,
                    #[serde(rename = "Suites")]
                    pub suites: Vec<String>,
                    #[serde(rename = "Comment")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                    #[serde(rename = "FileType")]
                    pub filetype: String,
                    #[serde(rename = "Types")]
                    pub types: Vec<String>,
                    #[serde(rename = "URIs")]
                    pub uris: Vec<String>,
                    #[serde(rename = "Components")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub components: Option<Vec<Option<String>>>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsFilesItems {
                    pub digest: Vec<u64>,
                    pub repositories: Vec<GETReturnsFilesItemsRepositoriesItems>,
                    #[serde(rename = "file-type")]
                    pub file_type: String,
                    pub path: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    pub digest: String,
                    pub errors: Vec<GETReturnsErrorsItems>,
                    pub infos: Vec<GETReturnsInfosItems>,
                    #[serde(rename = "standard-repos")]
                    pub standard_repos: Vec<GETReturnsStandardReposItems>,
                    pub files: Vec<GETReturnsFilesItems>,
                }
                impl RepositoriesClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub enabled: Option<bool>,
                    pub index: u64,
                    pub path: String,
                    pub digest: Option<String>,
                }
                impl RepositoriesClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub handle: String,
                    pub digest: Option<String>,
                }
                impl RepositoriesClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
            }
            impl AptClient {
                pub fn repositories(&self) -> repositories::RepositoriesClient {
                    repositories::RepositoriesClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod versions {
                pub struct VersionsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl VersionsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/versions"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl VersionsClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl AptClient {
                pub fn versions(&self) -> versions::VersionsClient {
                    versions::VersionsClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn apt(&self) -> apt::AptClient {
                apt::AptClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod firewall {
            pub struct FirewallClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl FirewallClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/firewall"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl FirewallClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod rules {
                pub struct RulesClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl RulesClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/rules"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub sport: Option<String>,
                    pub proto: Option<String>,
                    pub action: String,
                    #[serde(rename = "macro")]
                    pub macro_def: Option<String>,
                    pub iface: Option<String>,
                    pub source: Option<String>,
                    pub enable: Option<u64>,
                    #[serde(rename = "type")]
                    pub ty: String,
                    pub comment: Option<String>,
                    pub pos: Option<u64>,
                    pub digest: Option<String>,
                    pub log: Option<String>,
                    pub dest: Option<String>,
                    #[serde(rename = "icmp-type")]
                    pub icmp_type: Option<String>,
                    pub dport: Option<String>,
                }
                impl RulesClient {
                    pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub pos: u64,
                }
                impl RulesClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod pos {
                    pub struct PosClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl PosClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            pos: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, pos),
                            }
                        }
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturns {
                        #[serde(rename = "macro")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub macro_def: Option<String>,
                        pub action: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub ipversion: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub dport: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub iface: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub source: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub comment: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: String,
                        #[serde(rename = "icmp-type")]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub icmp_type: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub enable: Option<u64>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub dest: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub log: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub pos: u64,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub proto: Option<String>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub sport: Option<String>,
                    }
                    impl PosClient {
                        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct DELETEParams {
                        pub digest: Option<String>,
                    }
                    impl PosClient {
                        pub fn delete(
                            &self,
                            params: DELETEParams,
                        ) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub dport: Option<String>,
                        #[serde(rename = "type")]
                        pub ty: Option<String>,
                        #[serde(rename = "icmp-type")]
                        pub icmp_type: Option<String>,
                        pub enable: Option<u64>,
                        pub source: Option<String>,
                        pub delete: Option<String>,
                        pub log: Option<String>,
                        #[serde(rename = "macro")]
                        pub macro_def: Option<String>,
                        pub comment: Option<String>,
                        pub iface: Option<String>,
                        pub digest: Option<String>,
                        pub proto: Option<String>,
                        pub moveto: Option<u64>,
                        pub action: Option<String>,
                        pub sport: Option<String>,
                        pub dest: Option<String>,
                    }
                    impl PosClient {
                        pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                }
                impl RulesClient {
                    pub fn pos(&self, pos: &str) -> pos::PosClient {
                        pos::PosClient::new(self.client.clone(), &self.path, pos)
                    }
                }
            }
            impl FirewallClient {
                pub fn rules(&self) -> rules::RulesClient {
                    rules::RulesClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod options {
                pub struct OptionsClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl OptionsClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/options"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturns {
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nf_conntrack_tcp_timeout_syn_recv: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub smurf_log_level: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub tcp_flags_log_level: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub tcpflags: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nf_conntrack_max: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nosmurfs: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub protection_synflood_rate: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub log_nf_conntrack: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub protection_synflood: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub enable: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub log_level_in: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub protection_synflood_burst: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nf_conntrack_allow_invalid: Option<bool>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub log_level_out: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nf_conntrack_helpers: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub ndp: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub nf_conntrack_tcp_timeout_established: Option<u64>,
                }
                impl OptionsClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub digest: Option<String>,
                    pub nf_conntrack_allow_invalid: Option<bool>,
                    pub nf_conntrack_max: Option<u64>,
                    pub protection_synflood: Option<bool>,
                    pub log_level_out: Option<String>,
                    pub log_level_in: Option<String>,
                    pub ndp: Option<bool>,
                    pub enable: Option<bool>,
                    pub tcpflags: Option<bool>,
                    pub protection_synflood_burst: Option<u64>,
                    pub nosmurfs: Option<bool>,
                    pub nf_conntrack_tcp_timeout_established: Option<u64>,
                    pub protection_synflood_rate: Option<u64>,
                    pub smurf_log_level: Option<String>,
                    pub nf_conntrack_tcp_timeout_syn_recv: Option<u64>,
                    pub nf_conntrack_helpers: Option<String>,
                    pub log_nf_conntrack: Option<bool>,
                    pub tcp_flags_log_level: Option<String>,
                    pub delete: Option<String>,
                }
                impl OptionsClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
            }
            impl FirewallClient {
                pub fn options(&self) -> options::OptionsClient {
                    options::OptionsClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod log {
                pub struct LogClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl LogClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/log"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub until: Option<u64>,
                    pub limit: Option<u64>,
                    pub since: Option<u64>,
                    pub start: Option<u64>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub t: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub n: u64,
                }
                impl LogClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl FirewallClient {
                pub fn log(&self) -> log::LogClient {
                    log::LogClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn firewall(&self) -> firewall::FirewallClient {
                firewall::FirewallClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod replication {
            pub struct ReplicationClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ReplicationClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/replication"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub guest: Option<u64>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub id: String,
            }
            impl ReplicationClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            pub mod id {
                pub struct IdClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IdClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        id: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, id),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl IdClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod status {
                    pub struct StatusClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl StatusClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/status"),
                            }
                        }
                    }
                    impl StatusClient {
                        pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl IdClient {
                    pub fn status(&self) -> status::StatusClient {
                        status::StatusClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod log {
                    pub struct LogClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl LogClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/log"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct GETParams {
                        pub start: Option<u64>,
                        pub limit: Option<u64>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct GETReturnsItems {
                        pub t: String,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int",
                            deserialize_with = "::proxmox_api::deserialize_int"
                        )]
                        pub n: u64,
                    }
                    impl LogClient {
                        pub fn get(
                            &self,
                            params: GETParams,
                        ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error>
                        {
                            let path = self.path.to_string();
                            self.client.get(&path, &params)
                        }
                    }
                }
                impl IdClient {
                    pub fn log(&self) -> log::LogClient {
                        log::LogClient::new(self.client.clone(), &self.path)
                    }
                }
                pub mod schedule_now {
                    pub struct ScheduleNowClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl ScheduleNowClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/schedule_now"),
                            }
                        }
                    }
                    impl ScheduleNowClient {
                        pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &())
                        }
                    }
                }
                impl IdClient {
                    pub fn schedule_now(&self) -> schedule_now::ScheduleNowClient {
                        schedule_now::ScheduleNowClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl ReplicationClient {
                pub fn id(&self, id: &str) -> id::IdClient {
                    id::IdClient::new(self.client.clone(), &self.path, id)
                }
            }
        }
        impl NodeClient {
            pub fn replication(&self) -> replication::ReplicationClient {
                replication::ReplicationClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod certificates {
            pub struct CertificatesClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl CertificatesClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/certificates"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl CertificatesClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod acme {
                pub struct AcmeClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl AcmeClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/acme"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {}
                impl AcmeClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod certificate {
                    pub struct CertificateClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl CertificateClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}{}", parent_path, "/certificate"),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub force: Option<bool>,
                    }
                    impl CertificateClient {
                        pub fn put(
                            &self,
                            params: PUTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    impl CertificateClient {
                        pub fn delete(&self) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &())
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub force: Option<bool>,
                    }
                    impl CertificateClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<String, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                }
                impl AcmeClient {
                    pub fn certificate(&self) -> certificate::CertificateClient {
                        certificate::CertificateClient::new(self.client.clone(), &self.path)
                    }
                }
            }
            impl CertificatesClient {
                pub fn acme(&self) -> acme::AcmeClient {
                    acme::AcmeClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod info {
                pub struct InfoClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl InfoClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/info"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturnsItems {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub filename: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub issuer: Option<String>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub notafter: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub notbefore: Option<u64>,
                    #[serde(rename = "public-key-type")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub public_key_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub fingerprint: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pem: Option<String>,
                    #[serde(rename = "public-key-bits")]
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub public_key_bits: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub san: Option<Vec<Option<String>>>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subject: Option<String>,
                }
                impl InfoClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
            }
            impl CertificatesClient {
                pub fn info(&self) -> info::InfoClient {
                    info::InfoClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod custom {
                pub struct CustomClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl CustomClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/custom"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    pub restart: Option<bool>,
                }
                impl CustomClient {
                    pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub restart: Option<bool>,
                    pub key: Option<String>,
                    pub force: Option<bool>,
                    pub certificates: String,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct POSTReturns {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub fingerprint: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub filename: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub san: Option<Vec<Option<String>>>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub notafter: Option<u64>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub notbefore: Option<u64>,
                    #[serde(rename = "public-key-bits")]
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub public_key_bits: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub issuer: Option<String>,
                    #[serde(rename = "public-key-type")]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub public_key_type: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub pem: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub subject: Option<String>,
                }
                impl CustomClient {
                    pub fn post(
                        &self,
                        params: POSTParams,
                    ) -> Result<POSTReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl CertificatesClient {
                pub fn custom(&self) -> custom::CustomClient {
                    custom::CustomClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl NodeClient {
            pub fn certificates(&self) -> certificates::CertificatesClient {
                certificates::CertificatesClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod config {
            pub struct ConfigClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ConfigClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/config"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub property: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(rename = "acmedomain[n]")]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub acmedomain_n: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub digest: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub description: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub acme: Option<String>,
                #[serde(rename = "startall-onboot-delay")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub startall_onboot_delay: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub wakeonlan: Option<String>,
            }
            impl ConfigClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub wakeonlan: Option<String>,
                pub description: Option<String>,
                pub delete: Option<String>,
                #[serde(rename = "acmedomain[n]")]
                pub acmedomain_n: Option<String>,
                #[serde(rename = "startall-onboot-delay")]
                pub startall_onboot_delay: Option<u64>,
                pub acme: Option<String>,
                pub digest: Option<String>,
            }
            impl ConfigClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn config(&self) -> config::ConfigClient {
                config::ConfigClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod version {
            pub struct VersionClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl VersionClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/version"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub version: String,
                pub release: String,
                pub repoid: String,
            }
            impl VersionClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn version(&self) -> version::VersionClient {
                version::VersionClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod status {
            pub struct StatusClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StatusClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/status"),
                    }
                }
            }
            impl StatusClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub command: String,
            }
            impl StatusClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn status(&self) -> status::StatusClient {
                status::StatusClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod netstat {
            pub struct NetstatClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl NetstatClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/netstat"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl NetstatClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn netstat(&self) -> netstat::NetstatClient {
                netstat::NetstatClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod execute {
            pub struct ExecuteClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ExecuteClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/execute"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub commands: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct POSTReturnsItems {}
            impl ExecuteClient {
                pub fn post(
                    &self,
                    params: POSTParams,
                ) -> Result<Vec<POSTReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn execute(&self) -> execute::ExecuteClient {
                execute::ExecuteClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod wakeonlan {
            pub struct WakeonlanClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl WakeonlanClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/wakeonlan"),
                    }
                }
            }
            impl WakeonlanClient {
                pub fn post(&self) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn wakeonlan(&self) -> wakeonlan::WakeonlanClient {
                wakeonlan::WakeonlanClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod rrd {
            pub struct RrdClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RrdClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/rrd"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub cf: Option<String>,
                pub ds: String,
                pub timeframe: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub filename: String,
            }
            impl RrdClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn rrd(&self) -> rrd::RrdClient {
                rrd::RrdClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod rrddata {
            pub struct RrddataClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RrddataClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/rrddata"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub timeframe: String,
                pub cf: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl RrddataClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn rrddata(&self) -> rrddata::RrddataClient {
                rrddata::RrddataClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod syslog {
            pub struct SyslogClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl SyslogClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/syslog"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub since: Option<String>,
                pub start: Option<u64>,
                pub service: Option<String>,
                pub limit: Option<u64>,
                pub until: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                pub t: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub n: u64,
            }
            impl SyslogClient {
                pub fn get(
                    &self,
                    params: GETParams,
                ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn syslog(&self) -> syslog::SyslogClient {
                syslog::SyslogClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod journal {
            pub struct JournalClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl JournalClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/journal"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub endcursor: Option<String>,
                pub startcursor: Option<String>,
                pub lastentries: Option<u64>,
                pub since: Option<u64>,
                pub until: Option<u64>,
            }
            impl JournalClient {
                pub fn get(&self, params: GETParams) -> Result<Vec<String>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn journal(&self) -> journal::JournalClient {
                journal::JournalClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod vncshell {
            pub struct VncshellClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl VncshellClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/vncshell"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub cmd: Option<String>,
                pub websocket: Option<bool>,
                pub width: Option<u64>,
                pub height: Option<u64>,
                #[serde(rename = "cmd-opts")]
                pub cmd_opts: Option<String>,
            }
            impl VncshellClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn vncshell(&self) -> vncshell::VncshellClient {
                vncshell::VncshellClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod termproxy {
            pub struct TermproxyClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl TermproxyClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/termproxy"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub cmd: Option<String>,
                #[serde(rename = "cmd-opts")]
                pub cmd_opts: Option<String>,
            }
            impl TermproxyClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn termproxy(&self) -> termproxy::TermproxyClient {
                termproxy::TermproxyClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod vncwebsocket {
            pub struct VncwebsocketClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl VncwebsocketClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/vncwebsocket"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub port: u64,
                pub vncticket: String,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub port: String,
            }
            impl VncwebsocketClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient {
                vncwebsocket::VncwebsocketClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod spiceshell {
            pub struct SpiceshellClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl SpiceshellClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/spiceshell"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub cmd: Option<String>,
                #[serde(rename = "cmd-opts")]
                pub cmd_opts: Option<String>,
                pub proxy: Option<String>,
            }
            impl SpiceshellClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn spiceshell(&self) -> spiceshell::SpiceshellClient {
                spiceshell::SpiceshellClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod dns {
            pub struct DnsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl DnsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/dns"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub dns1: Option<String>,
                pub dns3: Option<String>,
                pub search: String,
                pub dns2: Option<String>,
            }
            impl DnsClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub dns2: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub dns3: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub search: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub dns1: Option<String>,
            }
            impl DnsClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn dns(&self) -> dns::DnsClient {
                dns::DnsClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod time {
            pub struct TimeClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl TimeClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/time"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub timezone: String,
            }
            impl TimeClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub time: u64,
                pub timezone: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub localtime: u64,
            }
            impl TimeClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn time(&self) -> time::TimeClient {
                time::TimeClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod aplinfo {
            pub struct AplinfoClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl AplinfoClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/aplinfo"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsItems {}
            impl AplinfoClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub template: String,
                pub storage: String,
            }
            impl AplinfoClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn aplinfo(&self) -> aplinfo::AplinfoClient {
                aplinfo::AplinfoClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod query_url_metadata {
            pub struct QueryUrlMetadataClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl QueryUrlMetadataClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/query-url-metadata"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct GETParams {
                pub url: String,
                #[serde(rename = "verify-certificates")]
                pub verify_certificates: Option<bool>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub mimetype: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub size: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub filename: Option<String>,
            }
            impl QueryUrlMetadataClient {
                pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn query_url_metadata(&self) -> query_url_metadata::QueryUrlMetadataClient {
                query_url_metadata::QueryUrlMetadataClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod report {
            pub struct ReportClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl ReportClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/report"),
                    }
                }
            }
            impl ReportClient {
                pub fn get(&self) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl NodeClient {
            pub fn report(&self) -> report::ReportClient {
                report::ReportClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod startall {
            pub struct StartallClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StartallClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/startall"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub vms: Option<String>,
                pub force: Option<bool>,
            }
            impl StartallClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn startall(&self) -> startall::StartallClient {
                startall::StartallClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod stopall {
            pub struct StopallClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl StopallClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/stopall"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub timeout: Option<u64>,
                pub vms: Option<String>,
                #[serde(rename = "force-stop")]
                pub force_stop: Option<bool>,
            }
            impl StopallClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn stopall(&self) -> stopall::StopallClient {
                stopall::StopallClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod migrateall {
            pub struct MigrateallClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl MigrateallClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/migrateall"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub vms: Option<String>,
                pub target: String,
                #[serde(rename = "with-local-disks")]
                pub with_local_disks: Option<bool>,
                pub maxworkers: Option<u64>,
            }
            impl MigrateallClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn migrateall(&self) -> migrateall::MigrateallClient {
                migrateall::MigrateallClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod hosts {
            pub struct HostsClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl HostsClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/hosts"),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub digest: Option<String>,
                pub data: String,
            }
            impl HostsClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub data: String,
                pub digest: Option<String>,
            }
            impl HostsClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl NodeClient {
            pub fn hosts(&self) -> hosts::HostsClient {
                hosts::HostsClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl NodesClient {
        pub fn node(&self, node: &str) -> node::NodeClient {
            node::NodeClient::new(self.client.clone(), &self.path, node)
        }
    }
}
pub mod storage {
    pub struct StorageClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl StorageClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/storage".to_string(),
            }
        }
    }
    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
    pub struct GETParams {
        #[serde(rename = "type")]
        pub ty: Option<String>,
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct GETReturnsItems {
        pub storage: String,
    }
    impl StorageClient {
        pub fn get(&self, params: GETParams) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &params)
        }
    }
    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
    pub struct POSTParams {
        pub domain: Option<String>,
        pub krbd: Option<bool>,
        pub server: Option<String>,
        pub nodes: Option<String>,
        pub is_mountpoint: Option<String>,
        #[serde(rename = "type")]
        pub ty: String,
        #[serde(rename = "content-dirs")]
        pub content_dirs: Option<String>,
        #[serde(rename = "data-pool")]
        pub data_pool: Option<String>,
        #[serde(rename = "prune-backups")]
        pub prune_backups: Option<String>,
        pub nocow: Option<bool>,
        pub tagged_only: Option<bool>,
        pub options: Option<String>,
        pub datastore: Option<String>,
        #[serde(rename = "master-pubkey")]
        pub master_pubkey: Option<String>,
        pub target: Option<String>,
        pub username: Option<String>,
        pub smbversion: Option<String>,
        pub saferemove: Option<bool>,
        pub export: Option<String>,
        pub maxfiles: Option<u64>,
        pub port: Option<u64>,
        pub storage: String,
        pub subdir: Option<String>,
        pub mkdir: Option<bool>,
        pub content: Option<String>,
        pub iscsiprovider: Option<String>,
        pub keyring: Option<String>,
        pub format: Option<String>,
        pub fuse: Option<bool>,
        pub server2: Option<String>,
        pub share: Option<String>,
        pub blocksize: Option<String>,
        pub bwlimit: Option<String>,
        pub nowritecache: Option<bool>,
        #[serde(rename = "fs-name")]
        pub fs_name: Option<String>,
        pub comstar_tg: Option<String>,
        pub portal: Option<String>,
        #[serde(rename = "encryption-key")]
        pub encryption_key: Option<String>,
        #[serde(rename = "max-protected-backups")]
        pub max_protected_backups: Option<()>,
        pub vgname: Option<String>,
        pub namespace: Option<String>,
        pub lio_tpg: Option<String>,
        pub volume: Option<String>,
        pub preallocation: Option<String>,
        pub comstar_hg: Option<String>,
        pub thinpool: Option<String>,
        pub base: Option<String>,
        pub monhost: Option<String>,
        pub authsupported: Option<String>,
        pub fingerprint: Option<String>,
        pub password: Option<String>,
        pub disable: Option<bool>,
        pub pool: Option<String>,
        pub saferemove_throughput: Option<String>,
        pub shared: Option<bool>,
        pub sparse: Option<bool>,
        pub mountpoint: Option<String>,
        pub path: Option<String>,
        pub transport: Option<String>,
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
    pub struct POSTReturnsConfig {
        #[serde(rename = "encryption-key")]
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub encryption_key: Option<String>,
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct POSTReturns {
        #[serde(rename = "type")]
        pub ty: String,
        pub storage: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub config: Option<POSTReturnsConfig>,
    }
    impl StorageClient {
        pub fn post(&self, params: POSTParams) -> Result<POSTReturns, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.post(&path, &params)
        }
    }
    pub mod storage {
        pub struct StorageClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl StorageClient {
            pub fn new(
                client: ::std::sync::Arc<::proxmox_api::Client>,
                parent_path: &str,
                storage: &str,
            ) -> Self {
                Self {
                    client,
                    path: format!("{}/{}", parent_path, storage),
                }
            }
        }
        impl StorageClient {
            pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.delete(&path, &())
            }
        }
        impl StorageClient {
            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct PUTParams {
            pub maxfiles: Option<u64>,
            #[serde(rename = "master-pubkey")]
            pub master_pubkey: Option<String>,
            #[serde(rename = "encryption-key")]
            pub encryption_key: Option<String>,
            pub server2: Option<String>,
            pub format: Option<String>,
            pub namespace: Option<String>,
            pub smbversion: Option<String>,
            pub is_mountpoint: Option<String>,
            pub subdir: Option<String>,
            pub blocksize: Option<String>,
            pub comstar_tg: Option<String>,
            pub digest: Option<String>,
            pub mkdir: Option<bool>,
            pub preallocation: Option<String>,
            pub comstar_hg: Option<String>,
            pub monhost: Option<String>,
            #[serde(rename = "prune-backups")]
            pub prune_backups: Option<String>,
            pub saferemove: Option<bool>,
            pub keyring: Option<String>,
            pub sparse: Option<bool>,
            #[serde(rename = "content-dirs")]
            pub content_dirs: Option<String>,
            pub tagged_only: Option<bool>,
            pub delete: Option<String>,
            pub bwlimit: Option<String>,
            pub krbd: Option<bool>,
            pub lio_tpg: Option<String>,
            pub username: Option<String>,
            pub nowritecache: Option<bool>,
            pub fingerprint: Option<String>,
            pub nodes: Option<String>,
            pub port: Option<u64>,
            pub content: Option<String>,
            pub disable: Option<bool>,
            pub options: Option<String>,
            pub mountpoint: Option<String>,
            #[serde(rename = "fs-name")]
            pub fs_name: Option<String>,
            pub pool: Option<String>,
            pub server: Option<String>,
            pub nocow: Option<bool>,
            #[serde(rename = "data-pool")]
            pub data_pool: Option<String>,
            #[serde(rename = "max-protected-backups")]
            pub max_protected_backups: Option<()>,
            pub password: Option<String>,
            pub shared: Option<bool>,
            pub saferemove_throughput: Option<String>,
            pub fuse: Option<bool>,
            pub transport: Option<String>,
            pub domain: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
        pub struct PUTReturnsConfig {
            #[serde(rename = "encryption-key")]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub encryption_key: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct PUTReturns {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub config: Option<PUTReturnsConfig>,
            pub storage: String,
            #[serde(rename = "type")]
            pub ty: String,
        }
        impl StorageClient {
            pub fn put(&self, params: PUTParams) -> Result<PUTReturns, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl StorageClient {
        pub fn storage(&self, storage: &str) -> storage::StorageClient {
            storage::StorageClient::new(self.client.clone(), &self.path, storage)
        }
    }
}
pub mod access {
    pub struct AccessClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl AccessClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/access".to_string(),
            }
        }
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct GETReturnsItems {
        pub subdir: String,
    }
    impl AccessClient {
        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
    pub mod users {
        pub struct UsersClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl UsersClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/users"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct GETParams {
            pub full: Option<bool>,
            pub enabled: Option<bool>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItemsTokensItems {
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub privsep: Option<bool>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub expire: Option<u64>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub comment: Option<String>,
            pub tokenid: String,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub comment: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub email: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub firstname: Option<String>,
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub enable: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub groups: Option<String>,
            #[serde(rename = "realm-type")]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub realm_type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub tokens: Option<Vec<Option<GETReturnsItemsTokensItems>>>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub lastname: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub keys: Option<String>,
            pub userid: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub expire: Option<u64>,
        }
        impl UsersClient {
            pub fn get(
                &self,
                params: GETParams,
            ) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &params)
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub groups: Option<String>,
            pub userid: String,
            pub expire: Option<u64>,
            pub comment: Option<String>,
            pub password: Option<String>,
            pub keys: Option<String>,
            pub lastname: Option<String>,
            pub enable: Option<bool>,
            pub firstname: Option<String>,
            pub email: Option<String>,
        }
        impl UsersClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        pub mod userid {
            pub struct UseridClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl UseridClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    userid: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, userid),
                    }
                }
            }
            impl UseridClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturnsTokens {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub privsep: Option<bool>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub expire: Option<u64>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub groups: Option<Vec<Option<String>>>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub email: Option<String>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub enable: Option<bool>,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int_optional",
                    deserialize_with = "::proxmox_api::deserialize_int_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub expire: Option<u64>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub keys: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub lastname: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub firstname: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub tokens: Option<GETReturnsTokens>,
            }
            impl UseridClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub enable: Option<bool>,
                pub firstname: Option<String>,
                pub keys: Option<String>,
                pub expire: Option<u64>,
                pub lastname: Option<String>,
                pub append: Option<bool>,
                pub comment: Option<String>,
                pub email: Option<String>,
                pub groups: Option<String>,
            }
            impl UseridClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            pub mod tfa {
                pub struct TfaClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl TfaClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/tfa"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct GETParams {
                    pub multiple: Option<bool>,
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
                pub struct GETReturns {
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub user: Option<String>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub types: Option<Vec<Option<String>>>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub realm: Option<String>,
                }
                impl TfaClient {
                    pub fn get(
                        &self,
                        params: GETParams,
                    ) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &params)
                    }
                }
            }
            impl UseridClient {
                pub fn tfa(&self) -> tfa::TfaClient {
                    tfa::TfaClient::new(self.client.clone(), &self.path)
                }
            }
            pub mod token {
                pub struct TokenClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl TokenClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/token"),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturnsItems {
                    pub tokenid: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub privsep: Option<bool>,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int_optional",
                        deserialize_with = "::proxmox_api::deserialize_int_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub expire: Option<u64>,
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub comment: Option<String>,
                }
                impl TokenClient {
                    pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                pub mod tokenid {
                    pub struct TokenidClient {
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        path: String,
                    }
                    impl TokenidClient {
                        pub fn new(
                            client: ::std::sync::Arc<::proxmox_api::Client>,
                            parent_path: &str,
                            tokenid: &str,
                        ) -> Self {
                            Self {
                                client,
                                path: format!("{}/{}", parent_path, tokenid),
                            }
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct PUTParams {
                        pub expire: Option<u64>,
                        pub comment: Option<String>,
                        pub privsep: Option<bool>,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct PUTReturns {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub expire: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub privsep: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub comment: Option<String>,
                    }
                    impl TokenidClient {
                        pub fn put(
                            &self,
                            params: PUTParams,
                        ) -> Result<PUTReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.put(&path, &params)
                        }
                    }
                    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                    pub struct POSTParams {
                        pub comment: Option<String>,
                        pub expire: Option<u64>,
                        pub privsep: Option<bool>,
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct POSTReturnsInfo {
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub expire: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub privsep: Option<bool>,
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub comment: Option<String>,
                    }
                    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                    pub struct POSTReturns {
                        pub info: POSTReturnsInfo,
                        pub value: String,
                        #[serde(rename = "full-tokenid")]
                        pub full_tokenid: String,
                    }
                    impl TokenidClient {
                        pub fn post(
                            &self,
                            params: POSTParams,
                        ) -> Result<POSTReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.post(&path, &params)
                        }
                    }
                    impl TokenidClient {
                        pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.delete(&path, &())
                        }
                    }
                    #[derive(
                        Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default,
                    )]
                    pub struct GETReturns {
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub comment: Option<String>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_int_optional",
                            deserialize_with = "::proxmox_api::deserialize_int_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub expire: Option<u64>,
                        #[serde(
                            serialize_with = "::proxmox_api::serialize_bool_optional",
                            deserialize_with = "::proxmox_api::deserialize_bool_optional"
                        )]
                        #[serde(skip_serializing_if = "Option::is_none", default)]
                        pub privsep: Option<bool>,
                    }
                    impl TokenidClient {
                        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                            let path = self.path.to_string();
                            self.client.get(&path, &())
                        }
                    }
                }
                impl TokenClient {
                    pub fn tokenid(&self, tokenid: &str) -> tokenid::TokenidClient {
                        tokenid::TokenidClient::new(self.client.clone(), &self.path, tokenid)
                    }
                }
            }
            impl UseridClient {
                pub fn token(&self) -> token::TokenClient {
                    token::TokenClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl UsersClient {
            pub fn userid(&self, userid: &str) -> userid::UseridClient {
                userid::UseridClient::new(self.client.clone(), &self.path, userid)
            }
        }
    }
    impl AccessClient {
        pub fn users(&self) -> users::UsersClient {
            users::UsersClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod groups {
        pub struct GroupsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl GroupsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/groups"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub comment: Option<String>,
            pub groupid: String,
        }
        impl GroupsClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub users: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub comment: Option<String>,
            pub groupid: String,
        }
        impl GroupsClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod groupid {
            pub struct GroupidClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl GroupidClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    groupid: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, groupid),
                    }
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturns {
                pub members: Vec<String>,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub comment: Option<String>,
            }
            impl GroupidClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub comment: Option<String>,
            }
            impl GroupidClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            impl GroupidClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
        }
        impl GroupsClient {
            pub fn groupid(&self, groupid: &str) -> groupid::GroupidClient {
                groupid::GroupidClient::new(self.client.clone(), &self.path, groupid)
            }
        }
    }
    impl AccessClient {
        pub fn groups(&self) -> groups::GroupsClient {
            groups::GroupsClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod roles {
        pub struct RolesClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl RolesClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/roles"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub privs: Option<String>,
            pub roleid: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub special: Option<bool>,
        }
        impl RolesClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub roleid: String,
            pub privs: Option<String>,
        }
        impl RolesClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        pub mod roleid {
            pub struct RoleidClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RoleidClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    roleid: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, roleid),
                    }
                }
            }
            impl RoleidClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub append: Option<bool>,
                pub privs: Option<String>,
            }
            impl RoleidClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
            pub struct GETReturns {
                #[serde(rename = "Sys.Modify")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_modify: Option<bool>,
                #[serde(rename = "VM.Clone")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_clone: Option<bool>,
                #[serde(rename = "VM.Console")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_console: Option<bool>,
                #[serde(rename = "Datastore.AllocateSpace")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub datastore_allocatespace: Option<bool>,
                #[serde(rename = "Pool.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub pool_allocate: Option<bool>,
                #[serde(rename = "Sys.PowerMgmt")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_powermgmt: Option<bool>,
                #[serde(rename = "Datastore.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub datastore_allocate: Option<bool>,
                #[serde(rename = "VM.Snapshot.Rollback")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_snapshot_rollback: Option<bool>,
                #[serde(rename = "VM.Monitor")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_monitor: Option<bool>,
                #[serde(rename = "VM.Config.CDROM")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_cdrom: Option<bool>,
                #[serde(rename = "Datastore.Audit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub datastore_audit: Option<bool>,
                #[serde(rename = "Permissions.Modify")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub permissions_modify: Option<bool>,
                #[serde(rename = "Sys.Audit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_audit: Option<bool>,
                #[serde(rename = "Datastore.AllocateTemplate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub datastore_allocatetemplate: Option<bool>,
                #[serde(rename = "Group.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub group_allocate: Option<bool>,
                #[serde(rename = "Sys.Console")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_console: Option<bool>,
                #[serde(rename = "Sys.Incoming")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_incoming: Option<bool>,
                #[serde(rename = "VM.Config.Network")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_network: Option<bool>,
                #[serde(rename = "VM.Config.Cloudinit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_cloudinit: Option<bool>,
                #[serde(rename = "Realm.AllocateUser")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub realm_allocateuser: Option<bool>,
                #[serde(rename = "User.Modify")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub user_modify: Option<bool>,
                #[serde(rename = "VM.Backup")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_backup: Option<bool>,
                #[serde(rename = "VM.Config.Options")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_options: Option<bool>,
                #[serde(rename = "VM.Config.CPU")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_cpu: Option<bool>,
                #[serde(rename = "VM.PowerMgmt")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_powermgmt: Option<bool>,
                #[serde(rename = "Realm.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub realm_allocate: Option<bool>,
                #[serde(rename = "Sys.Syslog")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sys_syslog: Option<bool>,
                #[serde(rename = "VM.Migrate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_migrate: Option<bool>,
                #[serde(rename = "SDN.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sdn_allocate: Option<bool>,
                #[serde(rename = "VM.Allocate")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_allocate: Option<bool>,
                #[serde(rename = "VM.Audit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_audit: Option<bool>,
                #[serde(rename = "SDN.Audit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub sdn_audit: Option<bool>,
                #[serde(rename = "Pool.Audit")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub pool_audit: Option<bool>,
                #[serde(rename = "VM.Config.Disk")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_disk: Option<bool>,
                #[serde(rename = "VM.Config.HWType")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_hwtype: Option<bool>,
                #[serde(rename = "VM.Config.Memory")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_config_memory: Option<bool>,
                #[serde(rename = "VM.Snapshot")]
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub vm_snapshot: Option<bool>,
            }
            impl RoleidClient {
                pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
        }
        impl RolesClient {
            pub fn roleid(&self, roleid: &str) -> roleid::RoleidClient {
                roleid::RoleidClient::new(self.client.clone(), &self.path, roleid)
            }
        }
    }
    impl AccessClient {
        pub fn roles(&self) -> roles::RolesClient {
            roles::RolesClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod acl {
        pub struct AclClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl AclClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/acl"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(rename = "type")]
            pub ty: String,
            pub ugid: String,
            pub path: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub propagate: Option<bool>,
            pub roleid: String,
        }
        impl AclClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct PUTParams {
            pub tokens: Option<String>,
            pub propagate: Option<bool>,
            pub groups: Option<String>,
            pub path: String,
            pub roles: String,
            pub users: Option<String>,
            pub delete: Option<bool>,
        }
        impl AclClient {
            pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl AccessClient {
        pub fn acl(&self) -> acl::AclClient {
            acl::AclClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod domains {
        pub struct DomainsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl DomainsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/domains"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub comment: Option<String>,
            #[serde(rename = "type")]
            pub ty: String,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub tfa: Option<String>,
            pub realm: String,
        }
        impl DomainsClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            #[serde(rename = "sync-defaults-options")]
            pub sync_defaults_options: Option<String>,
            pub verify: Option<bool>,
            pub bind_dn: Option<String>,
            pub capath: Option<String>,
            pub filter: Option<String>,
            pub realm: String,
            pub comment: Option<String>,
            pub scopes: Option<String>,
            pub user_attr: Option<String>,
            pub certkey: Option<String>,
            pub cert: Option<String>,
            pub domain: Option<String>,
            pub group_name_attr: Option<String>,
            pub port: Option<u64>,
            pub default: Option<bool>,
            pub group_classes: Option<String>,
            #[serde(rename = "username-claim")]
            pub username_claim: Option<String>,
            #[serde(rename = "issuer-url")]
            pub issuer_url: Option<String>,
            pub autocreate: Option<bool>,
            #[serde(rename = "acr-values")]
            pub acr_values: Option<String>,
            pub password: Option<String>,
            #[serde(rename = "type")]
            pub ty: String,
            pub prompt: Option<String>,
            #[serde(rename = "case-sensitive")]
            pub case_sensitive: Option<bool>,
            pub group_filter: Option<String>,
            pub user_classes: Option<String>,
            pub sync_attributes: Option<String>,
            pub tfa: Option<String>,
            pub group_dn: Option<String>,
            pub server2: Option<String>,
            pub mode: Option<String>,
            pub server1: Option<String>,
            #[serde(rename = "client-key")]
            pub client_key: Option<String>,
            pub secure: Option<bool>,
            pub sslversion: Option<String>,
            #[serde(rename = "client-id")]
            pub client_id: Option<String>,
            pub base_dn: Option<String>,
        }
        impl DomainsClient {
            pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        pub mod realm {
            pub struct RealmClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl RealmClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    realm: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, realm),
                    }
                }
            }
            impl RealmClient {
                pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            impl RealmClient {
                pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.delete(&path, &())
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct PUTParams {
                pub group_name_attr: Option<String>,
                #[serde(rename = "acr-values")]
                pub acr_values: Option<String>,
                #[serde(rename = "case-sensitive")]
                pub case_sensitive: Option<bool>,
                pub digest: Option<String>,
                pub server1: Option<String>,
                pub secure: Option<bool>,
                pub certkey: Option<String>,
                pub group_filter: Option<String>,
                pub autocreate: Option<bool>,
                pub cert: Option<String>,
                #[serde(rename = "client-id")]
                pub client_id: Option<String>,
                pub filter: Option<String>,
                pub mode: Option<String>,
                pub verify: Option<bool>,
                pub password: Option<String>,
                pub tfa: Option<String>,
                pub group_dn: Option<String>,
                pub scopes: Option<String>,
                pub user_attr: Option<String>,
                pub capath: Option<String>,
                pub prompt: Option<String>,
                pub user_classes: Option<String>,
                pub comment: Option<String>,
                #[serde(rename = "issuer-url")]
                pub issuer_url: Option<String>,
                pub sync_attributes: Option<String>,
                #[serde(rename = "sync-defaults-options")]
                pub sync_defaults_options: Option<String>,
                pub port: Option<u64>,
                pub base_dn: Option<String>,
                pub delete: Option<String>,
                pub domain: Option<String>,
                pub bind_dn: Option<String>,
                pub server2: Option<String>,
                pub sslversion: Option<String>,
                #[serde(rename = "client-key")]
                pub client_key: Option<String>,
                pub group_classes: Option<String>,
                pub default: Option<bool>,
            }
            impl RealmClient {
                pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.put(&path, &params)
                }
            }
            pub mod sync {
                pub struct SyncClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl SyncClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}{}", parent_path, "/sync"),
                        }
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct POSTParams {
                    pub full: Option<bool>,
                    pub purge: Option<bool>,
                    #[serde(rename = "remove-vanished")]
                    pub remove_vanished: Option<String>,
                    #[serde(rename = "dry-run")]
                    pub dry_run: Option<bool>,
                    pub scope: Option<String>,
                    #[serde(rename = "enable-new")]
                    pub enable_new: Option<bool>,
                }
                impl SyncClient {
                    pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.post(&path, &params)
                    }
                }
            }
            impl RealmClient {
                pub fn sync(&self) -> sync::SyncClient {
                    sync::SyncClient::new(self.client.clone(), &self.path)
                }
            }
        }
        impl DomainsClient {
            pub fn realm(&self, realm: &str) -> realm::RealmClient {
                realm::RealmClient::new(self.client.clone(), &self.path, realm)
            }
        }
    }
    impl AccessClient {
        pub fn domains(&self) -> domains::DomainsClient {
            domains::DomainsClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod openid {
        pub struct OpenidClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl OpenidClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/openid"),
                }
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub subdir: String,
        }
        impl OpenidClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod auth_url {
            pub struct AuthUrlClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl AuthUrlClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/auth-url"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                pub realm: String,
                #[serde(rename = "redirect-url")]
                pub redirect_url: String,
            }
            impl AuthUrlClient {
                pub fn post(&self, params: POSTParams) -> Result<String, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl OpenidClient {
            pub fn auth_url(&self) -> auth_url::AuthUrlClient {
                auth_url::AuthUrlClient::new(self.client.clone(), &self.path)
            }
        }
        pub mod login {
            pub struct LoginClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl LoginClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, "/login"),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                #[serde(rename = "redirect-url")]
                pub redirect_url: String,
                pub code: String,
                pub state: String,
            }
            impl LoginClient {
                pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
        }
        impl OpenidClient {
            pub fn login(&self) -> login::LoginClient {
                login::LoginClient::new(self.client.clone(), &self.path)
            }
        }
    }
    impl AccessClient {
        pub fn openid(&self) -> openid::OpenidClient {
            openid::OpenidClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod tfa {
        pub struct TfaClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl TfaClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/tfa"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub response: String,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct POSTReturns {
            pub ticket: String,
        }
        impl TfaClient {
            pub fn post(&self, params: POSTParams) -> Result<POSTReturns, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItemsEntriesItems {
            #[serde(
                serialize_with = "::proxmox_api::serialize_bool_optional",
                deserialize_with = "::proxmox_api::deserialize_bool_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub enable: Option<bool>,
            pub id: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int",
                deserialize_with = "::proxmox_api::deserialize_int"
            )]
            pub created: u64,
            pub description: String,
            #[serde(rename = "type")]
            pub ty: String,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsItems {
            pub entries: Vec<GETReturnsItemsEntriesItems>,
            pub userid: String,
        }
        impl TfaClient {
            pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        pub mod userid {
            pub struct UseridClient {
                client: ::std::sync::Arc<::proxmox_api::Client>,
                path: String,
            }
            impl UseridClient {
                pub fn new(
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    parent_path: &str,
                    userid: &str,
                ) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, userid),
                    }
                }
            }
            #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
            pub struct POSTParams {
                #[serde(rename = "type")]
                pub ty: String,
                pub value: Option<String>,
                pub challenge: Option<String>,
                pub totp: Option<String>,
                pub description: Option<String>,
                pub password: Option<String>,
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct POSTReturns {
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub recovery: Option<Vec<Option<String>>>,
                pub id: String,
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub challenge: Option<String>,
            }
            impl UseridClient {
                pub fn post(
                    &self,
                    params: POSTParams,
                ) -> Result<POSTReturns, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.post(&path, &params)
                }
            }
            #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
            pub struct GETReturnsItems {
                #[serde(
                    serialize_with = "::proxmox_api::serialize_bool_optional",
                    deserialize_with = "::proxmox_api::deserialize_bool_optional"
                )]
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub enable: Option<bool>,
                #[serde(rename = "type")]
                pub ty: String,
                pub id: String,
                pub description: String,
                #[serde(
                    serialize_with = "::proxmox_api::serialize_int",
                    deserialize_with = "::proxmox_api::deserialize_int"
                )]
                pub created: u64,
            }
            impl UseridClient {
                pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
                    let path = self.path.to_string();
                    self.client.get(&path, &())
                }
            }
            pub mod id {
                pub struct IdClient {
                    client: ::std::sync::Arc<::proxmox_api::Client>,
                    path: String,
                }
                impl IdClient {
                    pub fn new(
                        client: ::std::sync::Arc<::proxmox_api::Client>,
                        parent_path: &str,
                        id: &str,
                    ) -> Self {
                        Self {
                            client,
                            path: format!("{}/{}", parent_path, id),
                        }
                    }
                }
                #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
                pub struct GETReturns {
                    #[serde(rename = "type")]
                    pub ty: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_int",
                        deserialize_with = "::proxmox_api::deserialize_int"
                    )]
                    pub created: u64,
                    pub description: String,
                    pub id: String,
                    #[serde(
                        serialize_with = "::proxmox_api::serialize_bool_optional",
                        deserialize_with = "::proxmox_api::deserialize_bool_optional"
                    )]
                    #[serde(skip_serializing_if = "Option::is_none", default)]
                    pub enable: Option<bool>,
                }
                impl IdClient {
                    pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.get(&path, &())
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct DELETEParams {
                    pub password: Option<String>,
                }
                impl IdClient {
                    pub fn delete(&self, params: DELETEParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.delete(&path, &params)
                    }
                }
                #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
                pub struct PUTParams {
                    pub description: Option<String>,
                    pub enable: Option<bool>,
                    pub password: Option<String>,
                }
                impl IdClient {
                    pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                        let path = self.path.to_string();
                        self.client.put(&path, &params)
                    }
                }
            }
            impl UseridClient {
                pub fn id(&self, id: &str) -> id::IdClient {
                    id::IdClient::new(self.client.clone(), &self.path, id)
                }
            }
        }
        impl TfaClient {
            pub fn userid(&self, userid: &str) -> userid::UseridClient {
                userid::UseridClient::new(self.client.clone(), &self.path, userid)
            }
        }
    }
    impl AccessClient {
        pub fn tfa(&self) -> tfa::TfaClient {
            tfa::TfaClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod ticket {
        pub struct TicketClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl TicketClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/ticket"),
                }
            }
        }
        impl TicketClient {
            pub fn get(&self) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct POSTParams {
            pub otp: Option<String>,
            pub password: String,
            #[serde(rename = "new-format")]
            pub new_format: Option<bool>,
            pub realm: Option<String>,
            pub username: String,
            pub privs: Option<String>,
            #[serde(rename = "tfa-challenge")]
            pub tfa_challenge: Option<String>,
            pub path: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct POSTReturns {
            #[serde(rename = "CSRFPreventionToken")]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub csrfpreventiontoken: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub clustername: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub ticket: Option<String>,
            pub username: String,
        }
        impl TicketClient {
            pub fn post(&self, params: POSTParams) -> Result<POSTReturns, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.post(&path, &params)
            }
        }
    }
    impl AccessClient {
        pub fn ticket(&self) -> ticket::TicketClient {
            ticket::TicketClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod password {
        pub struct PasswordClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl PasswordClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/password"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct PUTParams {
            pub password: String,
            pub userid: String,
        }
        impl PasswordClient {
            pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl AccessClient {
        pub fn password(&self) -> password::PasswordClient {
            password::PasswordClient::new(self.client.clone(), &self.path)
        }
    }
    pub mod permissions {
        pub struct PermissionsClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl PermissionsClient {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}{}", parent_path, "/permissions"),
                }
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize, Default)]
        pub struct GETParams {
            pub userid: Option<String>,
            pub path: Option<String>,
        }
        impl PermissionsClient {
            pub fn get(&self, params: GETParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &params)
            }
        }
    }
    impl AccessClient {
        pub fn permissions(&self) -> permissions::PermissionsClient {
            permissions::PermissionsClient::new(self.client.clone(), &self.path)
        }
    }
}
pub mod pools {
    pub struct PoolsClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl PoolsClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/pools".to_string(),
            }
        }
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct GETReturnsItems {
        pub poolid: String,
    }
    impl PoolsClient {
        pub fn get(&self) -> Result<Vec<GETReturnsItems>, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
    #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
    pub struct POSTParams {
        pub poolid: String,
        pub comment: Option<String>,
    }
    impl PoolsClient {
        pub fn post(&self, params: POSTParams) -> Result<(), ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.post(&path, &params)
        }
    }
    pub mod poolid {
        pub struct PoolidClient {
            client: ::std::sync::Arc<::proxmox_api::Client>,
            path: String,
        }
        impl PoolidClient {
            pub fn new(
                client: ::std::sync::Arc<::proxmox_api::Client>,
                parent_path: &str,
                poolid: &str,
            ) -> Self {
                Self {
                    client,
                    path: format!("{}/{}", parent_path, poolid),
                }
            }
        }
        impl PoolidClient {
            pub fn delete(&self) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.delete(&path, &())
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct GETParams {
            #[serde(rename = "type")]
            pub ty: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturnsMembersItems {
            pub id: String,
            #[serde(
                serialize_with = "::proxmox_api::serialize_int_optional",
                deserialize_with = "::proxmox_api::deserialize_int_optional"
            )]
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub vmid: Option<u64>,
            pub node: String,
            #[serde(rename = "type")]
            pub ty: String,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub storage: Option<String>,
        }
        #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
        pub struct GETReturns {
            pub members: Vec<GETReturnsMembersItems>,
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub comment: Option<String>,
        }
        impl PoolidClient {
            pub fn get(&self, params: GETParams) -> Result<GETReturns, ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.get(&path, &params)
            }
        }
        #[derive(:: std :: fmt :: Debug, :: serde :: Serialize)]
        pub struct PUTParams {
            pub delete: Option<bool>,
            pub vms: Option<String>,
            pub comment: Option<String>,
            pub storage: Option<String>,
        }
        impl PoolidClient {
            pub fn put(&self, params: PUTParams) -> Result<(), ::proxmox_api::Error> {
                let path = self.path.to_string();
                self.client.put(&path, &params)
            }
        }
    }
    impl PoolsClient {
        pub fn poolid(&self, poolid: &str) -> poolid::PoolidClient {
            poolid::PoolidClient::new(self.client.clone(), &self.path, poolid)
        }
    }
}
pub mod version {
    pub struct VersionClient {
        client: ::std::sync::Arc<::proxmox_api::Client>,
        path: String,
    }
    impl VersionClient {
        pub fn new(client: ::std::sync::Arc<::proxmox_api::Client>) -> Self {
            Self {
                client,
                path: "/version".to_string(),
            }
        }
    }
    #[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
    pub struct GETReturns {
        pub release: String,
        pub version: String,
        pub repoid: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub console: Option<String>,
    }
    impl VersionClient {
        pub fn get(&self) -> Result<GETReturns, ::proxmox_api::Error> {
            let path = self.path.to_string();
            self.client.get(&path, &())
        }
    }
}