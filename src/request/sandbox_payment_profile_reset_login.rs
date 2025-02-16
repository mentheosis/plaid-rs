use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::sandbox_payment_profile_reset_login`].

On request success, this will return a [`SandboxPaymentProfileResetLoginResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxPaymentProfileResetLoginRequest {
    pub payment_profile_token: String,
}
impl SandboxPaymentProfileResetLoginRequest {}
impl FluentRequest<'_, SandboxPaymentProfileResetLoginRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, SandboxPaymentProfileResetLoginRequest> {
    type Output = httpclient::InMemoryResult<SandboxPaymentProfileResetLoginResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/sandbox/payment_profile/reset_login";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}