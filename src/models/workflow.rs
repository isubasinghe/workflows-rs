#![allow(unused_imports)]

use crate::models::k8s_simple::*;
use chrono::{DateTime, Utc};

use futures::StreamExt;

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

pub static DOCUMENT_FINALIZER: &str = "documents.kube.rs";

/// Generate the Kubernetes wrapper struct `Document` from our Spec and Status struct
///
/// This provides a hook for generating the CRD yaml (in crdgen.rs)
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Default))]
#[kube(kind = "Workflow", group = "kube.rs", version = "v1", namespaced)]
#[kube(status = "WorkflowStatus", shortname = "doc")]
pub struct WorkflowSpec {
    pub templates: Vec<Template>,
    pub entrypoint: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Parameter {}

pub type Parameters = Vec<Parameter>;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Artifact {}

pub type Artifacts = Vec<Artifact>;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
pub struct Input {
    parameters: Option<Parameters>,
    artifacts: Option<Artifact>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Output {
    parameters: Option<Parameters>,
    artifacts: Option<Artifact>,
    result: Option<String>,
    exit_code: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DAGTask {
    pub name: String,
    pub template: String,
    pub dependencies: Option<Vec<String>>,
    pub depends: Option<String>,
    pub inputs: Option<Vec<Input>>,
    pub outputs: Option<Vec<Output>>,
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DAGTemplate {
    pub target: Option<String>,
    pub tasks: Vec<DAGTask>,
    pub fail_fast: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Steps {}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Script {}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Template {
    pub name: String,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub dag: Option<DAGTemplate>,
    pub container: Option<Container>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub enum TaskStatus {
    Running,
    Stopped,
    Errored,
    Pending,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
pub struct WorkflowStatus {
    pub status: HashMap<String, TaskStatus>,
}
