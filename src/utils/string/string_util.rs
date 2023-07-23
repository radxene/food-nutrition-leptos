pub struct StringUtil;

impl StringUtil {
    pub fn capitalize(value: &str) -> String {
        format!(
            "{}{}",
            value.chars().next().unwrap().to_uppercase(),
            value.chars().skip(1).collect::<String>()
        )
    }
}
