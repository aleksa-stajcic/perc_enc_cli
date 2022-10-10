use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    #[arg(short = 'c', long = "character")]
    character: char,
    // #[arg(short='c', long = "character")]
    // character: Option<char>,
    // #[arg(short='e', long = "encoded")]
    // encoded: Option<String>,
}

fn main() {
    let map = HashMap::<char, String>::from([
        (' ', String::from("%20")),
        ('!', String::from("%21")),
        ('#', String::from("%23")),
        ('$', String::from("%24")),
        ('%', String::from("%25")),
        ('&', String::from("%26")),
        ('\'', String::from("%27")),
        ('(', String::from("%28")),
        (')', String::from("%29")),
        ('*', String::from("%2A")),
        ('+', String::from("%2B")),
        (',', String::from("%2C")),
        ('/', String::from("%2F")),
        (':', String::from("%3A")),
        (';', String::from("%3B")),
        ('=', String::from("%3D")),
        ('?', String::from("%3F")),
        ('@', String::from("%40")),
        ('[', String::from("%5B")),
        (']', String::from("%5D")),
    ]);

    let args = Args::parse();

    let ch = args.character;
    match map.get(&ch) {
        Some(e) => {
            println!("`{}` => `{}`", ch, e);
        }
        None => {
            panic!("Character `{}` doesn't get encoded.", ch)
        }
    }

    // &todo: handle not having entries in map
    // match args.character {
    //     Some(c) => {
    //         match map.get(&c) {
    //             Some(e) => { println!("`{}` => `{}`", c, e); },
    //             None => { panic!("Character `{}` doesn't get encoded.", c) } // &todo figure this shit out
    //         }
    //     },
    //     None => { } // todo: can this be removed and just have an `if let x = Some()`?
    // }

    // match args.encoded {
    //     Some(e) => {
    //         for (k, v) in &map {
    //             if v == &e {
    //                 println!("`{}` => `{}`", k, v);
    //                 break;
    //             }
    //         }
    //     },
    //     None => { } // todo: can this be removed and just have an `if let x = Some()`?
    // }
}
