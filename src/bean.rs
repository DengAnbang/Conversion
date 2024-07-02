use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct XmlBean {
    #[serde(rename = "name")]
    pub key: String,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "string", default)]
    pub strings: Vec<XmlBean>,
}

#[derive(Debug, PartialEq)]
pub struct Platform {
    pub file_name: String,
    pub key: String,
    pub suffix: String,
    pub col_num: u16,

}

impl Platform {
    pub fn new_suffix(suffix: String) -> Platform {
        return match suffix.as_str() {
            "xml" => Self::new_android(),
            "strings" => Self::new_ios(),
            "properties" => Self::new_java(),
            _ => {
                panic!("暂不支持的文件格式!");
            }
        };
    }
    pub fn new_key(key: String) -> Platform {
        return match key.as_str() {
            "android_key" => Self::new_android(),
            "ios_key" => Self::new_ios(),
            "java_key" => Self::new_java(),
            _ => {
                panic!("暂不支持的格式!");
            }
        };
    }
    pub fn new_android() -> Platform {
        return Platform { file_name: String::from("to_android.xlsx"), suffix: String::from("xml"), key: String::from("android_key"), col_num: 26 };
    }
    pub fn new_ios() -> Platform {
        return Platform { file_name: String::from("to_ios.xlsx"), suffix: String::from("xml"), key: String::from("ios_key"), col_num: 27 };
    }
    pub fn new_java() -> Platform {
        return Platform { file_name: String::from("to_java.xlsx"), suffix: String::from("xml"), key: String::from("java_key"), col_num: 28 };
    }
}


// #[derive(Debug)]
// pub enum Platform {
//     Android {
//         file_name: String,
//         key: String,
//         suffix: String,
//         col_num: u32,
//     },
//     Ios {
//         file_name: String,
//         key: String,
//         suffix: String,
//         col_num: u32,
//     },
//     Java {
//         file_name: String,
//         key: String,
//         suffix: String,
//         col_num: u32,
//     },
// }
//
// impl crate::bean::Platform {
//     pub fn new(suffix: String) -> crate::bean::Platform {
//         return match suffix.as_str() {
//             "xml" => crate::bean::Platform::Android { file_name: String::from("to_android.xlsx"), suffix: String::from("xml"), key: String::from("android_key"), col_num: 28 },
//             "strings" => crate::bean::Platform::Ios { file_name: String::from("to_ios.xlsx"), suffix: String::from("xml"), key: String::from("ios_key"), col_num: 29 },
//             "properties" => crate::bean::Platform::Java { file_name: String::from("to_java.xlsx"), suffix: String::from("xml"), key: String::from("java_key"), col_num: 30 },
//             _ => {
//                 panic!("暂不支持的文件格式!");
//             }
//         };
//     }
// }
