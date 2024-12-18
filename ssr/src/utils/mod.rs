use futures::{Future, StreamExt};
use leptos::{create_memo, Resource, Serializable, Signal, SignalStream, SignalWith};
use serde::{Deserialize, Serialize};

pub mod ab_testing;
pub mod event_streaming;
pub mod host;
pub mod icon;
pub mod ml_feed;
pub mod notifications;
pub mod posts;
pub mod profile;
#[cfg(feature = "qstash")]
pub mod qstash;
pub mod report;
pub mod route;
pub mod time;
pub mod token;
pub mod types;
pub mod user;
pub mod web;

/// Wrapper for PartialEq that always returns false
/// this is currently only used for resources
/// this does not provide a sane implementation of PartialEq
#[derive(Clone, Serialize, Deserialize)]
pub struct MockPartialEq<T>(pub T);

impl<T> PartialEq for MockPartialEq<T> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct ParentResource<S: 'static + Clone, T: 'static + Clone>(pub Resource<S, T>);

impl<S: 'static + Clone, T: 'static + Clone> ParentResource<S, T> {
    /// Derive another resource that depends on this resource
    /// Note: the source is not memoized like it is for resources
    pub fn derive<
        DS: 'static + Clone,
        DT: 'static + Serializable,
        F: Future<Output = DT> + 'static,
    >(
        &self,
        source: impl Fn() -> DS + 'static,
        fetcher: impl Fn(T, DS) -> F + Clone + 'static,
    ) -> Resource<MockPartialEq<DS>, DT> {
        let parent = self.0;
        let tracker = create_memo(move |prev| {
            let prev: bool = prev.copied().unwrap_or_default();
            let parent_is_none = parent.with(|p| p.is_none());
            // If parent is none -> Resource is reloading
            if parent_is_none {
                !prev
            // resource is loaded -> we were already waiting for it, so we don't need to reload
            } else {
                prev
            }
        });

        let parent_signal = Signal::derive(parent);
        Resource::new(
            move || {
                tracker();
                MockPartialEq(source())
            },
            move |st| {
                let mut val_st = parent_signal.to_stream();
                let fetcher = fetcher.clone();
                async move {
                    let val = loop {
                        let res = val_st.next().await.expect("Signal stream ended?!");
                        if let Some(val) = res {
                            break val;
                        }
                    };
                    fetcher(val, st.0).await
                }
            },
        )
    }

    pub async fn wait_untracked(&self) -> T {
        let parent = self.0;
        let parent_signal = Signal::derive(parent);
        let mut val_st = parent_signal.to_stream();
        loop {
            let res = val_st.next().await.expect("Signal stream ended?!");
            if let Some(val) = res {
                return val;
            }
        }
    }
}

use std::fmt::Display;

use crate::consts::CF_STREAM_BASE;

pub fn bg_url(uid: impl Display) -> String {
    format!("{CF_STREAM_BASE}/{uid}/thumbnails/thumbnail.jpg")
}

pub fn stream_url(uid: impl Display) -> String {
    format!("{CF_STREAM_BASE}/{uid}/manifest/video.m3u8")
}

pub fn mp4_url(uid: impl Display) -> String {
    format!("{CF_STREAM_BASE}/{uid}/downloads/default.mp4")
}

#[cfg(all(feature = "ga4", feature = "ssr"))]
pub mod off_chain {
    tonic::include_proto!("off_chain");
}
