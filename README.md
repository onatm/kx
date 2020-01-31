# kx

`kx` is a utility to switch interactively between kubernetes contexts without any external dependencies and bash witchcraft. Written in Rust :crab:

```md
USAGE:
  kx                   : list the contexts
  kx <NAME>            : switch to context <NAME>
  kx -c, --current     : show the current context name
```

## Not ready to use

Current implementation will break your kubernetes configuration due to a bug in `serde_yaml`: <https://github.com/dtolnay/serde-yaml/issues/87>
