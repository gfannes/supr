#[derive(Default)]
pub struct Logger {
    level: i32,
}

impl Logger {
    pub fn new(level: i32) -> Logger {
        Logger { level }
    }

    pub fn update_level(&mut self, level: &Option<i32>) -> &Logger {
        if let Some(level) = level {
            self.level = std::cmp::max(self.level, *level);
        }
        self
    }

    pub fn log(&self, level: i32, cb: impl FnOnce() -> ()) {
        if self.level >= level {
            cb();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_new() -> util::Result<()> {
        let logger = Logger::new(10);

        let mut count = 0;

        logger.log(9, || count += 1);
        assert_eq!(count, 1);

        logger.log(10, || count += 1);
        assert_eq!(count, 2);

        logger.log(11, || count += 1);
        assert_eq!(count, 2);

        Ok(())
    }

    #[test]
    fn test_default() -> util::Result<()> {
        let logger_def = Logger::default();
        let logger_new = Logger::new(0);

        let mut count_def = 0;
        let mut count_new = 0;

        logger_def.log(-1, || count_def += 1);
        logger_new.log(-1, || count_new += 1);
        assert_eq!(count_def, count_new);

        logger_def.log(0, || count_def += 1);
        logger_new.log(0, || count_new += 1);
        assert_eq!(count_def, count_new);

        logger_def.log(1, || count_def += 1);
        logger_new.log(1, || count_new += 1);
        assert_eq!(count_def, count_new);

        Ok(())
    }
}
