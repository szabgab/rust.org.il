# The source of [Rust in Israel](https://rust.org.il/)


Run `cargo run` to check the formatting of the speakers.yaml` file.


## Generate the site locally

Download the [Code Maven SSG](https://ssg.code-maven.com/) and run `code-maven web`.

## See the site locally

Install [Rustatic](https://rustatic.code-maven.com/) run `rustatic --nice --indexfile index.html --path _site/` and visit `http://localhost:5000/`

## Contribute

It is recommended that you install [pre-commit](https://pre-commit.com/) and configure it in the folder of the project by running `pre-commit install`.
From that point our checks will run locally before every commit.