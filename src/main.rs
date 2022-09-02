use clap::Parser;
use expanded_pathbuf::ExpandedPathBuf;
use lava_torrent::torrent::v1::{Torrent, TorrentBuilder};

#[derive(Parser)]
enum Command {
    Make {
        path: ExpandedPathBuf,
        out: ExpandedPathBuf,
    },
    Dump {
        path: ExpandedPathBuf,
    },
    Infohash {
        path: ExpandedPathBuf,
    },
}

fn main() {
    let cmd = Command::parse();

    match cmd {
        Command::Make { path, out } => {
            let torrent = TorrentBuilder::new(&path, 2_i64.pow(18)).build().unwrap();

            torrent.write_into_file(&out).unwrap();
        }
        Command::Dump { path } => {
            let torrent = Torrent::read_from_file(path).unwrap();

            println!("{torrent:?}");
        }
        Command::Infohash { path } => {
            let torrent = Torrent::read_from_file(path).unwrap();

            println!("{:?}", torrent.info_hash());
        }
    }
}
