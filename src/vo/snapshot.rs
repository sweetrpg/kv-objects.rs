/**
 * Snapshot value object.
 * @paulyhedral
 */
use serde::{Deserialize, Serialize};
use sweetrpg_model_core::vo::auditable::*;
use sweetrpg_model_core::vo::tag::*;


/// Snapshot value object.
/// This value object is a serializable representation of the Snapshot model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotVO {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub notes: String,
    pub tags: Vec<TagVO>,
    pub auditable_vo: AuditableVO,
}

// ----------------------------------------------------------------------

#[cfg(test)]
mod tests {}
