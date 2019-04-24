+++
title = "Zola Tutorial"
date = 2019-04-24
+++

## CLI usage

## Creating a simple one page landing page for a blog

Creating a landing page for your blog is easy, but it isn't necessarily obvious for someone
that has never used a static site generator or the templating tools available. 

Use a for loop to populate landing page with articles and use description or truncate content to add
a little description below the title.

To make the permalinks link correctly to your article page, set the `base_url` variable to `/`.

Date is necessary in the front matter for each content piece or else Zola won't be able to parse them.