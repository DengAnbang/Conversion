use std::error::Error;
use std::fs::{File, remove_file};
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rust_xlsxwriter::{RowNum, Workbook, Worksheet};
use serde_xml_rs::from_reader;

use crate::bean::{Platform, Resource, XmlBean};

pub fn to_xlsx(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
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
    let platform = Platform::new_suffix(suffix.unwrap().to_string());
    return if platform == Platform::new_android() {
        android(file_paths)
    } else if platform == Platform::new_ios() {
        ios(file_paths)
    } else if platform == Platform::new_java() {
        java(file_paths)
    } else {
        panic!("暂不支持的文件格式!");
    };
}

fn java(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let platform = Platform::new_java();
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30).ok();
    let mut row = 0;
    // 写入表头
    worksheet.write_string(row, 0, platform.key).expect("Failed to write header");
    worksheet.write_string(row, 1, "def_value").expect("Failed to write header");
    row = row + 1;
    for file_path in file_paths {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
                continue; // Skip empty lines and comments
            }

            if let Some((key, value)) = trimmed_line.split_once('=') {
                let key = key.trim().trim_matches('"').to_string();
                let value = value.trim().trim_matches(';').trim_matches('"').to_string();
                worksheet.write_string(row, 0, &key).expect("Failed to write key");
                worksheet.write_string(row, 1, &value).expect("Failed to write value");
                row = row + 1;
            }
        }
    }
    remove_file(&platform.file_name).ok();
    workbook.save(&platform.file_name)?;
    Ok(())
}

fn ios(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let platform = Platform::new_ios();
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30).ok();
    let mut row = 0;
    // 写入表头
    worksheet.write_string(row, 0, platform.key).expect("Failed to write header");
    worksheet.write_string(row, 1, "def_value").expect("Failed to write header");

    row = row + 1;
    for file_path in file_paths {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        let mut key = String::new();
        let mut value = String::new();
        let mut in_value = false;

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
                continue; // Skip empty lines and comments
            }

            if in_value {
                if trimmed_line.ends_with(';') {
                    value.push_str(&trimmed_line[..trimmed_line.len() - 1]);
                    worksheet.write_string(row, 0, &key).expect("Failed to write key");
                    worksheet.write_string(row, 1, &value.trim_matches('"').to_string()).expect("Failed to write value");
                    row = row + 1;
                    key.clear();
                    value.clear();
                    in_value = false;
                } else {
                    value.push_str(trimmed_line);
                }
            } else if let Some((k, v)) = trimmed_line.split_once('=') {
                key = k.trim().trim_matches('"').to_string();
                if v.ends_with(';') {
                    value = v.trim().trim_matches(';').trim_matches('"').to_string();

                    worksheet.write_string(row, 0, &key).expect("Failed to write key");
                    worksheet.write_string(row, 1, &value).expect("Failed to write value");
                    row = row + 1;
                    key.clear();
                    value.clear();
                } else {
                    value = v.trim().trim_matches('"').to_string();
                    in_value = true;
                }
            }
        }
    }
    remove_file(&platform.file_name).ok();
    workbook.save(&platform.file_name)?;
    Ok(())
}

fn android(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    let platform = Platform::new_android();
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 30).ok();
    let mut row = 0;
    // 写入表头
    worksheet.write_string(row, 0, platform.key).expect("Failed to write header");
    worksheet.write_string(row, 1, "def_value").expect("Failed to write header");
    row = row + 1;
    for i in file_paths {
        // 解析 XML 文件
        let resource: Resource = from_reader(BufReader::new(File::open(i)?)).expect("Failed to parse XML");
        let len = to_excel(resource.strings, worksheet, row);
        row = (len + row) as RowNum
    }
    remove_file(&platform.file_name).ok();
    workbook.save(&platform.file_name)?;
    Ok(())
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



