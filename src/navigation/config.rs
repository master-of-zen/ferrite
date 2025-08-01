use super::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "name")]
    ByName,
    #[serde(rename = "modified")]
    ByModified,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::ByName
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    pub sort_order:    SortOrder,
    pub reverse_order: bool,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            sort_order: SortOrder::default(), reverse_order: false
        }
    }
}

impl NavigationConfig {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn with_sort_order(mut self, order: SortOrder) -> Self {
        self.sort_order = order;
        self
    }

    pub fn with_reverse(mut self, reverse: bool) -> Self {
        self.reverse_order = reverse;
        self
    }
}
