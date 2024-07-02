use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use calamine::{DataType, open_workbook, Reader, Xlsx};
use crate::bean::{Platform, XmlBean};

pub fn form_xlsx(from_xlsx: String, reference_path: String) -> Result<(), Box<dyn Error>> {
    if !Path::new(&from_xlsx).exists() {
        return Err(Box::from(format!("File does not exist:{:?}", from_xlsx)));
    }
    if !Path::new(&reference_path).exists() {
        return Err(Box::from(format!("File does not exist:{:?}", reference_path)));
    }
    let mut excel: Xlsx<_> = open_workbook(reference_path.to_string())?;
    let mut platform: Platform = Platform::new_android();
    if let Ok(range) = excel.worksheet_range("Sheet1") {
        if let Some(data) = range.rows().next() {
            platform = Platform::new_key(data[0].to_string());
        }
    }
    let mut scores: Vec<HashMap<String, Option<String>>> = Vec::new();
    let mut excel: Xlsx<_> = open_workbook(from_xlsx.to_string())?;
    if let Ok(range) = excel.worksheet_range("Sheet1") {
        for (row, entry) in range.rows().enumerate() {
            //跳过第一列
            'outer: for (col, data) in entry.iter().skip(1).enumerate() {
                if row == 0 {
                    if let Some(_) = data.as_string() {
                        scores.push(HashMap::new())
                    } else {
                        break 'outer;
                    }
                }
                if scores.len() > col {
                    if let Some(s) = entry[platform.col_num as usize].as_string() {
                        scores[col].insert(s, data.as_string());
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }
    //获取参考文件的key的顺序
    let mut excel: Xlsx<_> = open_workbook(reference_path.to_string())?;
    let mut reference: Vec<String> = Vec::new();
    if let Ok(range) = excel.worksheet_range("Sheet1") {
        for key in range.rows().skip(1) {
            reference.push(key[0].to_string());
        }
    }
    let mut to: Vec<Vec<XmlBean>> = Vec::new();
    for score in 0..scores.len() {
        to.push(Vec::new());


        let option = &scores[score];
        to[score].push(XmlBean { key: platform.key.clone(), value: option.get(&platform.key).unwrap().clone() });
        for r in &reference {
            let x = option.get(r).unwrap();
            to[score].push(XmlBean { key: r.clone(), value: x.clone() })
        }
    }
    println!("{:?}", to);

    return Ok(());
}