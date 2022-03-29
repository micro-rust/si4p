//! The time report struct contains an abstraction to track the time some tasks took to execute.



use std::time::{ Duration, Instant };



pub struct TimeReport {
    /// Name of the task.
    name: String,

    /// The time it took to complete this task.
    time: Duration,

    /// Additional contents of the time report.
    subtask: Vec<TimeReport>,
}

impl TimeReport {
    /// Creates a new `TimeReport` with the given name.
    #[inline]
    pub fn new(name: String) -> TimeReport {
        TimeReport { name, time: Duration::new(0, 0), subtask: Vec::new() }
    }

    /// Starts the time report and returns the instant it begun.
    #[inline]
    pub fn start(&mut self) -> Instant {
        Instant::now()
    }

    /// Ends the time report.
    #[inline]
    pub fn end(&mut self, start: Instant) {
        self.time = Instant::now() - start;
    }

    /// Adds a subtask to the time report.
    pub fn add(&mut self, task: TimeReport) {
        self.subtask.push(task);
    }

    /// Creates a subtask with the given suffix.
    pub fn subtask(&mut self, sub: &str) -> TimeReport {
        TimeReport {
            name: format!("{}-{}", self.name, sub),
            time: Duration::new(0, 0),
            subtask: Vec::new(),
        }
    }

    /// Pretty print of the time report.
    fn display(&self, level: usize) -> String {
        // Build the indentation level.
        let indent = Self::indent(level);

        let mut string = format!("{}Task: {}\n{}Time: {}\n", indent.0, self.name, indent.1, self.timestr());

        for task in self.subtask.iter() {
            string += &task.display(level + 1)
        }

        string
    }

    /// Generates the time string.
    fn timestr(&self) -> String {
        if self.time.as_micros() < 1000 {
            return format!("{:3.2} us", self.time.as_nanos() as f64 / 1000.0);
        }

        if self.time.as_millis() < 1000 {
            return format!("{:3.2} ms", self.time.as_micros() as f64 / 1000.0);
        }

        format!("{:3.2} s", self.time.as_millis() as f64 / 1000.0)
    }

    /// Generates the indentation level string.
    fn indent(level: usize) -> (String, String) {
        match level {
            0 => (String::new(), String::new()),
            _ => {
                let mut dot = String::new();
                let mut clr = String::new();

                for _ in 0..level {
                    dot.push_str("  |");
                    clr.push_str("  |");
                }

                dot.push_str("-");
                clr.push_str(" ");

                (dot, clr)
            },
        }
    }
}

impl core::fmt::Display for TimeReport {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.display(0))
    }
}