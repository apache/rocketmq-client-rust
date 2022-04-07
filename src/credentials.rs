#[derive(Debug, PartialEq)]
pub struct Credentials {
    access_key: String,
    access_secret: String,
    session_token: Option<String>,
}

pub trait CredentialProvider {
    fn get_credentials(&self) -> Credentials;
}

pub struct StaticCredentialProvider {
    access_key: String,
    access_secret: String,
}

impl StaticCredentialProvider {
    pub fn new(access_key: &str, access_secret: &str) -> Self {
        Self {
            access_key: access_key.to_owned(),
            access_secret: access_secret.to_owned(),
        }
    }
}

impl CredentialProvider for StaticCredentialProvider {
    fn get_credentials(&self) -> Credentials {
        Credentials {
            access_key: self.access_key.clone(),
            access_secret: self.access_secret.clone(),
            session_token: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct EnvironmentVariableCredentialProvider {
    access_key: String,
    access_secret: String,
}

impl EnvironmentVariableCredentialProvider {
    pub fn new() -> Result<Self, std::env::VarError> {
        let access_key = std::env::var("ACCESS_KEY")?;
        let access_secret = std::env::var("ACCESS_SECRET")?;
        Ok(Self {
            access_key,
            access_secret,
        })
    }
}

impl CredentialProvider for EnvironmentVariableCredentialProvider {
    fn get_credentials(&self) -> Credentials {
        Credentials {
            access_key: self.access_key.clone(),
            access_secret: self.access_secret.clone(),
            session_token: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_static_credentials_provider() {
        let provider = StaticCredentialProvider::new("ak", "as");
        let credentials = provider.get_credentials();
        assert_eq!(
            credentials,
            Credentials {
                access_key: String::from("ak"),
                access_secret: String::from("as"),
                session_token: None,
            }
        );
    }

    #[test]
    fn test_environment_variable_credential_provider() {
        let env_credentials_provider = EnvironmentVariableCredentialProvider::new();
        assert_eq!(true, env_credentials_provider.is_err());
    }
}
