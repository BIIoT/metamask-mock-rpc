pub fn str_utc_time_now() -> String {
    let now = chrono::Utc::now();
    let str_time = now.naive_utc().to_string();
    let mut s = str_time.split(".");
    s.next().unwrap().to_string()
}