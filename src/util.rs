use crate::error::Result;
use fern::{
  self,
  colors::{
    Color,
    ColoredLevelConfig,
  }
};

pub fn setup_logger(verbosity: u64) -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::BrightGreen)
        .debug(Color::BrightCyan)
        .trace(Color::BrightMagenta);

    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity {
        0 => base_config
            .level(log::LevelFilter::Info),
        1 => base_config
            .level(log::LevelFilter::Debug),
        _2_or_more => base_config
            .level(log::LevelFilter::Trace),
    };

    base_config
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stderr())
        .apply()?;

    Ok(())
}
