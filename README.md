# filter.alfredworkflow

A utility that takes an [Alfred](https://www.alfredapp.com/) workflow's
`info.plist` file from `STDIN` and outputs a cleaned version onto `STDOUT` that
does not include values for environment variables that are set to "Don't
Export" in Alfred.

It's for people that store Alfred preferences in a public git repo.

## Usage

Intended for use with git's [clean/smudge
filtering](https://git-scm.com/book/en/v2/Customizing-Git-Git-Attributes#Keyword-Expansion).

### alfredworkflow_clean

``` shell
git clone https://github.com/kejadlen/filter_alfredworkflow.git
cd filter_alfredworkflow
cargo build --release
```

`alfredworkflow_clean` will be located in the `target/release` directory of the
repo and will need to be copied to somewhere in your `PATH`.

### git

#### .gitattributes

```
Alfred.alfredpreferences/workflows/*/info.plist filter=alfredworkflow
```

#### git config

``` shell
git config filter.alfredworkflow.clean alfredworkflow_clean
```
