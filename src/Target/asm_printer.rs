use super::instr::McInstr;

/// Prints assembly
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmPrinter {
    assembly: String,
    tabs: usize,

    // backend hooks
    pub(super) start_fn: Option<fn(&mut AsmPrinter)>,
    pub(super) const_pr: Option<fn(&mut AsmPrinter, &String, &crate::IR::Const)>,
}

impl AsmPrinter {
    /// Creates a new assembly printer
    pub fn new() -> Self {
        Self {
            assembly: String::new(),
            start_fn: None,
            const_pr: None,
            tabs: 0,
        }
    }

    /// Adds a tab
    pub fn add_tab(&mut self) {
        self.tabs += 1;
    }

    
    /// Removes a tab
    pub fn remove_tab(&mut self) {
        self.tabs -= 1;
    }

    /// adds a assembly line
    pub fn line<T: ToString>(&mut self, line: T) {
        for _ in 0..self.tabs {
            self.assembly.push_str("  ");
        }

        let line = line.to_string();
        self.assembly.push_str(line.as_str());
        self.assembly.push('\n');
    }

    /// "Starts up" the assembly printer
    /// (Can be used by some targets to add 
    /// comments or information on the top)
    pub fn start(&mut self) {
        if let Some(start_fn) = self.start_fn {
            start_fn(self);
        }
    }

    /// Adds a constant to the assembly
    pub fn add_const(&mut self, name: &String, constant: &crate::IR::Const) {
        let Some(constant_printer) = self.const_pr else {
            panic!("The target assembly printer doesn't currently support printing constants");
        };

        constant_printer(self, name, constant);
    }

    /// Begins function assembly printing
    pub fn begin_func(&mut self, name: &String) {
        self.line(format!("{name}:"));
        self.tabs += 1;
    }

    /// Ends function assembly printing
    pub fn end_func(&mut self) {
        self.tabs -= 1;
        self.line("");
    }

    /// Begins block printing
    pub fn begin_block(&mut self, name: &String, print: bool) {
        self.tabs += 1;

        if !print { return; }

        self.line(format!(".{name}:"));
    }

    /// Prints the instruction
    pub fn print_inst(&mut self, inst: &Box<dyn McInstr>) {
        self.line(inst.asm());
    }

    /// Ends block printing
    pub fn end_block(&mut self) {
        self.tabs -= 1;
    }

    /// Emits the generated assembly into a assembly string
    pub fn emit(&self) -> &String {
        &self.assembly
    }
}