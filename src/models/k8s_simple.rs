#![allow(unused_imports)]
use kube::{
    api::{Api, ListParams, Patch, PatchParams, ResourceExt},
    client::Client,
    runtime::{
        controller::{Action, Controller},
        events::{Event, EventType, Recorder, Reporter},
        finalizer::{finalizer, Event as Finalizer},
        watcher::Config,
    },
    CustomResource, Resource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::RwLock, time::Duration};
use tracing::*;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ContainerPort {}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct EnvFrom {}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct EnvVar {}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct Container {
    name: String,
    image: Option<String>,
    command: Option<Vec<String>>,
    args: Option<Vec<String>>,
}
