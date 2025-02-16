use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::wallet_transaction_execute`].

On request success, this will return a [`WalletTransactionExecuteResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequest {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: String,
    pub reference: String,
    pub wallet_id: String,
}
impl WalletTransactionExecuteRequest {}
pub struct WalletTransactionExecuteRequired<'a> {
    pub amount: WalletTransactionAmount,
    pub counterparty: WalletTransactionCounterparty,
    pub idempotency_key: &'a str,
    pub reference: &'a str,
    pub wallet_id: &'a str,
}
impl<'a> WalletTransactionExecuteRequired<'a> {}
impl FluentRequest<'_, WalletTransactionExecuteRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WalletTransactionExecuteRequest> {
    type Output = httpclient::InMemoryResult<WalletTransactionExecuteResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/wallet/transaction/execute";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}