#[derive(Copy, Clone)]
pub enum Priority {
  Info,
  Debug,
  Warning,
  Error,
}

impl Priority {
  pub fn as_str(&self) -> &'static str {
    match *self {
      Priority::Info => "INFO",
      Priority::Debug => "DEBUG",
      Priority::Warning => "WARNING",
      Priority::Error => "ERROR",
    }
  }
}

pub struct Logger {
  pub default_tag : &'static str,
  pub default_priority: Priority,
}

impl Logger {
  pub fn new() -> Logger {
    Logger {
      default_tag: "no-tag",
      default_priority: Priority::Debug,
    }
  }

  pub fn log(&self, message: &str) {
    println!("{:>8} | {} : {}", self.default_tag, self.default_priority.as_str(), message);
  }

  pub fn log_with_tag(&self, message: &str, tag: &str) {
    println!("{:>8} | {} : {}", tag, self.default_priority.as_str(), message);
  }

  pub fn log_with_priority(&self, message: &str, priority: &str) {
    println!("{:>8} | {} : {}", self.default_tag, priority, message);
  }

  pub fn log_with_tag_and_priority(&self, message: &str, tag: &str, priority: &str) {
    println!("{:>8} | {} : {}", tag, priority, message);
  }
}
