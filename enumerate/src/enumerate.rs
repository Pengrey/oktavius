use sysinfo::{System, Pid};
use regex::Regex;
use std::ffi::OsStr;

pub fn get_program_pids(program_name: &str, command_line_regex: Option<&str>) -> Vec<Pid> {
    let regex_filter = if let Some(pattern) = command_line_regex {
        match Regex::new(pattern) {
            Ok(re) => Some(re),
            Err(_) => return Vec::new(),
        }
    } else {
        None
    };

    let mut system = System::new_all();
    system.refresh_all();

    let mut pids: Vec<Pid> = Vec::new();

    for process in system.processes_by_name(OsStr::new(program_name)) {
        if let Some(ref re) = regex_filter {
            let full_command = process.cmd().join(OsStr::new(" "));

            if let Some(cmd_str) = full_command.to_str() {
                if re.is_match(cmd_str) {
                    pids.push(process.pid());
                }
            }
        } else {
            pids.push(process.pid());
        }
    }

    pids
}
