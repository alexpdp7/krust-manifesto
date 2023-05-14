use k8s_openapi::api::core::v1::Namespace;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub fn namespace(name: &str) -> Namespace {
    Namespace {
        metadata: metadata(name),
        ..Default::default()
    }
}

pub fn metadata(name: &str) -> ObjectMeta {
    ObjectMeta {
        name: Some(name.into()),
        ..Default::default()
    }
}

pub fn namespaced_metadata(namespace: &Namespace, name: &str) -> ObjectMeta {
    ObjectMeta {
        name: Some(name.into()),
        namespace: namespace.metadata.name.clone(),
        ..Default::default()
    }
}

pub trait AddLabel {
    fn add_label(&self, name: &str, value: &str) -> Self;
}

impl AddLabel for ObjectMeta {
    fn add_label(&self, name: &str, value: &str) -> Self {
        let mut result = self.clone();
        let mut labels = result.labels.unwrap_or_default();
        labels.insert(name.into(), value.into());
        result.labels = Some(labels);
        result
    }
}
