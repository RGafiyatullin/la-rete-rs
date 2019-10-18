#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsPath {
    pub(super) path: Vec<String>,
}

impl JsPath {
    pub fn from_path<S: AsRef<str>, II: IntoIterator<Item = S>>(path: II) -> Self {
        let path = path.into_iter().map(|s| s.as_ref().to_owned()).collect();
        Self { path }
    }
}
