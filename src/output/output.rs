use task::r#type::Task;

use crate::text::r#type::Text;
use crate::time::time::get_now_time_format;
use crate::*;
use std::borrow::Cow;

/// Output
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Parameters
/// - `Output`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// ```
///
/// ## Using the Constructor
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// ```
pub fn output(output: Output) {
    // Text
    let text: &str = output.text;
    let color: ColorType = output.color.clone();
    let bg_color: ColorType = output.bg_color.clone();
    let blod: bool = output.blod.clone();
    // endl
    let endl: bool = output.endl;
    let mut output: String = String::new();
    let mut task_list: Task<'_> = Task::new();
    // Add text
    task_list.add(Text {
        text,
        color,
        bg_color,
        blod: blod,
        endl,
    });
    // run
    task_list.run_all();
}
