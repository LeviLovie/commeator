use dioxus::prelude::*;
use prost::Message;

use crate::request::backend;

#[derive(Clone)]
pub struct FetchState<R, D> {
    pub last_req: Option<R>,
    pub data: Option<D>,
    pub loading: bool,
    pub error: Option<String>,
}

pub fn use_fetch<Req, Resp>(
    url: impl ToString,
    jwt: impl ToString,
    req: Req,
) -> Signal<FetchState<Req, Resp>>
where
    Req: 'static + std::fmt::Debug + Clone + PartialEq + Message,
    Resp: 'static + std::fmt::Debug + Default + Message,
{
    let url = url.to_string();
    let jwt = jwt.to_string();

    let state = use_signal(|| FetchState::<Req, Resp> {
        last_req: None,
        data: None,
        loading: true,
        error: None,
    });

    spawn({
        let url = url.clone();
        let jwt = jwt.clone();
        let req = req.clone();
        let mut state = state.clone();

        async move {
            if let Some(last_req) = &state.read().last_req {
                if *last_req == req {
                    return;
                }
            }

            state.set(FetchState {
                last_req: Some(req.clone()),
                data: None,
                loading: true,
                error: None,
            });

            let result = backend::<Req, Resp>(&url, jwt, req.clone()).await;

            match result {
                Ok(data) => state.set(FetchState {
                    last_req: Some(req),
                    data: Some(data),
                    loading: false,
                    error: None,
                }),
                Err(e) => state.set(FetchState {
                    last_req: Some(req),
                    data: None,
                    loading: false,
                    error: Some(format!("{e:?}")),
                }),
            }
        }
    });

    state
}
