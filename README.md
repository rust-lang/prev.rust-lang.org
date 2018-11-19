# rust-www

This repository contains the source files for rust-lang.org.

# NOTE: This repository will be deprecated soon. A new website is currently in Alpha Stage, and being reviewed by the Rust Teams. It will go into beta on November 29, 2018, at which point it will be linked here. The current site will remain up until the beta of the new site is over, however you should refrain from making large changes to this repository as it will soon be deprecated.

## Deployment

[![Build Status](https://travis-ci.org/rust-lang/rust-www.svg?branch=master)](https://travis-ci.org/rust-lang/rust-www)

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
