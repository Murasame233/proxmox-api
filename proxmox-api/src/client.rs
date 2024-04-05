use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    Post,
    Get,
    Put,
    Delete,
}

pub trait Client: Clone {
    type Error: core::fmt::Debug;

    /// Transmit an authenticated request to a Proxmox VE API endpoint
    /// using the provided method, path, body, and query.
    fn request_with_body_and_query<B, Q, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
        query: Option<&Q>,
    ) -> Result<R, Self::Error>
    where
        B: Serialize,
        Q: Serialize,
        R: DeserializeOwned;

    fn request_with_body<B, R>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body_and_query::<_, (), _>(method, path, Some(body), None)
    }

    fn request_with_query<Q, R>(
        &self,
        method: Method,
        path: &str,
        query: &Q,
    ) -> Result<R, Self::Error>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body_and_query::<(), _, _>(method, path, None, Some(query))
    }

    fn put<B, R>(&self, path: &str, body: &B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body(Method::Put, path, body)
    }

    fn post<B, R>(&self, path: &str, body: &B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body(Method::Post, path, body)
    }

    fn delete<B, R>(&self, path: &str, body: &B) -> Result<R, Self::Error>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_body(Method::Delete, path, body)
    }

    fn get<Q, R>(&self, path: &str, query: &Q) -> Result<R, Self::Error>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        self.request_with_query(Method::Get, path, query)
    }
}