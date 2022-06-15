use wasm_bindgen::prelude::*;
use crate::uuid;
use crate::Cells;
use crate::float;
use crate::ResourceKindId;
use std::collections::HashMap;
use crate::ResourceKinds;
use crate::Resources;
use crate::Machine;


#[wasm_bindgen]
pub struct Universe {
    pub(crate) machines: Vec<Machine>,
    pub(crate) machines_stores: Vec<float>,
    pub(crate) active_machines: HashMap<uuid, usize>,
    pub(crate) inactive_machines: HashMap<uuid, usize>,
    pub(crate) next_uuid: uuid,
    pub(crate) resources: Resources,
    pub(crate) active_resources: HashMap<uuid, usize>,
    pub(crate) inactive_resources: HashMap<uuid, usize>,
    pub(crate) active_resources_by_kind: HashMap<ResourceKindId, HashMap<uuid, usize> >,
    pub(crate) available_resources_by_kind: HashMap<ResourceKindId, HashMap<uuid, usize> >,
    pub(crate) resource_kinds: ResourceKinds,
    pub(crate) resource_kinds_by_text_id: HashMap<String, ResourceKindId>,
    pub(crate) base_diameter: float,
    pub CELLS_COUNT_BY_SIDE: usize,
    #[allow(non_snake_case_types)]
    pub CELLS_COUNT: usize,
    pub(crate) cells: Cells,
    pub step: usize,
    pub consumption_rate: float,
    pub replicate_threshold: float,
}
