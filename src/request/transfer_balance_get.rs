use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_balance_get`].

On request success, this will return a [`TransferBalanceGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferBalanceGetRequest {
    pub originator_client_id: Option<String>,
    pub type_: Option<String>,
}
impl TransferBalanceGetRequest {}
impl FluentRequest<'_, TransferBalanceGetRequest> {
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferBalanceGetRequest> {
    type Output = httpclient::InMemoryResult<TransferBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/balance/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}