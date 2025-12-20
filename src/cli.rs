use clap::{Parser, Subcommand};

use crate::util::Rgb;

/// What to do with the selected container.
#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Command {
    /// Focus the container
    #[command(about = "Focus the selected window (default)")]
    Focus,

    /// Swap current window with selected window
    #[command(about = "Swap focused window with the selected window")]
    Swap {
        /// Also focus the selected window after swapping
        #[arg(long)]
        focus: bool,
    },

    /// Print the container's ID
    #[command(about = "Print the selected window's ID")]
    Print,
}

/// A tool to help efficiently focus windows in Sway, inspired by i3-easyfocus.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The list of characters to use for hints
    #[arg(long, default_value = "fjghdkslaemuvitywoqpcbnxz")]
    pub chars: String,

    /// The window background color <rrggbb>
    #[arg(long, default_value = "1d1f21")]
    pub window_background_color: Rgb,

    /// The window background opacity <0-1.0>
    #[arg(long, default_value_t = 0.2)]
    pub window_background_opacity: f64,

    /// The label background color <rrggbb>
    #[arg(long, default_value = "1d1f21")]
    pub label_background_color: Rgb,

    /// The label background opacity <0-1.0>
    #[arg(long, default_value_t = 1.0)]
    pub label_background_opacity: f64,

    /// The label text color <rrggbb>
    #[arg(long, default_value = "c5c8c6")]
    pub label_text_color: Rgb,

    /// The label background color <rrggbb>
    #[arg(long, default_value = "285577")]
    pub focused_background_color: Rgb,

    /// The focused background opacity <0-1.0>
    #[arg(long, default_value_t = 1.0)]
    pub focused_background_opacity: f64,

    /// The focused text color <rrggbb>
    #[arg(long, default_value = "ffffff")]
    pub focused_text_color: Rgb,

    /// The font family
    #[arg(long, default_value = "monospace")]
    pub font_family: String,

    /// The font weight
    #[arg(long, default_value = "bold")]
    pub font_weight: String,

    /// The font size, see: https://www.w3.org/TR/css-fonts-3/#font-size-prop
    #[arg(long, default_value = "medium")]
    pub font_size: String,

    /// The label padding-x <px>
    #[arg(long, default_value_t = 4)]
    pub label_padding_x: i32,

    /// The label padding-y <px>
    #[arg(long, default_value_t = 0)]
    pub label_padding_y: i32,

    /// The label margin-x <px>
    #[arg(long, default_value_t = 4)]
    pub label_margin_x: i32,

    /// The label margin-y <px>
    #[arg(long, default_value_t = 2)]
    pub label_margin_y: i32,

    /// Show confirmation window after selection
    #[arg(long = "show-confirmation", default_value_t = false)]
    pub show_confirmation: bool,

    /// The selected command [default: Focus]
    #[command(subcommand)]
    pub command: Option<Command>,
}
