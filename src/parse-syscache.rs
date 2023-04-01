extern crate csv;

use argparse::{ArgumentParser, Store};
use csv::Writer;

use notatin::{
    err::Error,
    cli_util::parse_paths,
    parser_builder::{ ParserBuilder },
    parser::{ParserIterator}
};
use mft;



fn get_value_str(bytes:Vec<u8>) -> std::string::String  {
    let val = std::str::from_utf8(&bytes).unwrap().replace("\u{0}","");
    return val;
}

fn get_value_i32(bytes:Vec<u8>) -> i32 {
    let val = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
    return val;
}


fn main() -> Result<(), Error> {
    println!("[>] Starting...");



    let mut hive_path = "Syscache.hve".to_string();
    let mut mft_path = "MFT.raw".to_string();
    let mut output_path = "syscache_aprsed.csv".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("syscache parser");
        ap.refer(&mut output_path)
            .add_option(&["-o", "--output"], Store, "output file name")
            .required();
        ap.refer(&mut hive_path)
            .add_option(&["-h","--hive"], Store, "full path to syscache.hve and logs (hivename,log1,log2)")
            .required();
        ap.refer(&mut mft_path)
            .add_option(&["-m","--mft"], Store, "full path to $MFT")
            .required();
        ap.parse_args_or_exit();
    }

    let _outfile_arg = format!("{output_path}");

    let mut mft_parser = mft::MftParser::from_path(mft_path).unwrap();
    
        
    let (_base_hive,logfiles) = parse_paths(&hive_path);

    let mut pb = ParserBuilder::from_path(_base_hive);

    for log in logfiles.unwrap_or_default(){
        pb.with_transaction_log(log);
    }
    let parser = pb.build()?;

    let mut iter = ParserIterator::new(&parser);
    

    let mut writer = match Writer::from_path(output_path) {
        Ok(w) => w,
        Err(e) => panic!("Cannot open output file. Error: {}",e)
    };

    let mut header = Vec::new();
    header.push("reglastmod".to_string());
    header.push("mftentryno".to_string());
    header.push("lookup".to_string());
    header.push("sha1".to_string());

    match writer.write_record(header) {
        Ok(x) => x,
        Err(e) => println!("[!] error writing data: {}", e)
    };

    for (_index, key) in iter.iter().enumerate() {

        let path = key.get_pretty_path();
        if path.contains("ObjectTable"){
            if !path.contains("Indexes") { //this is wrong
                let keylastmod = key.last_key_written_date_and_time();

                let ae_file_id = key.get_value("AeFileID"); //sha1

                let file_id = key.get_value("_FileId_"); //mft entry no
                //let usn = key.get_value("_Usn_");
                //let usn_journal_id = key.get_value("_UsnJournalId_");
                //let ae_program_id = key.get_value("AeProgramID");
                
                //some defaults
                let mut sha1:std::string::String = "--none--".to_string();
                let mut mftentryno:i32 = 0;
                let mut entry_filepath = "".to_string();

                //sha1
                if ae_file_id != None{
                    let val = ae_file_id.unwrap().detail;
                    let val_bytes = val.value_bytes().unwrap();   
                    sha1  = get_value_str(val_bytes)[4..44].to_string();
                }
                if file_id != None{
                    let val = file_id.unwrap().detail;
                    let val_bytes = val.value_bytes().unwrap();
                    mftentryno = get_value_i32(val_bytes);
                    let entryno_u16:u64 = mftentryno as u64;
                    let entry = mft_parser.get_entry(entryno_u16);
                    
                    let fp = mft_parser.get_full_path_for_entry(&entry.unwrap()).unwrap();

                    entry_filepath = match fp { 
                        Some(val) => val.as_path().display().to_string(),
                        None => "N/A".to_string()
                    };
                    
                }
                

                let mut row = Vec::new();
                row.push(keylastmod.to_string());
                row.push(mftentryno.to_string());
                row.push(entry_filepath);
                row.push(sha1);

                match writer.write_record(row) {
                    Ok(x) => x,
                    Err(e) => println!("[!] error writing data: {}", e)
                };

                //println!("{},{},{},{}", keylastmod, mftentryno,entry_filepath,sha1);

            } 
        }
        
        
    }

    

    
    
    println!("[.] Done.");
    Ok(())
}

