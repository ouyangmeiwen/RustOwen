pub struct StringUtils;

impl StringUtils {
    /// 判断字符串是否为空或仅包含空白字符
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// 转换为大写
    pub fn to_uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    /// 转换为小写
    pub fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    /// 首字母大写，其余小写
    pub fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first
                .to_uppercase()
                .chain(chars.flat_map(|c| c.to_lowercase()))
                .collect(),
            None => String::new(),
        }
    }

    /// 去除字符串两端空格
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    /// 替换字符串中的子串
    pub fn replace(s: &str, from: &str, to: &str) -> String {
        s.replace(from, to)
    }

    /// 检查字符串是否包含某个子串
    pub fn contains(s: &str, substr: &str) -> bool {
        s.contains(substr)
    }

    /// 检查字符串是否以指定前缀开头
    pub fn starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    /// 检查字符串是否以指定后缀结尾
    pub fn ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    /// 获取字符串的长度
    pub fn length(s: &str) -> usize {
        s.len()
    }

    /// 检查字符串是否为数字
    pub fn is_numeric(s: &str) -> bool {
        s.chars().all(|c| c.is_numeric())
    }

    /// 移除多余空格，将连续空格合并为单个空格
    pub fn remove_extra_spaces(s: &str) -> String {
        s.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    /// 检查是否以大写字母开头
    pub fn starts_with_uppercase(s: &str) -> bool {
        s.chars().next().map_or(false, |c| c.is_uppercase())
    }

    /// 检查是否以小写字母开头
    pub fn starts_with_lowercase(s: &str) -> bool {
        s.chars().next().map_or(false, |c| c.is_lowercase())
    }

    /// 检查字符串是否为回文
    pub fn is_palindrome(s: &str) -> bool {
        let filtered: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        let lowercased = filtered.to_lowercase();
        lowercased == lowercased.chars().rev().collect::<String>()
    }

    /// 字符串反转
    pub fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// 按索引切片（支持索引超出范围）
    pub fn slice(s: &str, start: usize, end: usize) -> String {
        s.chars()
            .skip(start)
            .take(end.saturating_sub(start))
            .collect()
    }

    /// 重复字符串指定次数
    pub fn repeat(s: &str, times: usize) -> String {
        s.repeat(times)
    }

    /// 统计字符出现次数
    pub fn count_char(s: &str, c: char) -> usize {
        s.chars().filter(|&ch| ch == c).count()
    }

    /// 转换为 kebab-case（短横线分隔）
    pub fn to_kebab_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>()
            .join("-")
    }

    /// 转换为 snake_case（下划线分隔）
    pub fn to_snake_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>()
            .join("_")
    }
    /// 按字符分隔字符串，返回 `Vec<String>`
    pub fn split_by_char(s: &str, delimiter: char) -> Vec<String> {
        s.split(delimiter).map(|part| part.to_string()).collect()
    }

    /// 按字符串分隔字符串，返回 `Vec<String>`
    pub fn split_by_str(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|part| part.to_string()).collect()
    }

    /// 按空白字符分隔字符串，去掉多余空白
    pub fn split_whitespace(s: &str) -> Vec<String> {
        s.split_whitespace().map(|part| part.to_string()).collect()
    }

    /// 按分隔符拆分为固定数量的部分（多余部分合并到最后一个）
    pub fn split_n(s: &str, delimiter: char, n: usize) -> Vec<String> {
        let mut parts: Vec<String> = s
            .splitn(n, delimiter)
            .map(|part| part.to_string())
            .collect();
        if n > 0 && parts.len() == n {
            // 合并多余部分
            let remaining = parts.pop().unwrap_or_default();
            parts.push(remaining);
        }
        parts
    }

    /// 按分隔符拆分字符串并过滤空值
    pub fn split_and_filter_empty(s: &str, delimiter: char) -> Vec<String> {
        s.split(delimiter)
            .filter(|part| !part.trim().is_empty())
            .map(|part| part.to_string())
            .collect()
    }
}
