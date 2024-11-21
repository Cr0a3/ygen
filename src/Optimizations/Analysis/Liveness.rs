use std::collections::HashMap;

use ir::Ir;
use crate::{Optimizations::Analysis::CfgAnalysis::CFGAnalysis, IR::*};

/// Analysis the livness of variables
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivenessAnalysis {
    users: HashMap<Var, Vec<Box<dyn Ir>>>,
    last_uses: HashMap<Var, Option<BlockId>>,
}

impl LivenessAnalysis {
    /// Analyzes the entire function
    pub fn analayze(func: &Function) -> Self {
        let mut analyzer = Self {
            users: HashMap::new(),
            last_uses: HashMap::new(),
        };

        for block in &func.blocks {
            analyzer.analyze_block(block);
        }

        analyzer.analyze_last_uses(func);

        analyzer
    }

    /// Analyzes the given block
    pub fn analyze_block(&mut self, block: &Block) {
        for node in &block.nodes {
            for user in &node.inputs() {
                if let Some(users) = self.users.get_mut(&user) {
                    users.push(node.clone_box());
                } else {
                    // the variable was not yet defined **which does not** mean it
                    // doesn't exists because maybe the definition is in a unalysed block
                    self.users.insert(user.to_owned(), vec![node.clone_box()]);
                }
            }

            if let Some(output) = node.output() {
                if !self.users.contains_key(&output) {
                    self.users.insert(output, Vec::new());
                }
            }
        }
    }

    /// Analyzes in which ir node the variable is used latest
    pub fn analyze_last_uses(&mut self, _func: &Function) {
        // TODO
    }

    /// Returns all ir nodes which uses the ir variable
    pub fn users(&self, var: &Var) -> &Vec<Box<dyn Ir>> {
        let Some(users) = self.users.get(var) else {
            panic!("in the inner analaysis there is no variable called: {}", var.name);
        };

        users
    }

    /// Returns if the given variable is dead
    /// So either it has no users or
    /// all users are also dead
    pub fn is_dead(&self, var: &Var) -> bool {
        let users = self.users(var);

        if users.len() == 0 { return true; }

        let mut all_users_dead = false;

        for user in users {
            if all_users_dead {
                break
            }

            if let Some(out) = user.output() {
                all_users_dead = self.is_dead(&out);
            }
        }

        all_users_dead
    }
}