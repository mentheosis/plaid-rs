use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::processor_transactions_recurring_get`].

On request success, this will return a [`ProcessorTransactionsRecurringGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorTransactionsRecurringGetRequest {
    pub options: Option<TransactionsRecurringGetRequestOptions>,
    pub processor_token: String,
}
impl ProcessorTransactionsRecurringGetRequest {}
impl FluentRequest<'_, ProcessorTransactionsRecurringGetRequest> {
    pub fn options(mut self, options: TransactionsRecurringGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, ProcessorTransactionsRecurringGetRequest> {
    type Output = httpclient::InMemoryResult<ProcessorTransactionsRecurringGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/processor/transactions/recurring/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}