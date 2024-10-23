use pcap_parser::*;
use pcap_parser::traits::PcapReaderIterator;
use std::fs::File;

/// Read a pcapng file and prints out it contents.
/// 
/// Returns error or number of blocks read.
pub fn read_pcapng_file(fname: &str) -> Result<i32, String> {
    let mut num_blocks = 0;
    let file = match File::open(fname) {
        Ok(f) => f,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    let mut reader = match PcapNGReader::new(65536, file) {
        Ok(r) => r,
        Err(e) => {
            return Err(e.to_string());
        },
    };

    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    PcapBlockOwned::Legacy(legacy)    => {
                        println!("{:04x}: [PKT]\n{:?}\n", offset, legacy);
                    },
                    PcapBlockOwned::LegacyHeader(header) => {
                        println!("{:04x}: [HDR]\n{:?}\n", offset, header);
                    },
                    PcapBlockOwned::NG(packet) => {
                        println!("{:04x}: [NG]\n{:?}\n", offset, packet);
                    },
                };
                num_blocks += 1;
                reader.consume(offset);
            },
            Err(PcapError::Eof) => break,
            Err(PcapError::Incomplete(_)) => {
                reader.refill().unwrap();
            },
            Err(e) => panic!("error while reading: {:?}", e),
        }
    }

    Ok(num_blocks)
}
