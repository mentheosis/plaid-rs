use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::investments_holdings_get`].

On request success, this will return a [`InvestmentsHoldingsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetRequest {
    pub access_token: String,
    pub options: Option<InvestmentHoldingsGetRequestOptions>,
}
impl InvestmentsHoldingsGetRequest {}
impl FluentRequest<'_, InvestmentsHoldingsGetRequest> {
    pub fn options(mut self, options: InvestmentHoldingsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, InvestmentsHoldingsGetRequest> {
    type Output = httpclient::InMemoryResult<InvestmentsHoldingsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/investments/holdings/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}