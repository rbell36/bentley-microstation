//! Windows registry read helpers for Bentley Microstation.

pub const APP_KEY: &str = "Software\\Bentley\\Bentley Microstation";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Bentley Microstation";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Bentley Microstation") },
        RegValue { name: "Version",     value: String::from("v3.3.19") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Bentley Microstation") },
        RegValue { name: "Publisher",       value: String::from("Bentley") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Bentley Microstation\\uninstall.exe\"") },
    ]
}
