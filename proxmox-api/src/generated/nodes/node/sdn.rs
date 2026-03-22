pub mod fabrics;
pub mod vnets;
pub mod zones;
#[derive(Debug, Clone)]
pub struct SdnClient<T> {
    client: T,
    path: String,
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/sdn"),
        }
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN index."]
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
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn fabrics(&self) -> fabrics::FabricsClient<T> {
        fabrics::FabricsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn zones(&self) -> zones::ZonesClient<T> {
        zones::ZonesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn vnets(&self) -> vnets::VnetsClient<T> {
        vnets::VnetsClient::<T>::new(self.client.clone(), &self.path)
    }
}
