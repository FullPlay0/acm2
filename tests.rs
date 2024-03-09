// В файле tests.rs

#[cfg(test)]
mod tests {
    #[test]
    fn test_windows_specific_function() {
        #[cfg(target_os = "windows")]
        {
            // Тестирование функции для Windows
        }
    }

    #[test]
    fn test_linux_specific_function() {
        #[cfg(target_os = "linux")]
        {
            // Тестирование функции для Linux
        }
    }
}
