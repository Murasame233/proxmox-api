pub mod certificate;
#[derive(Debug, Clone)]
pub struct AcmeClient<T> {
    client: T,
    path: String,
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/acme"),
        }
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ACME index."]
    #[doc = ""]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn certificate(&self) -> certificate::CertificateClient<T> {
        certificate::CertificateClient::<T>::new(self.client.clone(), &self.path)
    }
}
