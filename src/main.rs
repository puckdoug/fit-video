use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("fit-video")
        .version(env!("CARGO_PKG_VERSION"))
        .author("FIT Video Creator")
        .about("Combines FIT data with video files to create data-overlaid videos")
        .arg(
            Arg::new("fit")
                .short('f')
                .long("fit")
                .value_name("FIT_FILE")
                .help("The FIT file containing activity data")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("The output video file (defaults to movie.mp4)")
                .required(false)
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("videos")
                .value_name("VIDEO_FILES")
                .help("The video files to combine with FIT data")
                .required(true)
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(PathBuf))
        )
        .get_matches();

    let fit_file = matches.get_one::<PathBuf>("fit").unwrap();
    let output_file = matches
        .get_one::<PathBuf>("output")
        .cloned()
        .unwrap_or_else(|| PathBuf::from("movie.mp4"));
    let videos: Vec<&PathBuf> = matches.get_many::<PathBuf>("videos").unwrap().collect();

    // Verify that the fit file exists
    if !fit_file.exists() {
        eprintln!("Error: FIT file does not exist: {}", fit_file.display());
        std::process::exit(1);
    }

    // Verify that all video files exist
    for video in &videos {
        if !video.exists() {
            eprintln!("Error: Video file does not exist: {}", video.display());
            std::process::exit(1);
        }
    }

    // Print what we would do (for now, until actual implementation)
    println!("Processing FIT file: {}", fit_file.display());
    println!("Output will be saved to: {}", output_file.display());
    println!("Video files to process:");
    for video in videos {
        println!("  - {}", video.display());
    }

    println!("Processing complete!");
    Ok(())
}