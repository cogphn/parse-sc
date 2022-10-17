
use mft;


fn main() {
    println!("[>] Starting...");

    let mftpath = "c:\\progs\\dev\\rust_regtest\\sample\\syscache\\mft_raw.export";

    let mut _parser = mft::MftParser::from_path(mftpath).unwrap();

    let _x = _parser.get_entry(14340);

    let _p = _parser.get_full_path_for_entry(&_x.unwrap());
    println!("{:?}", _p);

    //#println!("{:?}", parser.get_entry(1));
    println!("[.] Done.");
}
