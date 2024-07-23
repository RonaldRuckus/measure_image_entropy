A very quickly written library to calculate the entropy of pixels in each line of an image.
A use-case is to differentiate between an artistic image, or a professional document.

```
/// Calculates the entropy of an image.
/// ### Parameters
/// - `img_path` - The path to the image
/// - `slice_percentage` - The percentage of the image to sample.\
///     - A sample of 50.0 will run in roughly half the time but may not be as accurate.\
///     - Defaults to 100.0
pub fn calculate_image_entropy(img_path: &str, slice_percentage: Option<f64>) -> f64
```

### Improvements
- Load image in buffer instead of as whole
    - Probably needs a full re-write to do LOL but worth it
- Optional multi-threading