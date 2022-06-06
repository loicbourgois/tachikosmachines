// use wasm_bindgen::prelude::*;
use crate::uuid;
use std::collections::HashMap;


// #[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Cell {
    pub active_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    pub available_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    pub active_machines: HashMap<uuid, usize>,
    pub c9s: Vec<usize>,
    pub cell_id: usize,
}
