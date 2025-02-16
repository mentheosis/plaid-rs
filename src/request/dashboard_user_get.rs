use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::dashboard_user_get`].

On request success, this will return a [`DashboardUserGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserGetRequest {
    pub dashboard_user_id: String,
}
impl DashboardUserGetRequest {}
impl FluentRequest<'_, DashboardUserGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DashboardUserGetRequest> {
    type Output = httpclient::InMemoryResult<DashboardUserGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/dashboard_user/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}