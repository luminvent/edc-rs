use crate::{
    client::EdcConnectorClientInternal,
    types::{
        context::{WithContext, WithContextRef},
        policy::{NewPolicyDefinition, PolicyDefinition},
        query::Query,
        response::IdResponse,
    },
    EdcResult,
};

pub struct PolicyApi<'a>(&'a EdcConnectorClientInternal);

impl<'a> PolicyApi<'a> {
    pub(crate) fn new(client: &'a EdcConnectorClientInternal) -> PolicyApi<'a> {
        PolicyApi(client)
    }

    pub async fn create(
        &self,
        policy_definition: &NewPolicyDefinition,
    ) -> EdcResult<IdResponse<String>> {
        let url = self.get_endpoint(&[]);
        self.0
            .post::<_, WithContext<IdResponse<String>>>(
                url,
                &WithContextRef::odrl_context(policy_definition),
            )
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn get(&self, id: &str) -> EdcResult<PolicyDefinition> {
        let url = self.get_endpoint(&[id]);
        self.0
            .get::<WithContext<PolicyDefinition>>(url)
            .await
            .map(|ctx| ctx.inner)
    }

    pub async fn update(&self, policy_definition: &PolicyDefinition) -> EdcResult<()> {
        let url = self.get_endpoint(&[policy_definition.id()]);
        self.0
            .put(url, &WithContextRef::odrl_context(policy_definition))
            .await
    }

    pub async fn query(&self, query: Query) -> EdcResult<Vec<PolicyDefinition>> {
        let url = self.get_endpoint(&["request"]);
        self.0
            .post::<_, Vec<WithContext<PolicyDefinition>>>(
                url,
                &WithContextRef::default_context(&query),
            )
            .await
            .map(|results| results.into_iter().map(|ctx| ctx.inner).collect())
    }

    pub async fn delete(&self, id: &str) -> EdcResult<()> {
        let url = self.get_endpoint(&[id]);
        self.0.del(url).await
    }

    fn get_endpoint(&self, paths: &[&str]) -> String {
        [self.0.management_url.as_str(), "v3", "policydefinitions"]
            .into_iter()
            .chain(paths.iter().copied())
            .collect::<Vec<_>>()
            .join("/")
    }
}
