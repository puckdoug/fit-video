mod cli;
mod fit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = cli::parse_args()?;
    
    if args.metrics_mode {
        println!("Extracting metrics from FIT file: {}", args.fit_file.display());
        fit::extract_metrics(&args.fit_file)?;
    } else {
        println!("Processing FIT file: {}", args.fit_file.display());
        println!("Output will be saved to: {}", args.output_file.display());
        println!("Video files to process:");
        for video in &args.videos {
            println!("  - {}", video.display());
        }
        println!("Processing complete!");
    }

    Ok(())
}