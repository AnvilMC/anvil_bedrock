use std::fmt::Display;

#[derive(Debug)]
pub struct MCPEPacketDataError {
    path: Vec<String>,
    field_name: String,
    error_type: String,
}

impl MCPEPacketDataError {
    pub fn new(field_name: impl Into<String>, error_type: impl Into<String>) -> Self {
        Self {
            path: vec![],
            field_name: field_name.into(),
            error_type: error_type.into(),
        }
    }

    pub fn map(mut self, field: impl Into<String>) -> Self {
        self.path.push(field.into());
        self
    }
}

impl Display for MCPEPacketDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{} : {}",
            self.path
                .iter()
                .rev()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .join("."),
            self.field_name,
            self.error_type
        )
    }
}
