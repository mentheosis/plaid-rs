use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_sessions_get`].

On request success, this will return a [`CreditSessionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditSessionsGetRequest {
    pub user_token: String,
}
impl CreditSessionsGetRequest {}
impl FluentRequest<'_, CreditSessionsGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditSessionsGetRequest> {
    type Output = httpclient::InMemoryResult<CreditSessionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/sessions/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}