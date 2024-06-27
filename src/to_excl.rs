use std::{env, fs};
use std::collections::HashSet;
use std::error::Error;
use std::fs::{File, remove_file};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use rust_xlsxwriter::{RowNum, Workbook, Worksheet};

use serde_xml_rs::from_reader;


use crate::{Cli, Platform};
use crate::bean::{Resource, XmlBean};

pub fn to(cli: Cli) -> Result<(), Box<dyn Error>> {
    let platform = cli.platform.expect("platform is null");

    return match platform {
        Platform::Android => android(),
        Platform::Ios => ios(),
        Platform::Java => java(),
    };
}

fn java() -> Result<(), Box<dyn Error>> {
    todo!()
}

fn ios() -> Result<(), Box<dyn Error>> {
    todo!()
}

fn android() -> Result<(), Box<dyn Error>> {
    let result = env::current_dir();
    let file_paths = find_files(result.unwrap().as_path(), move |path| {
        return path.file_name().unwrap() == "strings.xml" && path.parent().unwrap().file_name().unwrap() == "values";
    });
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30).ok();
    let mut row = 0;
    // 写入表头
    worksheet.write_string(row, 0, "Key").expect("Failed to write header");
    worksheet.write_string(row, 1, "Value").expect("Failed to write header");
    row = row + 1;
    for i in &file_paths {
        // 解析 XML 文件
        let resource: Resource = from_reader(BufReader::new(File::open(i.as_path())?)).expect("Failed to parse XML");
        let len = to_excel(resource.strings, worksheet, row);
        row = (len + row) as RowNum
    }
    remove_file("to_android.xlsx").ok();
    workbook.save("to_android.xlsx")?;
    Ok(())
}


///循环遍历,找出满足条件的文件
fn find_files(dir: &Path, check: fn(&PathBuf) -> bool) -> HashSet<PathBuf> {
    let mut result = HashSet::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.extend(find_files(&path, check));
            } else if path.is_file() && check(&path) {
                result.insert(path);
            }
        }
    }
    result
}

fn to_excel(beans: Vec<XmlBean>, sheet: &mut Worksheet, row: RowNum) -> u32 {
    // 写入数据
    let len = beans.len() as u32;
    for (i, entry) in beans.iter().enumerate() {
        let key = &entry.key;
        let value = entry.value.as_deref().unwrap_or("");
        sheet.write_string(i as u32 + row, 0, key).expect("Failed to write key");
        sheet.write_string(i as u32 + row, 1, value).expect("Failed to write value");
    }
    return len;
}




