use tonic::metadata::{Ascii, MetadataMap, MetadataValue};
use tonic::{Extensions, Request};
use wink::{
    GetMinerConfigurationRequest, LoginRequest, PauseMiningRequest, ResumeMiningRequest,
    actions_service_client::ActionsServiceClient,
    authentication_service_client::AuthenticationServiceClient,
    configuration_service_client::ConfigurationServiceClient,
};

pub struct Braiins {
    pub ip: String,
    pub username: String,
    pub password: String,
    auth_token: Option<MetadataValue<Ascii>>,
}

impl Braiins {
    pub fn new(ip: String, username: String, password: String) -> Self {
        Self {
            ip,
            username,
            password,
            auth_token: None,
        }
    }

    fn server_addr(&self) -> String {
        format!("http://{}:50051", self.ip)
    }

    fn authenticated_request<T>(
        &self,
        message: T,
    ) -> Result<Request<T>, Box<dyn std::error::Error>> {
        let token = self
            .auth_token
            .as_ref()
            .ok_or("Not authenticated; call login() first")?;

        let mut metadata = MetadataMap::new();
        metadata.insert("authorization", token.clone());

        Ok(Request::from_parts(
            metadata,
            Extensions::default(),
            message,
        ))
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = AuthenticationServiceClient::connect(self.server_addr()).await?;

        let request = Request::new(LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        });

        let response = client.login(request).await?;

        self.auth_token = response.metadata().get("authorization").cloned().into();

        Ok(())
    }

    pub async fn get_configuration(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.authenticated_request(GetMinerConfigurationRequest {})?;
        let mut client = ConfigurationServiceClient::connect(self.server_addr()).await?;
        let response = client.get_miner_configuration(request).await?;

        println!("Miner Configuration: {:?}", response.into_inner());

        Ok(())
    }

    /// Pauses mining on the miner.
    pub async fn pause_miner(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.authenticated_request(PauseMiningRequest {})?;
        let mut client = ActionsServiceClient::connect(self.server_addr()).await?;
        client.pause_mining(request).await?;
        Ok(())
    }

    /// Resumes mining on the miner.
    pub async fn resume_miner(&self) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.authenticated_request(ResumeMiningRequest {})?;
        let mut client = ActionsServiceClient::connect(self.server_addr()).await?;
        client.resume_mining(request).await?;
        Ok(())
    }
}
