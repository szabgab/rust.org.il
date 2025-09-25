# The source of [Rust in Israel](https://rust.org.il/)


## Generate the site locally

Run `cargo run` to generate the web site in the `_site` folder.

## See the site locally

Install [Rustatic](https://rustatic.code-maven.com/) run

```
rustatic --nice --indexfile index.html --path _site/
```

then visit `http://localhost:5000/`

## Contribute

It is recommended that you install [pre-commit](https://pre-commit.com/) and configure it in the folder of the project by running `pre-commit install`.
From that point our checks will run locally before every commit.


# TODO

* If the companies have open source projects in Rust, link to them.
* Add more details explaining the type of applications each one writes in Rust.
* Link to the presentation given by employees of each company.
* Add contact info. (e.g. a community member)

## Other

* [42 Companies using Rust in production](https://kerkour.com/rust-in-production-2021) from April 8, 2021.


