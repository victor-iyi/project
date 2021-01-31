<!--
 Copyright (c) 2020 Victor I. Afolabi

 This software is released under the MIT License.
 https://opensource.org/licenses/MIT
-->

# Project

> Command line utility to create new project template.

## Templates

For more control of the generated project, you can create a `"template.toml"` file to configure how files are generated, variables that can
be substituted, directories and files to include/exclude, and many more. Your template configuration file can also contain placeholders
which are provided out-of-the-box. Currently supported palceholders are:

- `{{project-name}}` - This is supplied by either passing the `name` argument to the CLI or automatically inferred from the base project
  directory.

- `{{author-name}}` - Author's name is deteremed from your `cargo` or `git` configuration or a fallback to environment variables. You can also manually set the `$NAME` or `$USERNAME` environment variable.

- `{{author-email}}` - Author's email, like `author-name`, it's gotten from your `cargo` or `git` configuration and a fallback to environment variables. You can also manually set `$EMAIL` environment variable.

## Contribution

You are very welcome to modify and use them in your own projects.

Please keep a link to the [original repository]. If you have made a fork with substantial modifications that you feel may be useful, then please [open a new issue on GitHub][issues] with a link and short description.

[original repository]: https://github.com/victor-iyi/project
[issues]: https://github.com/victor-iyi/project/issues

## License (Apache)

This project is opened under the [Apache License 2.0][license] which allows very broad use for both private and commercial purposes.

A few of the images used for demonstration purposes may be under copyright. These images are included under the "fair usage" laws.

[license]: ./LICENSE