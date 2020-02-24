use std::collections::HashMap;

use crate::value;

type WordDef = String;

pub struct Context<'a> {
    pub dict: &'a mut HashMap<String, WordDef>,
    pub stack: &'a mut Vec<value::Value>,
}
