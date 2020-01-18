+++
title = "Histogram Specification"
description = "A type of pixel operation that using a look up table to determine the transformed value of a pixel"

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

# Template to use to render this page
template = "page.html"
+++

**Introduction**

You want to learn more about histogram specification relating to image processing; however, information is scarce on the Internet and books on image processing in a digestible format are rare. This article hopes to present algorithms for histogram specification so that you can implement them in the language of your choosing.  

**Histogram Equalization**
Before diving into histogram specification, histogram equalization has to be understood. The method to match a specified histogram derives from equalization. 
In histogram equalization the goal is to turn the input image’s probability distribution function into a uniform distribution. The probability distribution function(PDF) is the normalized distribution of the histogram for the image. It is found by dividing the number of pixels for each intensity value for the image’s range of possible intensity values by the total number of pixels in the image. For example, an 8 bit image has the range of 0 to 255 for its intensity values and if an image is 64x64 then each histogram bin is divided by 4096. In order to achieve a better distribution of the intensity values, it’s important to make the cumulative distribution function(CDF) of the image linear. A linear CDF is associated with a uniform PDF. In order to transform the CDF into a linear form, multiply each CDF value by the maximum value for the intensity range. 
There is a mathematical motivation concerning continuous random variables to tries to explain how PDFs and CDFs are related to histogram equalization. Multiply the equalized values by the max intensity value, i.e, 255 because the CDF is normalized and that is the only way to bring the values back to the non normalized range.  
Why does multiplying CDF value by max intensity value produce a value within intensity value range?

**Histogram Specification**

There are different algorithms for histogram specification that make trade offs between the amount of histogram distortion and execution speed of the algorithm. First we will look at the classic version of histogram matching that maps the reference histogram to a equalized histogram and uses that mapping from the equalized to reference histogram to create the mapping for the original image. 
In order to map from the original histogram to the reference histogram. You must figure out how to equalize your histogram then figure out how to mapping between the equalized histogram and the reference image histogram. Using that you can determine the mapping from the original image to the reference image. In simpler terms, map from original to equalized then figure out the inverse mapping from reference to equalized. 

Compute histogram for given image and find histogram equalization transformation 
Compute all values of the transformation function G 