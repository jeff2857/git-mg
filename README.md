# git-emergency

This tool can help you commit and push your code to server if emergency happened.

You save your code and type:

```shell
git-mg
```

in your project, and it's ok if you're not in the root directory.

Also, you can set alias to this tool to save typing time, like:

```shell
alias mg="git-mg"
```

in your .bashrc file, so you just type `mg` and press enter.

The tool will add and commit your uncommitted files, and push it to the server with a new branch named
`emergency/<origin-branch>-<email>-<save-time>`.

## Installation

```shell
cargo install git-mg
```

## Usage

```shell
git-mg [any message if you want]
```

