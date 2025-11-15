use chrono::{DateTime, NaiveDateTime};
use dioxus::{
    prelude::{debug, error, trace, warn},
    router::navigator,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{AUTH_LOGIN, AUTH_LOGOUT, CONFIG},
    request::{backend, backend_get},
    services,
};
use proto::{GenJwtResp, VerifyJwt, VerifyJwtResp};

const FILE: &str = "auth.ron";
const MAX_STEPS: u32 = 50;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct File {
    jwt: Option<String>,
    expires: Option<NaiveDateTime>,
    regen: Option<NaiveDateTime>,
}

pub enum State {
    Save(File),
    Load,
    FetchKratos,
    VerifyJWT(File),
    GenJWT,
    Logout,
    Login,
    AwaitCallback,
    Authenticated(File),
    Failed(String),
}

pub struct Auth {
    state: State,
}

impl Auth {
    pub fn new() -> Self {
        Self { state: State::Load }
    }

    pub async fn run(mut self) -> Self {
        debug!("Starting auth state machine");
        loop {
            if let State::Failed(msg) = self.state {
                error!("Auth state machine failed: {}", msg);
                panic!();
            }

            if matches![self.state, State::AwaitCallback | State::Authenticated(_)] {
                break;
            }

            debug!("Auth State: {}", self.state);
            self.step().await;
        }

        debug!("Final Auth State: {}", self.state);
        self
    }

    pub fn get_jwt(&self) -> String {
        match &self.state {
            State::Authenticated(file) => file.jwt.clone().unwrap_or_default(),
            _ => {
                warn!(
                    "Attempted to get JWT from unauthenticated state: {}",
                    self.state
                );
                String::new()
            }
        }
    }

    async fn step(&mut self) {
        match &self.state {
            State::Save(file) => match services::Storage::new().save_block(FILE, &file) {
                Ok(_) => self.state = State::Authenticated(file.clone()),
                Err(e) => {
                    self.state = State::Failed(format!("Failed to save auth file: {}", e));
                }
            },
            State::Load => match services::Storage::new().load_block::<File>(FILE) {
                Ok(file) => {
                    self.state = State::VerifyJWT(file);
                }
                Err(e) => {
                    warn!("No auth file found, fetching new credentials: {}", e);
                    self.state = State::FetchKratos;
                }
            },
            State::FetchKratos => match kratos::fetch().await {
                Ok(Some(_)) => {
                    self.state = State::GenJWT;
                }
                Ok(None) => {
                    self.state = State::Login;
                }
                Err(e) => {
                    self.state =
                        State::Failed(format!("Failed to fetch Kratos credentials: {}", e));
                }
            },
            State::VerifyJWT(file) => {
                let req = VerifyJwt {
                    token: file.jwt.clone().unwrap_or_default(),
                };
                let verify_resp: VerifyJwtResp = backend("/j/verify", "", req).await;
                if verify_resp.valid {
                    dioxus::prelude::info!("Loaded valid JWT from storage");
                    self.state = State::Authenticated(file.clone());
                } else {
                    dioxus::prelude::info!("Stored JWT is invalid or expired, generating new one");
                    self.state = State::FetchKratos;
                }
            }
            State::GenJWT => {
                let jwt_resp: GenJwtResp = backend_get("/j/gen", "").await;
                self.state = State::Save(File {
                    jwt: Some(jwt_resp.token),
                    expires: Some(
                        DateTime::from_timestamp(
                            jwt_resp
                                .expires_at
                                .parse()
                                .expect("Invalid expiration timestamp"),
                            0,
                        )
                        .unwrap()
                        .naive_utc(),
                    ),
                    regen: Some(chrono::Utc::now().naive_utc()),
                });
            }
            State::Logout => {
                self.state = State::AwaitCallback;
                navigator().replace(format!("{}{}", CONFIG.url_auth, AUTH_LOGOUT));
            }
            State::Login => {
                self.state = State::AwaitCallback;
                navigator().replace(format!("{}{}", CONFIG.url_auth, AUTH_LOGIN));
            }
            State::AwaitCallback => {}
            State::Authenticated(file) => {
                debug!("Authenticated with JWT: {:?}", file.jwt);
            }
            State::Failed(msg) => {
                error!("Auth state machine failed: {}", msg);
            }
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Save(file) => write!(f, "Save; jwt: {:?}", file.jwt),
            State::Load => write!(f, "Load"),
            State::FetchKratos => write!(f, "FetchKratos"),
            State::VerifyJWT(file) => write!(f, "VerifyJWT; jwt: {:?}", file.jwt),
            State::GenJWT => write!(f, "GenJWT"),
            State::Logout => write!(f, "Logout"),
            State::Login => write!(f, "Login"),
            State::AwaitCallback => write!(f, "AwaitCallback"),
            State::Authenticated(_) => write!(f, "Authenticated"),
            State::Failed(msg) => write!(f, "Failed({})", msg),
        }
    }
}

mod kratos {
    use anyhow::{Context, Result, bail};
    use serde::Deserialize;

    use crate::{
        config::{AUTH_WHOAMI, CONFIG},
        request::Request,
    };

    #[derive(Deserialize, Debug, Clone)]
    pub struct KratosUserData {
        pub identity: Identity,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Identity {
        pub traits: Traits,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Traits {
        pub email: String,
    }

    pub async fn fetch() -> Result<Option<String>> {
        let response = Request::get(format!("{}{}", CONFIG.url_auth, AUTH_WHOAMI))
            .build()
            .send()
            .await
            .context("Failed to send request to Kratos")?;
        match response.status() {
            200 => response
                .json::<KratosUserData>()
                .and_then(|data| Ok(Some(data.identity.traits.email))),
            401 => Ok(None),
            _ => {
                bail!("Unexpected response from Kratos: {}", response.status());
            }
        }
    }
}
