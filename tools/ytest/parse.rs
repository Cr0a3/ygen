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

    let mut stdout = false;
    let mut run = false;

    for line in input.lines() {
        append = true;

        if line.trim().starts_with("# RUN:") {
            append = false;
            stdout = false;
            run = true;
        }
        
        if line.trim().starts_with("# STDOUT:") {
            append = false;
            stdout = true;
            run = false;
        }

        if line.trim().starts_with("# IN:") {
            append = false;
            stdout = false;
            run = false;
        }

        if append {
            if run {
                out.cmd.push_str(&format!("{line}\n"));
            } else if stdout {
                out.expected_out.push_str(&format!("{line}\n"));
            } else {
                out.input.push_str(&format!("{line}\n"));
            }
        }
    }

    out.cmd = out.cmd.trim_start().to_string();

    //out.expected_out = unescaper::unescape(&out.expected_out).unwrap();

    out
}