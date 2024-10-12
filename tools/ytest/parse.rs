#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub cmd: Vec<String>,
    pub input: Option<String>,
    pub input2: Option<String>,
    pub expected_out: Option<String>,
    pub expected_stderr: Option<String>,
    pub expected_code: Option<i32>,
    pub ignore_fail: bool,
}

pub fn parse(input: String) -> Parsed { 
    let mut out = Parsed {
        cmd: Vec::new(),
        input: None,
        input2: None,
        expected_out: None,
        expected_code: None,
        expected_stderr: None,
        ignore_fail: false,
    };
    
    let mut append ;

    let mut stdout = false;
    let mut stderr = false;
    let mut run = false;

    let mut input2 = false;

    for line in input.lines() {
        append = true;

        if line.trim().starts_with("# RUN:") {
            append = false;
            stdout = false;
            stderr = false;
            run = true;
            input2 = false;
        }
        
        if line.trim().starts_with("# STDOUT:") {
            append = false;
            stdout = true;
            stderr = false;
            run = false;
            input2 = false;
        }

        if line.trim().starts_with("# STDERR:") {
            append = false;
            stdout = true;
            stderr = true;
            run = false;
            input2 = false;
        }

        if line.trim().starts_with("# IN2:") {
            append = false;
            stdout = false;
            stderr = false;
            run = false;
            input2 = true;
        }

        if line.trim().starts_with("# IN:") {
            append = false;
            stdout = false;
            stderr = false;
            run = false;
            input2 = false;
        }

        if line.trim().starts_with("# EXPECT_FAIL") {
            append= false;
            run = false;
            stdout = false;
            stderr = false;
            
            out.ignore_fail = true;
        }

        if line.trim().starts_with("# EXIT_CODE=") {
            let line = line.trim().replace("# EXIT_CODE=", "");
            out.expected_code = Some(str::parse::<i32>(&line).unwrap());
            append = false;
        }

        let line = line.replace("    ", "\t");

        if append {
            if run {
                if !line.is_empty() {
                    out.cmd.push(format!("{}\n", line.trim()));
                }
            } else if stdout {
                if let Some(expected) = &mut out.expected_out {
                    expected.push_str(&format!("{line}\n"));
                } else {
                    out.expected_out = Some(format!("{line}\n"));
                }
            } else if stderr {
                if let Some(expected) = &mut out.expected_stderr {
                    expected.push_str(&format!("{line}\n"));
                } else {
                    out.expected_stderr = Some(format!("{line}\n"));
                }
            } else {
                if input2 {
                    if let Some(input) = &mut out.input2 {
                        input.push_str(&format!("{line}\n"));
                    } else {
                        out.input2 = Some(format!("{line}\n"));
                    }
                } else {
                    if let Some(input) = &mut out.input {
                        input.push_str(&format!("{line}\n"));
                    } else {
                        out.input = Some(format!("{line}\n"));
                    }
                }
            }
        }
    }

    if let Some(out) = &mut out.expected_out {
        *out = out.chars().filter(|x| !x.is_whitespace()).collect::<String>();
    }

    out
}