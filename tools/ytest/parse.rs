#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub cmd: String,
    pub input: String,
    pub expected_out: String
}

pub fn parse(input: String) -> Parsed { 
    let mut out = Parsed {
        cmd: String::new(),
        input: String::new(),
        expected_out: String::new(),
    };
    
    let mut append ;

    for line in input.lines() {
        append = true;

        if line.trim_start().starts_with("# RUN:") {
            out.cmd += &line.replace("# RUN:", "");
            append = false;
        }
        
        if line.trim_start().starts_with("# STDOUT:") {
            out.expected_out += &line.replace("# STDOUT:", "");
            append = false;
        }

        if append {
            out.input.push_str(&format!("{}\n", line));
        }
    }

    out.cmd = out.cmd.trim_start().to_string();

    out
}