use std::error::Error;

use clap::Parser;

use crate::form_xlsx::form_xlsx;
use crate::merge_xlsx::merge_xlsx;
use crate::to_xlsx::to_xlsx;

mod to_xlsx;
mod bean;
mod merge_xlsx;
mod form_xlsx;


#[derive(Parser, Debug)]
#[command(name = "Conversion")]
#[command(author = "jiqimao")]
#[command(version)]
#[command(about = "一个帮助Android,ios,java 国际化文件和xlsx之间的转换工具", long_about = None)]
pub struct Cli {
    ///需要转换成xlsx的文件路径,可以是多个文件
    #[arg(value_enum, short, long, ignore_case = true)]
    pub to_xlsx: Vec<String>,

    ///将xlsx的文件还原成对应平台的数据,输入xlsx的文件的路径
    #[arg(value_enum, short, long, requires = "reference_path", ignore_case = true)]
    pub from_xlsx: Option<String>,

    ///需要合并的xlsx的文件路径
    #[arg(value_enum, short, long, ignore_case = true)]
    pub merge_xlsx: Vec<String>,

    ///需要参考的文件路径
    #[arg(short, long, requires = "from_xlsx", ignore_case = true)]
    pub reference_path: Option<String>,

}


pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.to_xlsx.len() > 0 {
        return to_xlsx(cli.to_xlsx);
    }
    if cli.merge_xlsx.len() > 0 {
        return merge_xlsx(cli.merge_xlsx);
    }
    if cli.from_xlsx != None && cli.reference_path != None {
        return form_xlsx(cli.from_xlsx.unwrap(), cli.reference_path.unwrap());
    }

    Ok(())
}





