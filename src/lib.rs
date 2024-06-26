use std::error::Error;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "Conversion")]
#[command(author = "jiqimao")]
#[command(version)]
#[command(about = "哈哈哈", long_about = None)]
pub struct Cli {
    ///[in]生成xml,[out]通过xml生成对应的文件,[merge]合并xml
    #[arg(value_enum, short, long, ignore_case = true)]
    mode: Mode,
    #[arg(value_enum, short, long, ignore_case = true)]
    platform: Option<Platform>,
    ///需要转换成对应平台的文件路径
    #[arg(short, long, requires = "platform", ignore_case = true)]
    file_path: Option<String>,
    ///需要参考的文件路径
    #[arg(short, long, requires = "file_path", ignore_case = true)]
    reference_path: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    In,
    Out,
    Merge,
}

// 注意要配置以下衍生宏
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Platform {
    Ios,
    Android,
    Java,
}





pub fn run(config: Cli) -> Result<(), Box<dyn Error>> {

    // println!(" {name}");
    let current_dir = env!("CARGO_MANIFEST_DIR");
    println!("a: {:?}", current_dir);
    Ok(())
}