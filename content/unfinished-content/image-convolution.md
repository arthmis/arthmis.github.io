+++
title = "Image Convolution"
description = ""

# The date of the post.
# 2 formats are allowed: YYYY-MM-DD (2012-10-02) and RFC3339 (2002-10-02T15:00:00Z)
# Do not wrap dates in quotes, the line below only indicates that there is no default date.
# If the section variable `sort_by` is set to `date`, then any page that lacks a `date`
# will not be rendered.
# Setting this overrides a date set in the filename.
date = 2019-04-18

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

### Convolution optimizations

- Handle the borders separately in their own loops (or whatever)
- Then convolve the rest of the inner image without worrying about bounds checking
- from there use separable filters, if possible, to get the most speed up
- perform the multiplications using integers(unsigned) then divide by floating point number(floating point is slower)
- Use integral images to also make convolution faster(need to find more information on this)
- use separable filters, if possible, for speed up (especially on large kernels)

### Convolution Kernels

- preferable to make them odd in height and width
- using even sized kernels can lead to aliasing?(have to look that up)
- using border pixels to deal with out of bounds kernels seems preferable
- other acceptable choices are wrapping and mirroring

### Different filters

#### Linear Filters
- gaussian, box
- unsharp mask
- sharpening filter

#### Non linear filters
- minimum filter
- maximum filters
- median filter
- weighted median filter