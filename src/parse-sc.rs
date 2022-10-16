use notatin::{
    err::Error,
    //cell_key_node::CellKeyNode,
    parser_builder::{ ParserBuilder },
    //parser::{Parser, ParserIterator}
    parser::{ParserIterator}
};


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

    let _base_hive = "/home/examiner/dev/reg_sc/samples/06_Syscache.hve";
    let _logfile1 = "/home/examiner/dev/reg_sc/samples/06_Syscache.hve.LOG1";
    let _logfile2 = "/home/examiner/dev/reg_sc/samples/06_Syscache.hve.LOG2";

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
                let program_id = key.get_value("AeFileID");
                let object_id = key.get_value("_ObjectId_");
                let file_id = key.get_value("_FileId_");
                let usn = key.get_value("_Usn_");
                let usn_journal_id = key.get_value("_UsnJournalId_");
                let ae_program_id = key.get_value("AeProgramID");

                
                
                //some defaults
                let mut sha1:std::string::String = "--none--".to_string();
                let mut mftentryno:i32 = 0;
                //let mut usnjrnlid = 0;

                //sha1
                if program_id != None{
                    let val = program_id.unwrap().detail;
                    let val_bytes = val.value_bytes().unwrap();   
                    sha1  = get_value_str(val_bytes);
                }
                if file_id != None{
                    let val = file_id.unwrap().detail;
                    let val_bytes = val.value_bytes().unwrap();
                    mftentryno = get_value_i32(val_bytes);
                }

                println!("{},{}", mftentryno,sha1);



                
                
                




            } 
        }
        
        
    }
    
    
    println!("[.] Done.");
    Ok(())
}

