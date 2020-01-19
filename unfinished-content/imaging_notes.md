+++
title = "Creating Image Processing Software with Webassembly"
description = "image processing"

# The date of the post.
# 2 formats are allowed: YYYY-MM-DD (2012-10-02) and RFC3339 (2002-10-02T15:00:00Z)
# Do not wrap dates in quotes, the line below only indicates that there is no default date.
# If the section variable `sort_by` is set to `date`, then any page that lacks a `date`
# will not be rendered.
# Setting this overrides a date set in the filename.
date = 2020-01-17

# The weight as defined in the Section page
# If the section variable `sort_by` is set to `weight`, then any page that lacks a `weight`
# will not be rendered.
weight = 0

# A draft page will not be present in prev/next pagination
draft = false

# If filled, it will use that slug instead of the filename to make up the URL
# It will still use the section path though
#slug = ""

# The path the content will appear at
# If set, it cannot be an empty string and will override both `slug` and the filename.
# The sections' path won't be used.
# It should not start with a `/` and the slash will be removed if it does
#path = ""

# Use aliases if you are moving content but want to redirect previous URLs to the
# current one. This takes an array of path, not URLs.
aliases = []

# Whether the page should be in the search index. This is only used if
# `build_search_index` is set to true in the config and the parent section
# hasn't set `in_search_index` to false in its front-matter
in_search_index = true

# Template to use to render this page
template = "page.html"

# The taxonomies for that page. The keys need to be the same as the taxonomies
# name configured in `config.toml` and the values an array of String like
# tags = ["rust", "web"]
[taxonomies]

# Your own data
[extra]
+++

## Convolution Filters separability
    Convolution matrices sometimes have a property that makes them separable.
    Being separable means if you have a n x n matrix, <i>A</i> then <i>A</i> can be split 
    into a <i>1 x n</i> row vector, <i>B</i> and a <i>n x 1</i> column vector, <i>C</i>.
    These vectors multiplied together recreate <i>A</i>.

## Convolution filters and image edges

    The diagram shows a problem with convolution filters. Sometimes the filter,
    at the edges, goes beyond the border of the image. If your image is represented
    by an array, then trying to access outside of the image would be an out of bounds
    access, which should crash the program. There are a few ways to deal with filters
    at the edges of images. One would be to create a clone of the image with the border
    extended to accomodate the filter size. This would be slow and impractical. Another 
    choice is to wrap the filter around the image. So if the filter passes the top border
    of the image then the part that doesn't overlap will overlap the bottom of the image.  
    Besides that, one can take the pixels at the border and use those as the pixel value
    whenever exceeding the border. You can also ignore the boundaries which will shrink the 
    image, but that probably isn't correct for most applications.

## Imaging Pipeline

    Cameras are usually the primary imaging device. Cameras have sensors that particularly sensitive
    to light. They in fact capture most light from the entire light spectrum like infrared and ultraviolet.
    Cameras take photos and videos; from there, it converts the analog signal, or the photons that hit
    the sensor into a digital signal. The more light that hits the sensor, the better the analog signal, the less
    noise is found in the image. The image or video that is converted from the analog signal is usually known as a
    raw image or video. This is the image in its purest form but it isn't appropriate for viewing and consumption
    like other image formats. There are certain programs that are capable of reading raw image formats. They are
    usually image editors. Raw image formats give an image the most amount of range for editing. Raw images
    have the most amount of information about an image, which allows more extreme editing while still preserving
    details and image dynamic range. For this explanation, however, I'll assume the camera edits the image
    for consumption and outputs a JPEG file. In order to do that, the camera throws away a lot of visually redundant
    information and compresses the image. It applies some modifications to the image's exposure and contrast, and
    creates a final image for viewing. From here, the user is free to do more image manipulation on the image, or send
    it for further processing on an image processing pipeline.