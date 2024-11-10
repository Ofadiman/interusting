#[derive(Debug)]
#[allow(dead_code)]
struct Task {
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    #[allow(dead_code)]
    fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        return Task {
            title: title.into(),
            description: description.into(),
            is_done: false,
        };
    }
}

/// Implementing `Default` trait might be useful for getting default struct from `Option` enum.
/// ```
/// let maybe_task: Option<Task> = None;
/// let task = maybe_task.unwrap_or_default();
/// ```
///
/// Another useful thing from implementing `Default` trait is the ability to generate default
/// values using spread operator.
/// ```
/// let task = Task {
///     is_done: true,
///     ..Default::default()
/// };
/// ```
impl Default for Task {
    fn default() -> Self {
        Task {
            title: String::from("No title."),
            description: String::from("No description."),
            is_done: false,
        }
    }
}

#[derive(Default)]
struct TaskBuilder {
    title: Option<String>,
    description: Option<String>,
    is_done: Option<bool>,
}

impl TaskBuilder {
    fn new() -> Self {
        TaskBuilder::default()
    }

    fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    fn is_done(&mut self, is_done: impl Into<bool>) -> &mut Self {
        self.is_done = Some(is_done.into());
        self
    }

    /// This method does not move the value of `task_builder`, so it can be used to construct
    /// multiple objects.
    /// ```
    /// let first_task = task_builder.build(); // `task_builder` is still valid after this call.
    ///
    /// task_builder.title("New title for 2nd task.");
    /// let second_task = task_builder.build();
    /// ```
    fn build(&self) -> Task {
        let default: Task = Default::default();

        return Task {
            title: self.title.clone().unwrap_or(default.title),
            description: self.description.clone().unwrap_or(default.description),
            is_done: self.is_done.unwrap_or(default.is_done),
        };
    }

    /// This method moves the value of `task_builder`, so it can be only used once.
    /// ```
    /// let first_task = task_builder.build_and_move(); // `task_builder` is no longer valid after this call because it has been moved by `build_and_move` call.
    /// ```
    fn build_and_move(self) -> Task {
        let default: Task = Default::default();

        return Task {
            title: self.title.clone().unwrap_or(default.title),
            description: self.description.clone().unwrap_or(default.description),
            is_done: self.is_done.unwrap_or(default.is_done),
        };
    }
}

fn main() {
    let mut task_builder = TaskBuilder::new();

    task_builder.title("Do the laundry.");
    task_builder.is_done(true);
    let first_task = task_builder.build();
    println!("{first_task:#?}");

    task_builder.title("Clean up your room.");
    let second_task = task_builder.build_and_move();
    println!("{second_task:#?}");
}
