use std::fs::File;
use std::io::Write;

use crate::system::basic::strings;
use crate::system::data::composed::statistics_input::StatisticsInput;
use crate::system::data::composed::statistics_output::StatisticsOutput;
use crate::system::data::elementary::channels::Channel;
use crate::system::defaults::algorithm_params::NUMBER_OF_INPUT_CHANNELS;
use crate::system::defaults::file_extensions;
use crate::system::defaults::messages::errors;
use crate::system::defaults::output_filenames::ChannelOutputNames;

pub fn create_statistics(input_params: &StatisticsInput, statistics_output: &[StatisticsOutput; NUMBER_OF_INPUT_CHANNELS], output_filepath_prefix: &String) {
    if input_params.channels.red {
        create_and_save_statistics(&statistics_output[Channel::RED as usize],
                                   output_filepath_prefix, ChannelOutputNames::RED);
    }

    if input_params.channels.green {
        create_and_save_statistics(&statistics_output[Channel::GREEN as usize],
                                   output_filepath_prefix, ChannelOutputNames::GREEN);
    }

    if input_params.channels.blue {
        create_and_save_statistics(&statistics_output[Channel::BLUE as usize],
                                   output_filepath_prefix, ChannelOutputNames::BLUE);
    }

    if input_params.channels.alpha {
        create_and_save_statistics(&statistics_output[Channel::RED as usize], output_filepath_prefix,
                                   ChannelOutputNames::ALPHA);
    }

    if input_params.channels.luminance {
        create_and_save_statistics(&statistics_output[Channel::RED as usize],
                                   output_filepath_prefix, ChannelOutputNames::LUMINANCE);
    }
}

fn create_and_save_statistics(statistics_output: &StatisticsOutput, output_filepath_prefix: &String, channel_output_name: &'static str) {
    let output_filename = strings::concat_with_static(output_filepath_prefix, channel_output_name);
    let mut file = File::create(strings::concat_with_static(&output_filename, file_extensions::STATISTICS))
        .expect(errors::FAILED_CREATING_FILE);

    let formatted_output = format!("### Statistics for {color}\n\
    | Parameter     | Wert          |\n\
    |:--------------|:--------------|\n\
    |Minimum        |{minimum}      |\n\
    |Maximum        |{maximum}      |\n\
    |Average        |{average}      |\n\
    |Variance       |{variance}     |\n\
    |Median         |{median}       |\n\
    |Contrast       |{contrast}     |\n\
    |Median         |{median}       |\n\
    |Dymamics       |{dynamics}     |",
                                   color = channel_output_name.to_uppercase(),
                                   minimum = statistics_output.min,
                                   maximum = statistics_output.max,
                                   average = statistics_output.average,
                                   variance = statistics_output.variance,
                                   median = statistics_output.median,
                                   contrast = statistics_output.contrast,
                                   dynamics = statistics_output.dynamics
    );

    file.write_all(formatted_output.as_bytes())
        .expect(errors::FAILED_WRITING_FILE);
}
