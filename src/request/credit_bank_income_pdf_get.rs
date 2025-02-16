use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::credit_bank_income_pdf_get`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetRequest {
    pub user_token: String,
}
impl CreditBankIncomePdfGetRequest {}
impl FluentRequest<'_, CreditBankIncomePdfGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, CreditBankIncomePdfGetRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/credit/bank_income/pdf/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}