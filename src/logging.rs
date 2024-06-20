use fern::Dispatch;
use log::LevelFilter;
use chrono::Local;
use owo_colors::OwoColorize;

pub fn init_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(move |out, message, record| {
            let level = record.level();
            let level_color = match level {
                log::Level::Error => "ERROR".red().to_string(),
                log::Level::Warn => "WARN".yellow().to_string(),
                log::Level::Info => "INFO".green().to_string(),
                log::Level::Debug => "DEBUG".white().to_string(),
                log::Level::Trace => "TRACE".bright_black().to_string(),
            };

            let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string().cyan().to_string();
            let module = record.module_path().unwrap_or("unknown").blue().to_string();
            let func = record.file().unwrap_or("unknown").purple().to_string();
            let line = record.line().unwrap_or(0).to_string().magenta().to_string();
            let message = message.to_string().bright_white().to_string();

            out.finish(format_args!(
                "{date} | {module}: > {func}: > {line} | {level} | {message}\x1B[0m",
                date = date,
                module = module,
                func = func,
                line = line,
                level = level_color,
                message = message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
