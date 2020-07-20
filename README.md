# buildkite_waiter

_Development Status: Pre-Alpha_

A command-line tool which waits for Buildkite builds to complete, and then notifies you.

## Installation

Download the binary from the [latest release](https://github.com/liamdawson/buildkite_waiter/releases/latest), and place it somewhere on `$PATH`.

## Usage

```shell
$ buildkite_waiter --help
buildkite_waiter 0.0.1-alpha.2

USAGE:
    buildkite_waiter <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    login     Save a Buildkite API Access Token
    logout    Remove the saved Buildkite API token
    wait      Wait for a build to finish

$ buildkite_waiter login
Generate an API Access Token at https://buildkite.com/user/api-access-tokens/new
Ensure you enable "Read Builds", and optionally "Read User".
Buildkite API Access Token: 
OK

$ buildkite_waiter wait --notification --url https://buildkite.com/my-great-org/my-pipeline/builds/1
# or
$ buildkite_waiter wait --output-notification-json --organization my-great-org --pipeline my-pipeline --number 1
Waiting for my-pipeline/1 by Keith Pitt on branch master
  Bumping to version 0.2-beta.6
...
{"title":"Build Passed","subtitle":"solid-octane-service/1636 master","message":"Finished 20 minutes and 33 seconds ago"}
```

## API Access Token

The API Access Token is stored in a system keychain. While the "Read Builds" permission is necessary,
the "Read User" permission is for an anticipated future feature, and is intended to always be optional.

## License

MIT OR Apache-2.0
