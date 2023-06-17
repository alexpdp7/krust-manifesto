use std::collections::BTreeMap;

use k8s_openapi::api::core::v1::{ConfigMap, Namespace, Secret};

use crate::meta::namespaced_metadata;

pub fn config_map(namespace: &Namespace, name: &str, data: BTreeMap<String, String>) -> ConfigMap {
    ConfigMap {
        data: Some(data),
        metadata: namespaced_metadata(namespace, name),
        ..Default::default()
    }
}

pub fn secret(namespace: &Namespace, name: &str, data: BTreeMap<String, String>) -> Secret {
    Secret {
        string_data: Some(data),
        metadata: namespaced_metadata(namespace, name),
        ..Default::default()
    }
}
