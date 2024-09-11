#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub cmd: Vec<String>,
    pub input: String,
    pub expected_out: String,
    pub expected_code: i32,
}

pub fn parse(input: String) -> Parsed { 
    let mut out = Parsed {
        cmd: Vec::new(),
        input: String::new(),
        expected_out: String::new(),
        expected_code: 0,
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

        if line.trim().starts_with("# EXIT_CODE=") {
            let line = line.trim().replace("# EXIT_CODE=", "");
            out.expected_code = str::parse::<i32>(&line).unwrap();
            append = false;
        }

        let line = line.replace("    ", "\t");

        if append {
            if run {
                if !line.is_empty() {
                    out.cmd.push(format!("{}\n", line.trim()));
                }
            } else if stdout {
                out.expected_out.push_str(&format!("{line}\n"));
            } else {
                out.input.push_str(&format!("{line}\n"));
            }
        }
    }

    out.expected_out = out.expected_out.chars().filter(|x| !x.is_whitespace()).collect::<String>();

    //out.expected_out = unescaper::unescape(&out.expected_out).unwrap();

    out
}