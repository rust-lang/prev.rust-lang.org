# rust-www

This repository contains the source files for rust-lang.org.

## Deployment

type | status
| ---- | ---- |
| `master` | [![master branch](https://travis-ci.org/rust-lang/rust-www.svg?branch=master)](https://travis-ci.org/rust-lang/rust-www) |
| `friends` | [![rust-www-friends-verifier](https://img.shields.io/travis/simeg/rust-www-friends-verifier/master.svg?label=friends)](https://travis-ci.org/simeg/rust-www-friends-verifier) |

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
