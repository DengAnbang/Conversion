use std::error::Error;

use calamine::Reader;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use crate::merge_xlsx::merge_xlsx;

use crate::to_excl::to;

mod to_excl;
mod bean;
mod merge_xlsx;


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
    if cli.merge_xlsx.len() > 0 {
        return merge_xlsx(cli.merge_xlsx);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_xlsx() {
        let result = run(crate::Cli {
            to_xlsx: vec!["./java/module1/messages.properties".parse().unwrap(), "./java/module2/messages.properties".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
        let result = run(crate::Cli {
            to_xlsx: vec!["./android/module1/values/strings.xml".parse().unwrap(), "./android/module2/values/strings.xml".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
        let result = run(crate::Cli {
            to_xlsx: vec!["./ios/module1/Localizable.strings".parse().unwrap(), "./ios/module2/Localizable.strings".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn merge_xlsx() {
        let result = run(crate::Cli {
            to_xlsx: vec![],
            from_xlsx: None,
            merge_xlsx: vec!["./to_android.xlsx".parse().unwrap(), "./to_ios.xlsx".parse().unwrap(), "./to_java.xlsx".parse().unwrap()],
            reference_path: None,
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}


