use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_bank_income_refresh`].

On request success, this will return a [`CreditBankIncomeRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequest {
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
    pub user_token: String,
}
impl CreditBankIncomeRefreshRequest {}
impl FluentRequest<'_, CreditBankIncomeRefreshRequest> {
    pub fn options(mut self, options: CreditBankIncomeRefreshRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, CreditBankIncomeRefreshRequest> {
    type Output = httpclient::InMemoryResult<CreditBankIncomeRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/bank_income/refresh";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}