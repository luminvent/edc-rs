use crate::{
    client::EdcConnectorClientInternal,
    types::{context::WithContext, dataplane::DataPlaneInstance},
    EdcResult,
};

pub struct DataPlaneApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> DataPlaneApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> DataPlaneApi<'a> {
        DataPlaneApi(client)
    }

    pub async fn list(&self) -> EdcResult<Vec<DataPlaneInstance>> {
        let url = self.get_endpoint(&[]);
        self.0
            .get::<Vec<WithContext<DataPlaneInstance>>>(url)
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "dataplanes"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}
