+++
title = "Creating Image Processing Website with Wasm"
description = "Creating a website that shows different image processing algorithms and data structures used in image editing, computer vision, and other imaging applications."

# The date of the post.
# 2 formats are allowed: YYYY-MM-DD (2012-10-02) and RFC3339 (2002-10-02T15:00:00Z)
# Do not wrap dates in quotes, the line below only indicates that there is no default date.
# If the section variable `sort_by` is set to `date`, then any page that lacks a `date`
# will not be rendered.
# Setting this overrides a date set in the filename.
date = 2020-01-17

# Template to use to render this page
template = "page.html"

# The taxonomies for that page. The keys need to be the same as the taxonomies
# name configured in `config.toml` and the values an array of String like
# tags = ["rust", "web"]
[taxonomies]

# Your own data
[extra]
+++

## Motivation

I wanted to create a means of demonstrating basic image processing ideas that are used in computer vision, image editing, and other imaging applications. [This website](https://www.imagproc.com/) shows off the more fundamental image processing techniques that underpin imaging applications. I wanted a website that consolidates the basic algorithms and data structures used in image processing to give someone a beginners overview of the possibilities. 

## Technologies 

Since this is a web app I went with javascript, css, and html for the frontend. I wanted to use Webassembly (wasm) for the image processing algorithms. It has near native performance in browser and is supported by all browser engines. It is also easier to write high performance code in wasm because you can use languages like C/C++ and Rust to write efficient code.
 
Since Webassembly is a compilation target for the web, I used Rust to write the actual image processing algorithms. I used Rust over C++ because it probably has the best wasm support. [Wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [wasm-pack](https://github.com/rustwasm/wasm-pack) make javascript, wasm interoperability and compiling to wasm virtually painless, allowing me to write my code without worrying about wasm initialization and javascript binding code. Additionally I wouldn't benefit from using a mature image processing library in C++ because everything will be written from scratch.

A typical example of calling wasm from javascript went like:
```javascript

  function box_blur_image(img, width, kernel_size) {

    // box blur is the wasm function
    let box_blur_raw_data = new Uint8ClampedArray(
        box_blur(
            img,
            width,
            kernel_size
        )
    );

    return box_blur_raw_data;
  }

```
In order to compile rust to wasm I used wasm-bindgen macros allowing wasm-bindgen to create javascript bindings to wasm. They will act as the interop for the javascript used in the UI. Example rust code looks like:
```rust

  // the function javascript calls to perform box blur on an image
  #[wasm_bindgen]
  pub fn box_blur(input_image: Vec<u8>, width: u32, kernel_size: u32) -> Vec<u8> {
      let height = (input_image.len() as u32 / CHANNEL_COUNT) / width;
      let mut image: RgbaImage =
          image::ImageBuffer::from_vec(width, height, input_image)
              .expect("expected image from canvas");

      let kernel = MeanKernel::new(kernel_size);
      box_filter_mut(kernel, &mut image);

      image.into_vec()
  }

```
## Performance Considerations

For this project performance was very important. It is one of the reasons wasm was chosen over javascript. Wasm is generally faster, has a higher performance ceiling, and has more consistent performance than javascript.

I quickly discovered that executing wasm on the main thread would be detrimental to the responsiveness of the website. It would probably be fine on a fast desktop class machine, however, on a mobile phone it would lead to a terrible user experience. The reason performance wasn't good enough was because if anything takes long to execute on the main thread then the browser was blocked from updating the UI, causing perceptible lag. 

Ideal web performance requires that screen updates every 16ms for 60 fps(frames per second). Of those 16ms, some are used for the browser's rendering logic which means my code has to execute in under 10ms as stated by the [RAIL guidelines](https://developers.google.com/web/fundamentals/performance/rail). One operation, Box Blur, takes over 100ms to execute for an RGB image of dimensions 1400 by 900 on desktop machines.

While profiling the app, I discovered passing the image data from the canvas to wasm cost several milliseconds, even on a desktop machine. This means that trying to get everything done within 10ms would be impossible unless I used WebGL for the image processing. 

Some of the algorithms use sliders to change inputs for the algorithm. Every time a user would move the slider the website would lag. To overcome these performance problems, a web worker was used to move wasm execution to another thread. Now every time the user moves the slider or clicks a button a command and data input are sent to the web worker which tell the web worker which algorithm to execute. 

Initially I sent the image over with the user input every time a slider was moved. This turned out to be fine for desktop, but caused slight lag when initially moving the slider on mobile phones. To solve this issue, whenever the user first selects their image for processing, a copy of it is moved to the web worker so no time is wasted sending an entire image to the web worker. This frees up the browser to perform its rendering without any javascript code blocking execution. 

## Algorithms

For the sake of releasing the project in a reasonable time, I decided to start with only four basic algorithms, inversion, box blur, gamma corrections, and sobel edge detector. Inversion was simple to implement; it is the pixel value subtracted from the max possible value for a pixel. 
```rust

  fn invert_mut(image: &mut RgbaImage) {
      let max = std::u8::MAX;
      for pixel in image.pixels_mut() {
          pixel[0] = max - pixel[0];
          pixel[1] = max - pixel[1];
          pixel[2] = max - pixel[2];
      }
  }

```
### Gamma 
Gamma correction used a [Lookup Table](https://en.wikipedia.org/wiki/Lookup_table) because the calculations use floating point arithmetic which can be slow when done millions of times. Using the LUT, I only have to calculate the new pixel value for each 256 possible values, because it's an 8 bit image, and then apply them as I loop through the image. 

```rust

    // power law transform is the general name of gamma correction
  fn power_law_transform_mut(image: &mut RgbaImage, gamma: f32) {
      let lut = {
          let mut lut = [0_u8; 256];
          let max = 255.0;
          for (i, val) in lut.iter_mut().enumerate() {
              // pixel value is normalized first
              *val = ((i as f32 / max).powf(gamma) * max).round() as u8;
          }
          lut
      };
      for pixel in image.pixels_mut() {
          pixel[0] = lut[pixel[0] as usize];
          pixel[1] = lut[pixel[1] as usize];
          pixel[2] = lut[pixel[2] as usize];
      }
  }

```

### Box Blur
Box blur required more research to make it performant in a single threaded application without access to SIMD, multithreading or GPGPU. Because box blur is a convolution filter, it can become quickly expensive to convolve the filter with the image. 

For example, if you had a `1000 x 1000` RGBA image and `3 x 3` box filter, you would perform 27 million additions and 3 million divisions. That isn't bad, but if that filter was `51 x 51` then there would be approximately 7.8 billion additions and the same 3 million divisions.  That is approximately 289 times slower to perform than a `3 x 3` box filter. 

Box filter convolution can be optimized by using the fact it is a [separable convolution](https://en.wikipedia.org/wiki/Separable_filter) filter. Essentially the filter is broken into two smaller filters that, when multiplied together, recreate the original filter. 
```rust 

    // example of 3x3 box filter
    // when convolving you have to divide by the size
    // of the kernel, since box blur takes the average
    // of a region centered on the pixel
  let box_kernel = 
      [
          1, 1, 1,
          1, 1, 1,
          1, 1, 1,
      ]; // * 1 / 9

  // blurs in x direction
  let box_kernel_x = [
      1, 1, 1,
  ]; // * 1 / 3
  // blurs in y direction
  let box_kernel_y = [
      1, 
      1, 
      1,
  ]; // * 1 / 3

```

While separability improves performance significantly, it doesn't fix the issue of increasingly larger kernels. If you tried the box filter on the website, you would see that the larger filter sizes don't feel any slower than the smaller sizes. That performance increase was achieved by using an optimization that used running sums to calculate the convolution value. This mitigated the costs of blurring with larger and larger kernels. This is the [implementation](https://fgiesen.wordpress.com/2012/07/30/fast-blurs-1/) I followed.

The trade off for this improved version is that while the larger filters are enormously faster, the smaller filters aren't as fast as the regular separable versions. I will do further benchmarking, in the future, to determine at which filter size the running sums implementation becomes faster. 

### Sobel Edge Detector
Sobel Edge Detector isn't nearly as expensive to calculate as the larger box filters and fortunately doesn't increase in size. The Sobel operator is the combination of two convolution filters. It uses one filter to find the vertical edges of the image and the second filter finds the horizontal edges. 
```rust

    // finds horizontal edges
  let sobel_x = [
    -1, 0, 1,
    -2, 0, 2,
    -1, 0, 1,
  ];

    // finds vertical edges
  let sobel_y = [
    -1, -2, -1,
    0,  0,  0,
    1,  2,  1,
  ];

```

The edges found are combined to create the final image that displays all the potential edges. 

```rust

  for (final_pixel, (horizontal_edge, vertical_edge)) in image.pixels()
    .zip(horizontal_edges.pixels())
    .zip(vertical_edges.pixels_mut()) {

    final_pixel[0] = 
        ((horizontal_edge[0].powf(2.0)) + (vertical_edge[0].powf(2.0))).sqrt();

  }

```
Finally, a threshold is applied to the image which is used to decide which edges are considered strong enough for the final output. A strong edge is an area that had a large brightness difference between two regions of the image. Thresholding also helps with eliminating image noise which would otherwise show up as 'edges'. 

## Future Features

I'll be adding more image processing algorithms and some data structures like histograms, unsharp mask, gaussian blur, hough transform and waveforms. Performance will have to be greatly improved for lower powered devices. Currently I can probably test for the resolution of mobile devices and determine a more optimal size for the image preview used for display. The current preview image uses a 1.5 megapixel image downsized from the users original image. Even such a small image is unnecessarily large for small mobile phones. 

When Webassembly gets SIMD and multithreading support, it will open greater performance opportunities and should also be beneficial to lowered powered devices.
