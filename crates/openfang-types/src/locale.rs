//! Shared locale normalization helpers for UI, API, and CLI layers.

/// Public language code for English.
pub const LANGUAGE_EN: &str = "en";
/// Public language code for Simplified Chinese.
pub const LANGUAGE_ZH_CN: &str = "zh-CN";

/// Normalize a locale or language hint to a supported public language code.
pub fn normalize_language(input: &str) -> &'static str {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return LANGUAGE_EN;
    }

    let lowered = trimmed.replace('_', "-").to_ascii_lowercase();
    if lowered == "en" || lowered.starts_with("en-") {
        return LANGUAGE_EN;
    }
    if lowered == "zh"
        || lowered.starts_with("zh-")
        || lowered.starts_with("zhcn")
        || lowered.starts_with("zh-hans")
    {
        return LANGUAGE_ZH_CN;
    }
    LANGUAGE_EN
}

/// Resolve a final language from a persisted config value and an optional detected locale.
pub fn resolve_language(config_language: &str, detected_locale: Option<&str>) -> &'static str {
    if !config_language.trim().is_empty() {
        return normalize_language(config_language);
    }
    detected_locale
        .map(normalize_language)
        .unwrap_or(LANGUAGE_EN)
}

/// Detect the system locale from conventional environment variables.
pub fn detect_system_locale() -> Option<String> {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|key| std::env::var(key).ok())
        .filter(|value| !value.trim().is_empty())
        .map(|value| value.split('.').next().unwrap_or("").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_supported_languages() {
        assert_eq!(normalize_language("en"), LANGUAGE_EN);
        assert_eq!(normalize_language("en-US"), LANGUAGE_EN);
        assert_eq!(normalize_language("zh"), LANGUAGE_ZH_CN);
        assert_eq!(normalize_language("zh_CN"), LANGUAGE_ZH_CN);
        assert_eq!(normalize_language("zh-Hans"), LANGUAGE_ZH_CN);
        assert_eq!(normalize_language("zh-CN"), LANGUAGE_ZH_CN);
    }

    #[test]
    fn normalize_unknown_language_falls_back_to_english() {
        assert_eq!(normalize_language("de"), LANGUAGE_EN);
        assert_eq!(normalize_language(""), LANGUAGE_EN);
    }

    #[test]
    fn resolve_prefers_explicit_config() {
        assert_eq!(resolve_language("zh", Some("en-US")), LANGUAGE_ZH_CN);
        assert_eq!(resolve_language("en", Some("zh-CN")), LANGUAGE_EN);
    }

    #[test]
    fn resolve_uses_detected_locale_when_config_missing() {
        assert_eq!(resolve_language("", Some("zh-CN")), LANGUAGE_ZH_CN);
        assert_eq!(resolve_language("", Some("fr-FR")), LANGUAGE_EN);
        assert_eq!(resolve_language("", None), LANGUAGE_EN);
    }
}
