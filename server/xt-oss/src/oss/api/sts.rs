pub mod builders {
    use crate::oss;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssumeRoleQuery<'a> {
        #[serde(rename = "DurationSeconds")]
        pub duration_seconds: Option<u32>,
        #[serde(rename = "RoleArn")]
        pub role_arn: &'a str,
        #[serde(rename = "RoleSessionName")]
        pub role_session_name: &'a str,
        #[serde(rename = "Policy")]
        pub policy: Option<&'a str>,
    }

    impl<'a> fmt::Display for AssumeRoleQuery<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", serde_qs::to_string(self).unwrap())
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssumeRoleUser{
        #[serde(rename = "AssumedRoleId")]
        pub assumed_role_id: String,
        #[serde(rename = "Arn")]
        pub arn: String,
        pub credentials: AssumeRoleCredentials,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AssumeRoleCredentials {
        #[serde(rename = "SecurityToken")]
        pub security_token: String,
        #[serde(rename = "Expiration")]
        pub expiration: String,
        #[serde(rename = "AccessKeyId")]
        pub access_key_id: String,
        #[serde(rename = "AccessKeySecret")]
        pub access_key_secret: String,
        #[serde(rename = "SourceIdentity")]
        pub source_identity: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub  struct  AssumeRoleResponse {
        #[serde(rename = "RequestId")]
        pub request_id: String,
        #[serde(rename = "AssumedRoleUser")]
        pub assumed_role_user: AssumeRoleUser,
    }

    impl<'a> Default for AssumeRoleQuery<'a> {
        fn default() -> Self {
            Self {
                duration_seconds: Some(3600),
                role_arn: "",
                role_session_name: "",
                policy: None,
            }
        }
    }

    pub struct GetStsTokenBuilder<'a> {
        client: &'a oss::Client<'a>,
        query: AssumeRoleQuery<'a>,
    }

    impl<'a> GetStsTokenBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                query: AssumeRoleQuery::default(),
            }
        }

        pub fn with_duration_seconds(mut self,duration_seconds: u32) -> Self {
            self.query.duration_seconds = Some(duration_seconds);
            self
        }

        pub fn with_session_name(mut self,session_name: &'a str) -> Self {
            self.query.role_session_name = session_name;
            self
        }
        
        pub fn with_policy(mut self,policy: &'a str) -> Self {
            self.query.policy = Some(policy);
            self
        }

        pub async fn execute(&self) -> oss::Result<AssumeRoleResponse> {
            let mut url=self.client.base_url();
            let query=self.query.to_string();
            if !query.is_empty() {
                url=format!("{}?{}",url,query);
            }
            let resp=self.client.request.task().with_url(&url).with_method(http::Method::POST).with_resource("/").execute().await?;
            let resp_body=resp.text().await?;
            let resp:AssumeRoleResponse=serde_json::from_str(&resp_body)?;
            Ok(resp)
        }
    }
    
}

impl<'a> oss::Client<'a> {
    pub fn GetStsToken(&self) -> GetStsTokenBuilder {
        GetStsTokenBuilder::new(self)
    }

    
}