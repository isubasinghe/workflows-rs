use kube::CustomResourceExt;
use workflows_rs::models::workflow::Workflow;
fn main() {
    print!("{}", serde_yaml::to_string(&Workflow::crd()).unwrap())
}
