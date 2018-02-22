use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::env::current_dir;

use umbrella::block::device::{BlockNumber, BlockDevice};

pub mod parse;
pub use self::parse::*;

/// The mutable state that backs a shell (environment variables, current directory, ...)
pub struct Env {
    current_dir: RefCell<PathBuf>
}

impl Env {
    pub fn new() -> Env {
        let dir = current_dir().expect("ERROR: Insufficient permissions to read master process current directory");
        Env {
            current_dir: RefCell::new(dir)
        }
    }

    pub fn current_dir(&self) -> PathBuf {
        self.current_dir.borrow().clone()
    }
}

pub fn cd(env: &Env, args: Args) {
    let parser = Parser::new(vec![Argument::string()]);
    args.parse_explain("cd", parser, |parsed| {
        let path = PathBuf::from(parsed.at(0).string());
        if path.is_dir() {
            let mut buf = env.current_dir.borrow_mut();
            if path.is_absolute() {
                *buf = path;
            } else {
                *buf = buf.join(path);
            }
        } else {
            eprintln!("ERROR: cd requires the argument to be a directory and to be accessible")
        }
    })
}

pub fn new_fs(_env: &Env, args: Args) {
    let parser = Parser::new(
        vec![ Argument::string()
            , Argument::nat()
            , Argument::Optional(Kind::Nat)
        ]
    );
    args.parse_explain("newfs", parser, |parsed| {
        let file_name = parsed.at(0).string();
        let block_count = parsed.at(1).nat();
        let block_size = parsed.optional(2).nat().map(|n| n as u16);
        match BlockDevice::create(&file_name, block_count, block_size) {
            Ok(_) => {}
            Err(err) => eprintln!("{}", err)
        }
    })
}

pub fn mount(_env: &Env, args: Args) {
    let parser = Parser::new(vec![Argument::string()]);
    args.parse_explain("mount", parser, |parsed| {
        let file_name = parsed.at(0).string();
        if ! Path::new(&file_name).exists() {
            eprintln!("The device: {0} does not exist. Try running 'newfs {0} 128' first.", file_name);
            return
        }
        match BlockDevice::open(&file_name) {
            Ok(_device) => {
                println!("open")
            }
            Err(err) => eprintln!("{}", err)
        }
    })
}

pub fn block_map(_env: &Env, _args: Args) {
    eprintln!("unimplemented");
}

pub fn alloc_block(_env: &Env, _args: Args) {
    eprintln!("unimplemented");
}

pub fn free_block(_env: &Env, args: Args) {
    let parser = Parser::new(vec![Argument::nat()]);
    args.parse_explain("free_block", parser, |parsed| {
        let _block_number = BlockNumber::new(parsed.at(0).nat());
        eprintln!("unimplemented");
    })
}

pub fn inode_map(_env: &Env, _args: Args) {
    eprintln!("unimplemented");
}

pub fn alloc_inode(_env: &Env, args: Args) {
    let parser = Parser::new(vec![Argument::string()]);
    args.parse_explain("alloc_inode", parser, |parsed| {
        let _inode_type = parsed.at(0).string();
        eprintln!("unimplemented");
    })
}

pub fn free_inode(_env: &Env, args: Args) {
    let parser = Parser::new(vec![Argument::nat()]);
    args.parse_explain("free_inode", parser, |parsed| {
        let _block_number = BlockNumber::new(parsed.at(0).nat());
        eprintln!("unimplemented");
    })

}

pub fn unmount(_env: &Env, _args: Args) {
    eprintln!("unimplemented");
}


