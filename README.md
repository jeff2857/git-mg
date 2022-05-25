# git-emergency

This tool can help you commit and push your code to server if emergency happened.

Inspired by [git-eq](https://github.com/jmevel/git-eq.git).

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

Clone this repo and change directory into it, then you can type the following command to install the executable file to your machine.

```shell
cargo install --path ./
```

## Usage

```shell
git-mg [any message if you want]
```

