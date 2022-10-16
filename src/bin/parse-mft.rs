
use mft;


fn main() {
    println!("[>] Starting...");

    let mftpath = "c:\\progs\\dev\\rust_regtest\\sample\\syscache\\mft_raw.export";

    let _parser = mft::MftParser::from_path(mftpath);

    //#println!("{:?}", parser.get_entry(1));
    println!("[.] Done.");
}
