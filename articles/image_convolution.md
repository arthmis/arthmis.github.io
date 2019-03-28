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
