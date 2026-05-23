use anyhow::ensure;

pub struct Username(String);

impl Username {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Username {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();

        ensure!(!value.is_empty(), "username must not be empty");

        Ok(Self(value.to_owned()))
    }
}
