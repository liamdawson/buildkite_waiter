# buildkite_waiter (alpha)

A command-line tool which waits for Buildkite builds to complete, and then notifies you.

## Installation

### Homebrew

```shell
brew install liamdawson/repo/buildkite_waiter
```

### Shell

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/liamdawson/buildkite_waiter/releases/latest/download/buildkite_waiter-installer.sh | sh
```

### Manual

Download the binary from the [latest release](https://github.com/liamdawson/buildkite_waiter/releases/latest), and place it somewhere on `$PATH`.

## Examples

```shell
$ buildkite_waiter login
Generate an API Access Token at https://buildkite.com/user/api-access-tokens/new
Ensure you enable "Read Builds", and optionally "Read User".
Buildkite API Access Token:
OK

# wait for the latest build triggered by the logged in user, receive a notification when done (requires "Read User" permission)
$ buildkite_waiter latest --mine

# wait for the latest build on the main branch for my-pipeline in my-great-org
$ buildkite_waiter latest --organization my-great-org --pipeline my-pipeline --branch main

# wait for a build based on the web URL
$ buildkite_waiter by-url https://buildkite.com/my-great-org/my-pipeline/builds/1

# wait for a build based on build number
$ buildkite_waiter by-number my-great-org my-pipeline 1

# wait for latest, but don't send an OS notification
$ buildkite_waiter latest --no-notification

# organization can be set by environment variables (among other options)
$ export BUILDKITE_WAITER_ORGANIZATION=my-great-org
$ buildkite_waiter latest --pipeline my-pipeline
```

## API Access Token

The API Access Token is stored in a system keychain. While the "Read Builds" permission is necessary, the "Read User"
permission is only necessary to use `latest --mine`, which can be replaced with `latest --creator <your id>`.

## License

MIT OR Apache-2.0
