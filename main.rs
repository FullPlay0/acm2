mod gui;
mod acm;

#[cfg(target_os = "windows")]
fn windows_specific_function() {
    // Код, специфичный для Windows
    println!("This is Windows-specific functionality.");
}

#[cfg(target_os = "linux")]
fn linux_specific_function() {
    // Код, специфичный для Linux
    println!("This is Linux-specific functionality.");
}

fn main() {
    if std::env::args().len() > 1 {
        let matches = clap::App::new("ACM CLI")
            .version("1.0.0")
            .author("Your Name <you@example.com>")
            .about("Implements arithmetic congruence monoids (ACM)")
            .subcommand(clap::SubCommand::with_name("calculate")
                .about("Calculates ACM"))
            .get_matches();

        // Пример использования переменной matches для условной логики
        if matches.is_present("calculate") {
            // Здесь должна быть ваша логика вызова функции calculate_acm
            println!("Calculate ACM functionality goes here.");
        }
    } else {
        // Импортируем необходимые модули в начало файла
        use crate::gui::GUI;
        use iced::Application;

        // Вызов функции sum_vector_parallel
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = acm::sum_vector_parallel(&numbers);
        println!("Sum of numbers: {}", sum);

        // Вызываем специфичную для операционной системы функцию
        #[cfg(target_os = "windows")]
        windows_specific_function();

        #[cfg(target_os = "linux")]
        linux_specific_function();

        // Запуск GUI приложения
        GUI::run(iced::Settings::default()).expect("An error occurred while starting the GUI");
    }
}
