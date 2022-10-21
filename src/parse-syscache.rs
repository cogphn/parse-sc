use std::path::Path;
//use csv::Writer;
use argparse::{ArgumentParser, Store};

//use std::fs;

use notatin::{
    err::Error,
    //cell_key_node::CellKeyNode,
    cli_util::parse_paths,
    parser_builder::{ ParserBuilder },
    //parser::{Parser, ParserIterator}
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
    let mut parser = pb.build()?;

    let mut iter = ParserIterator::new(&parser);
    
    for (_index, key) in iter.iter().enumerate() {

        let path = key.get_pretty_path();
        if path.contains("ObjectTable"){
            if !path.contains("Indexes") { //this is wrong
                let keylastmod = key.last_key_written_date_and_time();
                
                //todo: fix indexes modtime
                //let pb1 = ParserBuilder::from_path(format!("{}",path)
                //let indexes_key = key.get_sub_key_by_path(&mut parser,"Indexes"); //todo: figure out
                //let indexesmod = indexes_key.last_key_written_date_and_time();


                let ae_file_id = key.get_value("AeFileID"); //sha1
                //let object_id = key.get_value("_ObjectId_");
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
                println!("{},{},{},{}", keylastmod, mftentryno,entry_filepath,sha1);

            } 
        }
        
        
    }
    
    
    println!("[.] Done.");
    Ok(())
}

