pub struct RrdClient<T> {
    client: T,
    path: String,
}
impl<T> RrdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rrd"),
        }
    }
}
impl<T> RrdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read node RRD statistics (returns PNG)"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub filename: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(ds: String, timeframe: Timeframe) -> Self {
        Self {
            ds,
            timeframe,
            cf: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The RRD consolidation function"]
    #[doc = ""]
    pub cf: Option<Cf>,
    #[doc = "The list of datasources you want to display."]
    #[doc = ""]
    pub ds: String,
    #[doc = "Specify the time frame you are interested in."]
    #[doc = ""]
    pub timeframe: Timeframe,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The RRD consolidation function"]
#[doc = ""]
pub enum Cf {
    AVERAGE,
    MAX,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Specify the time frame you are interested in."]
#[doc = ""]
pub enum Timeframe {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}
