#[tauri::command]
pub fn multiplication99() -> String {
    let mut result = String::new();

    for i in 1..=9 {
        for j in 1..=i {
            let product = i * j;
            // 格式化每个算式，使用全角空格对齐中文排版
            let s = format!("{}×{}={}\t", j, i, product);
            result.push_str(&s);
        }
        result.push('\r'); // 每行结束后换行
        result.push('\n'); // 每行结束后换行
    }
    println!("{:?}", result);
    result
}
