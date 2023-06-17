use std::collections::BTreeMap;

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            ConfigMap, ConfigMapEnvSource, Container, ContainerPort, EnvFromSource, Namespace,
            PodSpec, PodTemplateSpec, Secret, SecretEnvSource, Service, ServiceAccount,
            ServicePort, ServiceSpec,
        },
        networking::v1::{
            HTTPIngressPath, HTTPIngressRuleValue, Ingress, IngressBackend, IngressRule,
            IngressServiceBackend, IngressSpec, ServiceBackendPort,
        },
    },
    apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta},
};

use crate::meta::{namespace, namespaced_metadata, AddLabel};

pub fn deployment(
    namespace: &Namespace,
    name: &str,
    image: &str,
    ports: Vec<ContainerPort>,
) -> (Deployment, Vec<Service>) {
    let labels = Some(BTreeMap::from([("app".into(), name.into())]));
    let deployment = Deployment {
        metadata: namespaced_metadata(namespace, name).add_label("app", name),
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels: labels.clone(),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: labels.clone(),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.into(),
                        image: Some(image.into()),
                        ports: Some(ports.clone()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    // one service per port so that we can choose to create ingresses more easily
    let services = ports
        .iter()
        .map(|p| Service {
            metadata: namespaced_metadata(namespace, name).add_label("app", name),
            spec: Some(ServiceSpec {
                selector: labels.clone(),
                ports: Some(vec![ServicePort {
                    port: p.container_port,
                    protocol: p.protocol.clone(),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        })
        .collect();
    (deployment, services)
}

pub trait SetServiceAccount {
    fn set_service_account(&self, name: &ServiceAccount) -> Self;
}

impl SetServiceAccount for Deployment {
    fn set_service_account(&self, account: &ServiceAccount) -> Self {
        let mut result = self.clone();
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .service_account = account.metadata.name.clone();
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .service_account_name = account.metadata.name.clone();
        result
    }
}

pub trait SetEnvFromConfigMap {
    fn set_env_from_config_map(&self, config_map: &ConfigMap) -> Self;
}

impl SetEnvFromConfigMap for Deployment {
    fn set_env_from_config_map(&self, config_map: &ConfigMap) -> Self {
        let mut result = self.clone();
        let env_from = &result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .containers[0]
            .env_from;
        let mut env_from = env_from.clone().unwrap_or(Vec::new());
        env_from.push(EnvFromSource {
            config_map_ref: Some(ConfigMapEnvSource {
                name: config_map.metadata.name.clone(),
                optional: Some(false),
            }),
            ..Default::default()
        });
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .containers[0]
            .env_from = Some(env_from);
        result
    }
}

pub trait SetEnvFromSecret {
    fn set_env_from_secret(&self, secret: &Secret) -> Self;
}

impl SetEnvFromSecret for Deployment {
    fn set_env_from_secret(&self, secret: &Secret) -> Self {
        let mut result = self.clone();
        let env_from = &result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .containers[0]
            .env_from;
        let mut env_from = env_from.clone().unwrap_or(Vec::new());
        env_from.push(EnvFromSource {
            secret_ref: Some(SecretEnvSource {
                name: secret.metadata.name.clone(),
                optional: Some(false),
            }),
            ..Default::default()
        });
        result
            .spec
            .as_mut()
            .unwrap()
            .template
            .spec
            .as_mut()
            .unwrap()
            .containers[0]
            .env_from = Some(env_from);
        result
    }
}

pub enum Protocol {
    UDP,
    TCP,
    SCTP,
}

impl std::string::ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::UDP => "UDP",
            Protocol::TCP => "TCP",
            Protocol::SCTP => "SCTP",
        }
        .to_string()
    }
}

pub fn container_port(port: i32, protocol: Protocol) -> ContainerPort {
    ContainerPort {
        container_port: port,
        protocol: Some(protocol.to_string()),
        ..Default::default()
    }
}

pub trait GetIngress {
    fn ingress(&self, host_name: &str) -> Ingress;
}

impl GetIngress for Service {
    fn ingress(&self, host_name: &str) -> Ingress {
        Ingress {
            metadata: namespaced_metadata(
                &namespace(&self.metadata.namespace.clone().unwrap()),
                &self.metadata.name.clone().unwrap(),
            ),
            spec: Some(IngressSpec {
                rules: Some(vec![IngressRule {
                    host: Some(host_name.into()),
                    http: Some(HTTPIngressRuleValue {
                        paths: vec![HTTPIngressPath {
                            backend: IngressBackend {
                                service: Some(IngressServiceBackend {
                                    name: self.metadata.name.clone().unwrap(),
                                    port: Some(ServiceBackendPort {
                                        number: Some(
                                            self.spec.clone().unwrap().ports.unwrap()[0].port,
                                        ),
                                        ..Default::default()
                                    }),
                                }),
                                ..Default::default()
                            },
                            path: Some("/".to_string()),
                            path_type: "Prefix".to_string(),
                        }],
                    }),
                }]),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
