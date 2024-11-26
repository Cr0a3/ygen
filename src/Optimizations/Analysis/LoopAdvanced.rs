use crate::{ydbg, IR::{ir, Function}};

/// Advanced informations about a loop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedLoopInfo {
    actual: super::LoopInfo,
    form: AdvancedLoopForm,
}

impl AdvancedLoopInfo {
    /// Returns the loop form
    pub fn get_form(&self) -> &AdvancedLoopForm {
        &self.form
    }

    /// Returns the loop it wraps
    pub fn get_loop(&self) -> &super::LoopInfo {
        &self.actual
    }

    /// Returns if the loop is a for loop
    pub fn is_for(&self) -> bool {
        matches!(&self.form, AdvancedLoopForm::For)
    }

    /// Returns if the loop is a whil loop
    pub fn is_while(&self) -> bool {
        matches!(&self.form, AdvancedLoopForm::While)
    }
}

/// The loop form (like `for` or `while`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdvancedLoopForm {
    /// ```
    /// for (init; cond; action) { 
    ///     body 
    /// }
    /// ```
    For,
    /// ```
    /// while (cond) { 
    ///     body 
    /// }
    /// ```
    While,
    /// ```
    /// do {
    ///   body
    /// } while (cond);
    /// ```
    DoWhile,
}

/// ## AdvancedLoopAnalysis
/// Analyzes high level information out of a loop
/// This includes:
///   - form (e.g: for)
///   - constant counter (used for loop unrolling)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedLoopAnalysis<'a> {
    loops: Vec<AdvancedLoopInfo>,
    livness: super::LivenessAnalysis,
    func: &'a Function,
}

impl<'a> AdvancedLoopAnalysis<'a> {
    /// Creates the analysis 
    pub fn new(func: &'a Function) -> Self {
        Self {
            livness: super::LivenessAnalysis::analyze(&func),
            loops: Vec::new(),
            func: func,
        }
    }

    /// Analyzes inner loop information
    pub fn analyze(&mut self) {
        let mut loops = super::LoopAnalysis::new(&self.func);
        loops.analyze();


        for li in loops.loops() {
            self.analyze_loop(li);
        }
    }

    fn analyze_loop(&mut self, li: super::LoopInfo) {
        // Quickly think of a few common loop forms.
        // my brain tells me about:
        //  - for 
        //  - while
        // Where a for loop is like this:
        // for (int; cond; action) { body }
        // and a while loop is like this:
        // while (cond) { body }
        // So in order to check the loop form, we just need to check for the 
        // action, init 
        
        let cond = li.condition();

        let br_cond = if let Some(br_cond) = cond.as_any().downcast_ref::<ir::BrCond>() {
            br_cond 
        } else {
            ydbg!("[AdvancedLoopAnalysis] Found `while true` loop starting in block {}", li.condition_block().name);
            self.loops.push(AdvancedLoopInfo {
                actual: li,
                form: AdvancedLoopForm::While, // ahm actually 🤓 it's a `while (true)` loop
            });


            return;
        };

        let cond = br_cond.getCondition();
        let maker = self.livness.maker(&cond);

        let loop_inputs = maker.inputs();
        let mut used_in_loop = Vec::new();

        for block in li.body() {
            if block == li.condition_block() { continue; }

            let block = self.func.get_block_for(block).unwrap();

            for node in &block.nodes {
                    for input in &loop_inputs {
                        if node.inputs().contains(input) {
                            used_in_loop.push(input.to_owned());
                        }
                    }
            }
        }

        ydbg!("[AdvancedLoopAnalysis] recursively used in loop: {used_in_loop:?}");

        if used_in_loop.is_empty() {
            ydbg!("[AdvancedLoopAnalysis] found while loop");
            self.loops.push(AdvancedLoopInfo { 
                actual: li, 
                form: AdvancedLoopForm::While 
            });
            return;
        }

        ydbg!("[AdvancedLoopAnalysis] found for loop");
        self.loops.push(AdvancedLoopInfo { 
            actual: li, 
            form: AdvancedLoopForm::For 
        });
    }
}