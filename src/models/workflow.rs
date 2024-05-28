#![allow(unused_imports)]

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

#[derive( Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub enum Input {
    Param,
    Null
}

#[derive( Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub enum Output {
    Param,
    Null
}

impl Default for Output {
    fn default() -> Self {
        Output::Null
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::Null
    }
}

#[derive(Default, Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DAGTask {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub inputs: Option<Vec<Input>>,
    pub outputs: Option<Vec<Output>>
}


#[derive(Default, Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DAGTemplate {
    pub tasks: Vec<DAGTask>
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Template {
    pub name: String,
    pub dag: DAGTemplate,
    pub share_pod: Option<bool>,
}

impl Default for Template {
    fn default() -> Self {
        let name = String::default();
        let dag = DAGTemplate::default();
        let share_pod = Some(false);
        Self {
            name,
            dag,
            share_pod,
        }
    }
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
