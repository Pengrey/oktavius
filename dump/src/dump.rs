use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::Debug::ReadProcessMemory,
        Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_PRIVATE},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

// RAII wrapper for the process handle
struct ProcessHandle(HANDLE);

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

pub fn iterate_over_mem(pid: u32, extractors: &[fn(&str, bool)], verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Get a handle to the process with VM READ and QUERY permissions
        let process_handle = ProcessHandle(match OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid) {
            Ok(handle) => handle,
            Err(e) => {
                if verbose {
                    println!("[-] Failed to open process: {}", e);
                }
                return Err(e.into());
            }
        });

        if process_handle.0.is_invalid() {
            if verbose {
                println!("[-] Invalid process handle");
            }
            return Ok(());
        }

        let mut address: usize = 0;
        let mut mem_info: MEMORY_BASIC_INFORMATION = zeroed();

        // Iterate over memory regions
        while VirtualQueryEx(process_handle.0, Some(address as *const c_void), &mut mem_info, size_of::<MEMORY_BASIC_INFORMATION>()) != 0 {
            // Check if the memory is committed and of a private type
            if mem_info.State == MEM_COMMIT && mem_info.Type == MEM_PRIVATE {
                let mut buffer: Vec<u8> = vec![0; mem_info.RegionSize];
                let mut bytes_read: usize = 0;

                // Read the memory region into the buffer
                let result = ReadProcessMemory(process_handle.0, mem_info.BaseAddress, buffer.as_mut_ptr() as *mut c_void, mem_info.RegionSize, Some(&mut bytes_read as *mut _));

                match result {
                    Ok(_) => {
                        if bytes_read > 0 {
                            let text = String::from_utf8_lossy(&buffer[..bytes_read]);
                            for extractor_fn in extractors {
                                extractor_fn(&text, verbose);
                            }
                        }
                    }
                    Err(_e) => {}
                }
            }
            // Move to the next memory region
            address = mem_info.BaseAddress as usize + mem_info.RegionSize;
        }
    }

    Ok(())
 }
