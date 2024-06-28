use std::error::Error;

use clap::{Parser, ValueEnum};
use clap::builder::Str;
use serde::Deserialize;

use crate::to_excl::to;

mod to_excl;
mod bean;


#[derive(Parser, Debug)]
#[command(name = "Conversion")]
#[command(author = "jiqimao")]
#[command(version)]
#[command(about = "哈哈哈", long_about = None)]
pub struct Cli {
    ///需要转换成xlsx的文件路径,可以是多个文件
    #[arg(value_enum, short, long, ignore_case = true)]
    to_xlsx: Vec<String>,

    ///将xlsx的文件还原成对应平台的数据,输入xlsx的文件的路径
    #[arg(value_enum, short, long, requires = "reference_path", ignore_case = true)]
    from_xlsx: Option<String>,

    ///需要合并的xlsx的文件路径
    #[arg(value_enum, short, long, ignore_case = true)]
    merge_xlsx: Vec<String>,

    ///需要参考的文件路径
    #[arg(short, long, requires = "from_xlsx", ignore_case = true)]
    reference_path: Option<String>,

}


pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.to_xlsx.len() > 0 {
        return to(cli.to_xlsx);
    }


    // return match cli.mode {
    //     Mode::To => to(cli),
    //     Mode::From => from(cli),
    //     Mode::Merge => merge(cli),
    // };
    Ok(())
}

fn from(cli: Cli) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn merge(cli: Cli) -> Result<(), Box<dyn Error>> {
    Ok(())
}


