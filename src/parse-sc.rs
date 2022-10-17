use notatin::{
    err::Error,
    //cell_key_node::CellKeyNode,
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

    let _base_hive = "C:\\progs\\dev\\parse-sc\\samples\\syscache\\01_Syscache.hve";
    let _logfile1 = "C:\\progs\\dev\\parse-sc\\samples\\syscache\\01_Syscache.hve.LOG1";
    let _logfile2 = "C:\\progs\\dev\\parse-sc\\samples\\syscache\\01_Syscache.hve.LOG2";

    let _mftpath = "c:\\progs\\dev\\rust_regtest\\sample\\syscache\\mft_raw.export";

    let mut mft_parser = mft::MftParser::from_path(_mftpath).unwrap();

    let parser = ParserBuilder::from_path(_base_hive)
        .recover_deleted(false)
        .with_transaction_log(_logfile1)
        .with_transaction_log(_logfile2)
        .build()?;
    
    let mut iter = ParserIterator::new(&parser);
    
    for (index, key) in iter.iter().enumerate() {

        let path = key.get_pretty_path();
        if path.contains("ObjectTable"){
            if !path.contains("Indexes") {
                let keylastmod = key.last_key_written_date_and_time();
                
                //let indexes_key = key.get_sub_key_by_path(&mut parser,"Indexes"); //doesn't work
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
                //let mut usnjrnlid:i32 = -1;

                //sha1
                if ae_file_id != None{
                    let val = ae_file_id.unwrap().detail;
                    let val_bytes = val.value_bytes().unwrap();   
                    sha1  = get_value_str(val_bytes);
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

