use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

pub trait Mapper<From, To>: Sync + Send
where
    From: Serialize,
    To: DeserializeOwned,
{
    fn map(&self, from: &From) -> Result<To> {
        let json = serde_json::to_string(from)?;
        Ok(serde_json::from_str(&json)?)
    }
}

pub fn default<From, To>() -> &'static dyn Mapper<From, To>
where
    From: Serialize,
    To: DeserializeOwned,
{
    &DEFAULT_MAPPER
}

static DEFAULT_MAPPER: DefaultMapper = DefaultMapper {};

struct DefaultMapper {}

impl<From, To> Mapper<From, To> for DefaultMapper
where
    From: Serialize,
    To: DeserializeOwned,
{
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    struct From {
        id: Option<i32>,
        created_by: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    struct To {
        id: Option<i32>,
    }

    #[test]
    fn test_mapper() -> Result<()> {
        let mapper = default::<From, To>();
        let mapper2 = default::<To, From>();
        let from = From {
            id: Some(1),
            created_by: Some("test".to_string()),
        };
        let to = mapper.map(&from)?;
        assert_eq!(to.id, from.id);

        let from2 = mapper2.map(&to)?;
        assert_eq!(from2.id, from.id);
        assert_eq!(from2.created_by, None);
        Ok(())
    }
}
