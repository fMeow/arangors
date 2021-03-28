# Arangors Contribution Guidelines

Thank you for your interest in making `arangors` better! We'd love to have your contribution.
We expect all contributors to abide by the 
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). 
Thank you so much!

## Pull Requests

To contribute to `arangors`, please send in pull requests on GitHub to the develop branch. 
We'll review them and either merge or request changes. 
Github action is enabled, so you may get feedback from it too.

If you make additions or other changes to a pull request, 
feel free to either amend previous commits or only add new ones, however you prefer. 

## Contribution Workflow
Before creating a pull request, please check that:
1. rustfmt is applied to the latest code
2. for new features, you are encouraged to add e2e test code
3. your commit message is complied with Angular Commit Style

A pull request will not be accepted if it fails to pass Github Action test.

Thank you so much!

## Commit Message Style Guide
`arangors` is stick with angular commit style, and use standard-version to generate
changelog automatically. Please make sure your commit messages follow 
[angular commit style](https://www.conventionalcommits.org/en/v1.0.0).

### Brief Introduction
The commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

A blank line is required between the first line(type and description) and body.

Type must be the following:

- **build**: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
- **ci**: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
- **docs**: Documentation only changes
- **feat**: A new feature
- **fix**: A bug fix
- **perf**: A code change that improves performance
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- **test**: Adding missing tests or correcting existing tests

And there is no restriction on scope, description and message body.

When a commit introduces breaking API changes, it must have a footer starting with `BREAKING CHANGE:`.
The following lines is an example of angular commit style with breaking change.

```
feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other config files
```

## E2E Test Guide 

To run e2e test, we recommend you use docker to get a fresh and isolated installation of
arango server. A [docker-compose.yml]() file is provided, so you can use `docker-compose up -d` 
to setup a fresh and working arangodb server at `localhost:8529`. 

The e2e test requires some predefined data. So before running e2e test, please 
use `tests/init_db.sh` to add data to arangodb server.

```sh
$ docker-compose up -d
Creating network "arangors_default" with the default driver
Creating arangors_arangodb_1 ... done
$ sh tests/init_db.sh
```

Then you can run `cargo test` to see if it works. Please test both async and blocking 
version at least once.
