use image::{GenericImageView, Pixel};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

/// Calculates the entropy of an image.
/// ### Parameters
/// - `img_path` - The path to the image
/// - `slice_percentage` - The percentage of the image to sample.\
///     - A sample of 50.0 will run in roughly half the time but may not be as accurate.\
///     - Defaults to 100.0
pub fn calculate_image_entropy(img_path: &str, slice_percentage: Option<f64>) -> f64 {
    let img = image::open(img_path).unwrap();
    let (width, height) = img.dimensions();

    let lines_to_sample: Vec<u32> = if let Some(percentage) = slice_percentage {
        if percentage >= 100.0 {
            (0..height).collect()
        } else {
            let percentage = percentage.clamp(1.0, 100.0);
            let num_lines = (height as f64 * percentage / 100.0).round() as u32;
            let mut rng = rand::thread_rng();
            let mut line_indices: Vec<u32> = (0..height).collect();
            line_indices.shuffle(&mut rng);
            line_indices.into_iter().take(num_lines as usize).collect()
        }
    } else {
        (0..height).collect()
    };

    let mut color_counts = HashMap::new();
    let mut total_pixels = 0;

    for y in lines_to_sample {
        for x in 0..width {
            let rgb = img.get_pixel(x, y).to_rgb();
            *color_counts.entry(rgb).or_insert(0) += 1;
            total_pixels += 1;
        }
    }

    color_counts.values().fold(0.0, |entropy, &count| {
        let p = count as f64 / total_pixels as f64;
        entropy - p * p.log2()
    })
}

mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_calculate_image_entropy() {
        dotenv().ok();

        let mut plots: HashMap<String, Vec<u128>> = HashMap::new();

        let mut intervals = vec![10.0];
        {
            let mut current_value = 10.0;
            while current_value < 100.0 {
                current_value += 10.0;
                if current_value < 100.0 {
                    intervals.push(current_value);
                }
            }
        }
        // Load from ENV
        let testing_folder_loc =
                std::env::var("TEST_DIR").expect("Missing `TEST_DIR` environment variable");

        // Gather image related file paths
        let mut image_paths = vec![];
        let image_extensions = vec!["jpg", "jpeg", "png", "bmp", "gif", "tiff", "webp"];
        let limit = 10;

        for extension in image_extensions {
            let pattern = format!("{}/**/*.{}", testing_folder_loc, extension);
            for entry in glob::glob(&pattern).unwrap() {
                match entry {
                    Ok(path) => {
                        image_paths.push(path.to_str().unwrap().to_string());
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }

        let mut master_lapsed_time = 0;

        for interval in intervals { 
            let mut interval_plots: Vec<u128> = Vec::new();
            let mut interval_elapsed_time = 0;

            for img_path in image_paths.iter().take(limit) {
                let start = std::time::Instant::now();
                let entropy = calculate_image_entropy(img_path, Some(interval));
                let elapsed = start.elapsed().as_millis();
                interval_plots.push(elapsed);
                interval_elapsed_time += elapsed;
                println!(
                    "Path: {}\nEntropy: {}\nTime: {}ms\n----------------",
                    img_path, entropy, elapsed
                );
            }

            plots.insert(interval.to_string(), interval_plots);

            master_lapsed_time += interval_elapsed_time;

            println!("Interval elapsed time: {}s", (interval_elapsed_time / 1000));
            println!("Average elapsed time: {}ms", interval_elapsed_time / limit as u128);
            println!("Total elapsed time: {}s", (master_lapsed_time / 1000));
            println!("Estimated time per interval: {}s", (master_lapsed_time as f64 / 1000.0) / (interval / 10.0));
        }

        println!("Plots: {:?}", plots);
    }
}
