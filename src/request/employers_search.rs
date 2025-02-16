use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::employers_search`].

On request success, this will return a [`EmployersSearchResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployersSearchRequest {
    pub products: Vec<String>,
    pub query: String,
}
impl EmployersSearchRequest {}
impl FluentRequest<'_, EmployersSearchRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, EmployersSearchRequest> {
    type Output = httpclient::InMemoryResult<EmployersSearchResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/employers/search";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}