use core::panic::PanicInfo;
use cortex_m::asm;
use log::{Level, LevelFilter, Log, Metadata, Record};
use rtt_target::{rprintln, rtt_init_print};

static LOGGER: Logger = Logger;

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match (record.file(), record.line()) {
                (Some(file), Some(line)) => rprintln!(
                    "[ {} ] {}:{} — {}",
                    record.level(),
                    file,
                    line,
                    record.args()
                ),
                (_, _) => rprintln!("[ {} ] — {}", record.level(), record.args()),
            }
        }
    }

    fn flush(&self) {}
}

#[allow(dead_code)]
/// Setup the logger with the default (info) level.
pub fn init() {
    init_with_level(LevelFilter::Info);
}

#[allow(dead_code)]
/// Setup the logger with a specific level.
pub fn init_with_level(level: LevelFilter) {
    rtt_init_print!();
    rprintln!("Initialized rtt");

    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level);

    log::trace!("Initialized logger. Level = {}.", level);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{}", info);

    // trigger a hard fault to abort
    asm::udf()
}
