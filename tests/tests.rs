#[cfg(test)]
mod tests {
    use std::fs::{remove_dir_all, remove_file};
    use Conversion::{Cli, run};

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
        let result = run(Cli {
            to_xlsx: vec!["./test_res/java/module1/messages.properties".parse().unwrap(), "./test_res/java/module2/messages.properties".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
        let result = run(Cli {
            to_xlsx: vec!["./test_res/android/module1/values/strings.xml".parse().unwrap(), "./test_res/android/module2/values/strings.xml".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
        let result = run(Cli {
            to_xlsx: vec!["./test_res/ios/module1/Localizable.strings".parse().unwrap(), "./test_res/ios/module2/Localizable.strings".parse().unwrap()],
            from_xlsx: None,
            merge_xlsx: vec![],
            reference_path: None,
        });
        assert!(result.is_ok());
    }


    fn merge_xlsx() {
        let result = run(Cli {
            to_xlsx: vec![],
            from_xlsx: None,
            merge_xlsx: vec!["./to_android.xlsx".parse().unwrap(), "./to_ios.xlsx".parse().unwrap(), "./to_java.xlsx".parse().unwrap()],
            reference_path: None,
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }


    fn form_xlsx_android() {
        let result = run(Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./test_res/merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_android.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }


    fn form_xlsx_ios() {
        let result = run(Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./test_res/merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_ios.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    fn form_xlsx_java() {
        let result = run(Cli {
            to_xlsx: vec![],
            from_xlsx: Some("./test_res/merge_test.xlsx".parse().unwrap()),
            merge_xlsx: vec![],
            reference_path: Some("./to_java.xlsx".parse().unwrap()),
        });
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}