use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_token_create`].

On request success, this will return a [`ProcessorTokenCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl ProcessorTokenCreateRequest {}
impl FluentRequest<'_, ProcessorTokenCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, ProcessorTokenCreateRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/token/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}