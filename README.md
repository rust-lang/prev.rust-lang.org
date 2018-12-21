# prev.rust-lang.org

This repository contains the source files for prev.rust-lang.org.

# NOTE: This repository has already been deprecated, use <https://github.com/rust-lang/www.rust-lang.org> instead.

## Deployment

[![Build Status](https://travis-ci.org/rust-lang/prev.rust-lang.org.svg?branch=master)](https://travis-ci.org/rust-lang/prev.rust-lang.org)

The site is built with TravisCI and automatically deployed to S3.

## Testing Locally

To install jekyll (and any other dependencies, if we get any):

```
bundle install
```

To run this website as a developer:

```
jekyll serve
```

This will serve the site at `localhost:4000`.
