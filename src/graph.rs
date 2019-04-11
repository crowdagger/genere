// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::errors::Result;

use std::collections::HashMap;
use std::collections::HashSet;

use error_chain::bail;

struct Graph {
    graph: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<()> {
        let mut entry = self.graph.entry(from.to_string()).or_insert(vec![]);
        {
            let to = to.to_string();
            if !entry.contains(&to) {
                entry.push(to);
            } else {
                return Ok(());
            }
        }
        // Check that there is no reverse dependency from to to from
        // Not optimal but f... it
        if self.has_indirect_dependency(to, from) {
            bail!("Trying to add a dependency from {from} to {to} but {to} depends on {from} already", from=from, to=to);
        }
        Ok(())    
    }
    
    fn has_indirect_dependency(&self, from: &str, to: &str) -> bool {
        if let Some(v) = self.graph.get(from) {
            for new_to in v {
                if new_to == to {
                    return true;
                } else {
                    if self.has_indirect_dependency(from, new_to) {
                        return true;
                    }
                }
            }
        }
        false
    }
}


#[test]
fn graph() {
    let mut graph = Graph::new();
    graph.add_dependency("a", "b").unwrap();
    graph.add_dependency("b", "c").unwrap();
    let res = graph.add_dependency("c", "a");
    assert!(res.is_err());
}
