use std::fmt::Display;
use std::str::FromStr;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {

    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(i32).range(1..=8))]
    multi: i32,

    // --size 10x10
    #[arg(short, long, default_value_t = Size2(3,3), value_parser = clap::value_parser!(Size2))]
    size: Size2,
}
impl Options {
    pub fn from_args() -> Self {
        Self::parse()
    }
    pub fn width(&self) -> usize {
        self.size.0
    }
    pub fn height(&self) -> usize {
        self.size.1
    }
    pub fn samples(&self) -> i32 {
        self.multi
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size2(usize, usize);
impl Display for Size2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.0, self.1))
    }
}
impl FromStr for Size2 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<_> = s.split("x").collect();

        if vals.len() != 2 {
            Err("invalid Size2 format".to_string())
        } else {
            Ok(Self(
                usize::from_str(vals[0]).map_err(|e| format!("{e}"))?,
                usize::from_str(vals[1]).map_err(|e| format!("{e}"))?,
            ))
        }
    }
}
