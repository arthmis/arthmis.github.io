+++
title = "Creating Image Processing Software with Webassembly"
description = "The journey to creating a website that uses webassembly to demonstrate basic image processing algorithms." 

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

I wanted to create a means of demonstrating basic image processing ideas that are used in computer vision, image editing, and other imaging applications. This website shows off the more fundamental image processing techniques that under pin imaging applications. I haven't found a website that consolidates the basics and gives an overview of the algorithms and data structures and how they're implemented.

## Tech stack used

Since this is a web app I went with javascript, css, and html for the frontend. I used plain javascript to script the functionality of the website. React was a consideration for view logic, and while it is nice to use, I didn't think it was necessary for this website. I also didn't want to waste time relearning it while doing my first solo web app. I wanted to use Webassembly (wasm) for image processing algorithms. Therefore, javascript would send the image data to wasm where wasm would execute the algorithm on the data and send back the result to javascript. Wasm is also supported by all major browsers, so user support shouldn't be an issue. Since Webassembly is a compilation target for the web, I used Rust to write the actual image processing algorithms. I used Rust over C++ because it probably has the best wasm support and I don't need to use any open source image processing libraries, because everything will be written from scratch.

## Designing

The user would be able to upload an image they have where it would be displayed on the right on a larger screen or the center on a smaller mobile device. The possible algorithm options would be displayed on the left for user selection. Clicking on one of the algorithms displays the appropriate slider or button for manipulating the image using that image processing technique. Below that will be a basic written explanation of the algorithm and how it is implemented. For this project performance was very important. It is one of the reasons wasm was chosen over javascript because wasm is generally faster and has more consistent performance.  I quickly discovered that executing wasm on the main thread would be detrimental to the responsiveness of the website. It would probably be fine on a fast desktop class machine, however, on a mobile phone it would lead to a terrible user experience. Reasons performance wasn't good enough is because if anything takes long to execute on the main thread then there is noticeable lag on the website. Ideal web performance requires that screen updates every 16ms for 60 fps. Of those 16ms, some are used for the browser's rendering logic which means my code has to execute in under 10ms using the [RAIL guidelines](https://developers.google.com/web/fundamentals/performance/rail)

 Unfortunately just passing the image data from the canvas to wasm cost several milliseconds, even on a desktop machine. This means that trying to get everything done within 10ms would be impossible unless I used WebGL for the image processing.  Some of the algorithms use sliders to change inputs for the algorithm, which means it is ideal that the slider doesn't lag every time the user moves it. To overcome these performance problems, a web worker was used to move wasm execution to another thread. Now every time the user moves the slider or clicks a button a command and input is sent to the web worker that tells it which algorithm to execute. Initially I sent the image over with the user input every time a slider was moved. This turned out to be fine for desktop, but caused slight delay on mobile phones. To solve this issue, whenever the user selects their image, a copy of it is moved to the web worker so no time is wasted sending an entire image to the web worker. This frees up the browser to perform its rendering without any javascript code blocking execution. 



## Algorithms

For the sake of releasing the project in a reasonable time, I decided to start with only four basic algorithms, inversion, box blur, gamma corrections, and sobel edge detector. Inversion was simple to implement. Gamma correction used a Lookup Table because the calculations make use of floating point arithmetic which can be slow when done millions of times. First the lookup table is precalculated then applied to each channel of the image, except alpha. Using the LUT I only have to calculate the new pixel value for each 255 possible values, because it's an 8 bit image, and then apply them as I loop through the image. 

Box blur required more research to make it performant for a single threaded application without SIMD, multithreading or GPGPU. Because box blur is a convolution filter, it can become quickly expensive to convolve the filter with the image. For example, if you had a 1000 x 1000 image and 3 x 3 box filter, you would perform 27 million additions and 3 million divisions. It is 27 million and not 9 million because the box filter has to be applied to the red, green, and blue channel of the image. That isn't bad, but if that filter was 51 x 51 then there would be approximately 7.8 billion additions and the same 3 million divisions.  That is approximately 289 times slower to perform than a 3 x 3 box filter. The first optimization comes from the fact that box filter is a separable convolution filter(explain more maybe). If you tried the box filter on the website, you would see that the larger filter sizes don't feel any slower than the smaller sizes. That performance increase was achieved by using an optimization that used running sums to calculate the convolution value. The implementation I followed came from this [website](https://fgiesen.wordpress.com/2012/07/30/fast-blurs-1/)

The trade off for this improved version is that while the larger filters are enormously faster, the smaller filters aren't as fast as the regular separable versions. I have not benchmarked to find out where the running sums become faster. 

Sobel Edge Detector isn't nearly as expensive to calculate as the larger box filters and fortunately doesn't increase in size. The Sobel operator is also a convolution filter; two to be exact. It uses one filter to find the vertical edges of the image and the second filter finds the horizontal edges. The edges found are combined to create the final image that displays all the potential edges. Finally, a threshold is applied to the image which is used to decide which edges are considered strong enough for the final output. Thresholding also helps with eliminating image noise which would otherwise show up as 'edges'. (Think about showing code examples between different possible implementations used for performance reasons)

## Performance

Performance is acceptable for laptops and desktops. More powerful modern phones also have decent performance, however, few year old phones and cheaper smart phones don't have great performance. When Webassembly gets SIMD and multithreading support, it will open greater performance opportunities and should be beneficial to lowered powered devices. 