use plotly::{Bar, Layout, NamedColor, Plot};
use plotly::common::{Marker, Title};
use plotly::layout::Axis;

use crate::system::basic::strings;
use crate::system::defaults::{colors, file_extensions, output_filenames::ChannelOutputNames};
use crate::system::defaults::algorithm_params::NUMBER_OF_COLOR_VALUES;
use crate::system::data::composed::histogram_input::HistogramInput;
use crate::system::data::composed::histogram_output::HistogramOutput;

pub fn create_histograms(input_params: &HistogramInput, histogram_output: &HistogramOutput, output_filepath_prefix: &String) {
    let y_max = compute_maximum(input_params, histogram_output);

    if input_params.channels.red {
        create_and_save_histogram(&histogram_output.red_data, y_max, input_params.logarithmic, colors::RED,
                                  &strings::concat_with_static(output_filepath_prefix, ChannelOutputNames::RED));
    }

    if input_params.channels.green {
        create_and_save_histogram(&histogram_output.green_data, y_max, input_params.logarithmic, colors::GREEN,
                                  &strings::concat_with_static(output_filepath_prefix, ChannelOutputNames::GREEN));
    }

    if input_params.channels.blue {
        create_and_save_histogram(&histogram_output.blue_data, y_max, input_params.logarithmic, colors::BLUE,
                                  &strings::concat_with_static(output_filepath_prefix, ChannelOutputNames::BLUE));
    }

    if input_params.channels.alpha {
        create_and_save_histogram(&histogram_output.alpha_data, y_max, input_params.logarithmic, colors::VIOLET,
                                  &strings::concat_with_static(output_filepath_prefix, ChannelOutputNames::ALPHA));
    }

    if input_params.channels.luminance {
        create_and_save_histogram(&histogram_output.luminance_data, y_max, input_params.logarithmic, colors::ORANGE,
                                  &strings::concat_with_static(output_filepath_prefix, ChannelOutputNames::LUMINANCE));
    }
}

fn compute_maximum(input_params: &HistogramInput, histogram_output: &HistogramOutput) -> f64 {
    let maximum_function = |max, &val| if val > max { val } else { max };

    let maximum_red = histogram_output.red_data.iter().fold(0.0f64, maximum_function);
    let maximum_green = histogram_output.green_data.iter().fold(0.0f64, maximum_function);
    let maximum_blue = histogram_output.blue_data.iter().fold(0.0f64, maximum_function);
    let maximum_alpha = histogram_output.alpha_data.iter().fold(0.0f64, maximum_function);
    let maximum_luminance = histogram_output.luminance_data.iter().fold(0.0f64, maximum_function);

    let mut maximum = 0.0;

    if input_params.channels.red {
        maximum = maximum_function(maximum, &maximum_red);
    }

    if input_params.channels.green {
        maximum = maximum_function(maximum, &maximum_green);
    }

    if input_params.channels.blue {
        maximum = maximum_function(maximum, &maximum_blue);
    }

    if input_params.channels.alpha {
        maximum = maximum_function(maximum, &maximum_alpha);
    }

    if input_params.channels.luminance {
        maximum = maximum_function(maximum, &maximum_luminance);
    }

    return maximum;
}

fn create_and_save_histogram(histogram_data: &[f64; NUMBER_OF_COLOR_VALUES], y_range_max: f64, logarithmic: bool, color: &str, output_filepath_prefix: &String) {
    let trace = Bar::new(
        (0..NUMBER_OF_COLOR_VALUES).collect(),
        histogram_data.to_vec(),
    )
        .name("Frequency of Pixel Values")
        .marker(Marker::new().color(color));

    let layout = create_layout(y_range_max, logarithmic);

    let mut plot: Plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    plot.to_html(strings::concat_with_static(&output_filepath_prefix, file_extensions::HISTOGRAMS));
}

fn create_layout(y_range_max: f64, logarithmic: bool) -> Layout {
    let x_axis = Axis::new()
        .title(Title::new("Pixel Value"))
        .line_color(NamedColor::LightGray);

    let y_axis_title = if logarithmic { Title::new("Logarithm of Frequency") } else { Title::new("Frequency") };

    let y_axis = Axis::new()
        .title(y_axis_title)
        .line_color(NamedColor::White)
        .grid_color(NamedColor::White)
        .range(vec![0.0, y_range_max])
        .auto_range(false);

    return Layout::new()
        .xaxis(x_axis)
        .yaxis(y_axis)
        .plot_background_color(colors::GRAY);
}
