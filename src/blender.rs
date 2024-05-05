use std::hash::Hash;
use std::sync::{Arc, RwLock};
use iced::advanced::graphics::futures::BoxStream;
use iced::advanced::subscription::{EventStream, Hasher, Recipe};
use iced::subscription;
use iced::Subscription;
use crate::app::Message;

pub struct Build {
    hash: String,
    version: String,
    release_cycle: String,
}

pub enum Type {
    Daily,
    Experimental,
    Patch,
}

pub struct Repository {
    remote: Arc<RwLock<Vec<(Type, Build)>>>,
    local: Arc<RwLock<Vec<(Type, Build)>>>,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            remote: Arc::new(RwLock::new(vec![])),
            local: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl Repository {
    pub fn subscription(&self) -> Subscription<Message> {
        subscription::(|_sink| {
            let stream = EventStream::new(self.clone());

            Box::pin(stream)
        })
    }
}