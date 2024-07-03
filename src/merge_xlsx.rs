use std::collections::HashMap;
use std::error::Error;
use std::fs::remove_file;
use std::path::Path;

use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use crate::bean::Platform;

pub fn merge_xlsx(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    if file_paths.len() <= 0 {
        return Err(Box::from("输入的文件为空"));
    }
    for path in &file_paths {
        if !Path::new(&path).exists() {
            return Err(Box::from(format!("File does not exist:{:?}", path)));
        }
    }

    println!("合并中..");
    // 创建 XLSX 文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let mut row = 0;
    let mut platform: Platform = Platform::new_android();
    worksheet.write_string(row, 0, "待翻译内容").expect("Failed to write header");
    worksheet.set_column_width(0, 30).ok();
    worksheet.write_string(row, 1, "翻译好的内容(en)").expect("Failed to write header");
    worksheet.set_column_width(1, 30).ok();
    row += 1;
    let mut scores: HashMap<String, u32> = HashMap::new();
    for path in file_paths.iter() {
        // 打开 XLSX 文件
        let mut excel: Xlsx<_> = open_workbook(path.to_string())?;
        if let Ok(range) = excel.worksheet_range("Sheet1") {
            if range.is_empty() {
                break;
            }
            for (i, entry) in range.rows().enumerate() {
                if i == 0 {
                    let data = &entry[0];
                    //获得到这个文件的平台
                    platform = Platform::new_key(data.to_string());
                    worksheet.set_column_width(platform.col_num, 30).ok();
                    worksheet.write_string(0, platform.col_num, platform.key).expect("Failed to write header");
                } else {
                    let key = entry[0].to_string();
                    let value = entry[1].to_string();
                    if let Some(row_num) = scores.get(&key) {
                        worksheet.write_string(*row_num, platform.col_num, &key).expect("Failed to write header");
                    } else {
                        worksheet.write_string(row, platform.col_num, &key).expect("Failed to write header");
                        worksheet.write_string(row, 0, &value).expect("Failed to write header");
                        scores.insert(String::from(value), row);
                        row += 1;
                    }
                }
            }
        }
    }
    remove_file("../merge.xlsx").ok();
    workbook.save("../merge.xlsx")?;
    println!("合并成功,合并后的文件:{}", "../merge.xlsx");
    return Ok(());
}
