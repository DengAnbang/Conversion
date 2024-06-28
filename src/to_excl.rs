use std::{env, fs};
use std::error::Error;
use std::fs::{File, remove_file};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use rust_xlsxwriter::{RowNum, Workbook, Worksheet};
use serde_xml_rs::from_reader;

use crate::bean::{Resource, XmlBean};

pub fn to(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let suffix = Path::new(&file_paths[0])
        .extension()
        .and_then(|s| s.to_str());
    for path in file_paths.iter().skip(1) {
        let current_extension = Path::new(path)
            .extension()
            .and_then(|s| s.to_str());

        if current_extension != suffix {
            return Err(Box::from("文件后缀不一致"));
        }
    }
    for path in &file_paths {
        if !Path::new(&path).exists() {
            return Err(Box::from(format!("File does not exist:{:?}", path)));
        }
    }
    return android(file_paths);
}

fn java() -> Result<(), Box<dyn Error>> {
    todo!()
}

fn ios() -> Result<(), Box<dyn Error>> {
    todo!()
}

fn android_auto() -> Result<(), Box<dyn Error>> {
    let result = env::current_dir();
    let file_paths = find_files(result.unwrap().as_path(), move |path| {
        return path.file_name().unwrap() == "strings.xml" && path.parent().unwrap().file_name().unwrap() == "values";
    });
    return android(file_paths);
}

fn android(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30).ok();
    let mut row = 0;
    // 写入表头
    worksheet.write_string(row, 0, "此列不用修改").expect("Failed to write header");
    worksheet.write_string(row, 1, "此列为待翻译").expect("Failed to write header");
    row = row + 1;
    for i in file_paths {
        // 解析 XML 文件
        let resource: Resource = from_reader(BufReader::new(File::open(i)?)).expect("Failed to parse XML");
        let len = to_excel(resource.strings, worksheet, row);
        row = (len + row) as RowNum
    }
    remove_file("to_android.xlsx").ok();
    workbook.save("to_android.xlsx")?;
    Ok(())
}


///循环遍历,找出满足条件的文件
fn find_files(dir: &Path, check: fn(&PathBuf) -> bool) -> Vec<String> {
    let mut result = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.extend(find_files(&path, check));
            } else if path.is_file() && check(&path) {
                result.push(path.to_str().unwrap().to_string());
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




