use dioxus::prelude::*;
use std::future::Future;

#[derive(Clone, Debug, PartialEq)]
pub enum ApiData<T> {
    Loading,
    Loaded(T),
    Error(String),
}

impl<T> ApiData<T> {
    pub fn loading() -> Self {
        Self::Loading
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn loaded(value: T) -> Self {
        Self::Loaded(value)
    }

    pub fn error(err: impl ToString) -> Self {
        Self::Error(err.to_string())
    }

    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Loaded(v) => Some(v),
            _ => None,
        }
    }
}

pub fn use_api_data<T, F, Fut>(load_fn: F) -> Signal<ApiData<T>>
where
    T: 'static + Clone + PartialEq,
    F: Fn() -> Fut + Copy + 'static,
    Fut: Future<Output = Result<T, ServerFnError>> + 'static,
{
    let data = use_signal(|| ApiData::loading());

    use_future({
        move || {
            let mut data = data;
            async move {
                info!("Starting data load");
                match load_fn().await {
                    Ok(result) => data.set(ApiData::loaded(result)),
                    Err(err) => data.set(ApiData::error(err)),
                }
            }
        }
    });

    data
}
