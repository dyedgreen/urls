# https://urls.fyi

[![CI](https://github.com/dyedgreen/urls/actions/workflows/ci.yml/badge.svg)](https://github.com/dyedgreen/urls/actions/workflows/ci.yml)

A personal link aggregator.

## Development
Pointers for how to do things.

### Build FrontEnd JS Resources

To generate a production build:
```bash
$ cd www
$ npm install
$ ./node-modules/.bin/snowpack build
```

To constantly rebuild (for fish)
```fish
$ cd www
$ while true; ./node_modules/.bin/snowpack build; sleep 2; end
```
