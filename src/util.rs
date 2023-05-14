use serde::ser;

pub fn to_yaml<T>(value: &T) -> String
where
    T: ?Sized + ser::Serialize,
{
    serde_yaml::to_string(value).unwrap()
}

pub fn combine_yamls(yamls: Vec<String>) -> String {
    yamls
        .iter()
        .fold("".to_string(), |acc, y| format!("{acc}---\n{y}"))
}
