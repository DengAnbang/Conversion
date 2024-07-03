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


#[cfg(test)]
mod tests {
    use std::fs::{remove_dir_all, remove_file};
    use super::*;

    #[test]
    fn test_all() {
        to_xlsx();
        merge_xlsx();
        form_xlsx_android();
        form_xlsx_ios();
        form_xlsx_java();
        remove_dir_all("./java_file").ok();
        remove_dir_all("./android_file").ok();
        remove_dir_all("./ios_file").ok();
        remove_file("to_android.xlsx").ok();
        remove_file("to_ios.xlsx").ok();
        remove_file("to_java.xlsx").ok();
    }



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


    fn form_xlsx_android() {
        let result = run(crate::Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_android.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }


    fn form_xlsx_ios() {
        let result = run(crate::Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_ios.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    fn form_xlsx_java() {
        let result = run(crate::Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_java.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}


