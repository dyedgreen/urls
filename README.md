# Urls

[![CI](https://github.com/dyedgreen/urls/actions/workflows/ci.yml/badge.svg)](https://github.com/dyedgreen/urls/actions/workflows/ci.yml)

A personal link aggregator.

## To Do
- [ ] Implement mailer infrastructure ... (will be used for pwd less logins)
- [ ] Implement sessions
- [ ] Implement uses, user sessions and a login page
- [ ] Decide on front-end ...

## Development
Pointers for how to do things

### Build FrontEnd JS Resources

Run the following commands. If you want to generate the final build, omit the
`--watch`, since it will prevent bundling.

```bash
$ cd www
$ npm install
$ ./node-modules/.bin/snowpack build (--watch)
```
