#[derive(Debug, Clone)]
pub struct MatcherFieldsClient<T> {
    client: T,
    path: String,
}
impl<T> MatcherFieldsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/matcher-fields"),
        }
    }
}
impl<T> MatcherFieldsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns known notification metadata fields"]
    #[doc = ""]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &()).await
    }
}
impl GetOutputItems {
    pub fn new(name: String) -> Self {
        Self {
            name,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Name of the field."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
