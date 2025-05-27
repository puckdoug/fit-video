use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

pub struct AppArgs {
    pub fit_file: PathBuf,
    pub metrics_mode: bool,
    pub output_file: PathBuf,
    pub videos: Vec<PathBuf>,
}

pub fn parse_args() -> Result<AppArgs, Box<dyn std::error::Error>> {
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
            Arg::new("metrics")
                .short('m')
                .long("metrics")
                .help("Extract metrics from FIT file without video processing")
                .action(ArgAction::SetTrue)
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
                .required(false)
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(PathBuf))
        )
        .get_matches();

    let fit_file = matches.get_one::<PathBuf>("fit").unwrap().clone();
    let metrics_mode = matches.get_flag("metrics");
    
    // Default to movie.mp4 if no output file is specified
    let output_file = match matches.get_one::<PathBuf>("output") {
        Some(file) => file.clone(),
        None => PathBuf::from("movie.mp4"),
    };
    
    let videos: Vec<PathBuf> = matches
        .get_many::<PathBuf>("videos")
        .unwrap_or_default()
        .cloned()
        .collect();

    // Verify that the fit file exists
    if !fit_file.exists() {
        eprintln!("Error: FIT file does not exist: {}", fit_file.display());
        std::process::exit(1);
    }

    // Check if we need video files
    if !metrics_mode && videos.is_empty() {
        eprintln!("Error: Video files are required unless --metrics is specified");
        std::process::exit(1);
    }

    // Verify that all video files exist (if any)
    for video in &videos {
        if !video.exists() {
            eprintln!("Error: Video file does not exist: {}", video.display());
            std::process::exit(1);
        }
    }

    Ok(AppArgs {
        fit_file,
        metrics_mode,
        output_file,
        videos,
    })
}