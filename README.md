# camo polish

> a workaround for [Github's aggressive caching of user image content](https://github.com/github/markup/issues/224)


Sends an [HTTP PURGE](https://stackoverflow.com/questions/25857508/what-is-the-http-method-purge) request for image links to github's camo host.


```sh
$ cargo run -q softprops/envy
Ok(
    [
        (
            "Build Status",
            "https://travis-ci.org/softprops/envy.svg?branch=master",
            200
        ),
        (
            "Coverage Status",
            "https://coveralls.io/repos/github/softprops/envy/badge.svg?branch=master",
            200
        ),
        (
            "Software License",
            "https://img.shields.io/badge/license-MIT-brightgreen.svg",
            200
        ),
        (
            "crates.io",
            "http://meritbadge.herokuapp.com/envy",
            200
        ),
        (
            "Master API docs",
            "https://img.shields.io/badge/docs-master-green.svg",
            200
        )
    ]
)
```