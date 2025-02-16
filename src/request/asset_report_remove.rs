use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::asset_report_remove`].

On request success, this will return a [`AssetReportRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReportRemoveRequest {
    pub asset_report_token: String,
}
impl AssetReportRemoveRequest {}
impl FluentRequest<'_, AssetReportRemoveRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, AssetReportRemoveRequest> {
    type Output = httpclient::InMemoryResult<AssetReportRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/asset_report/remove";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}