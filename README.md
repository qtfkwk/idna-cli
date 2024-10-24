# About

Encode/decode Unicode domain names to/from IDNA ASCII

# Usage

```text
$ idna -h
Encode/decode Unicode domain names to/from IDNA ASCII

Usage: idna [OPTIONS] [DOMAINS]...

Arguments:
  [DOMAINS]...  One or more domains

Options:
  -d, --decode           Decode IDNA ASCII input to Unicode
  -f, --files <FILES>    One or more files
  -o, --output <FORMAT>  Output format (csv, json, json-pretty, rust,
                         rust-pretty) [default: csv]
  -h, --help             Print help
  -V, --version          Print version
```

```text
$ idna -V
idna 0.2.0
```

# Examples

## Encode

```text
$ idna goögle.com
"Unicode","ASCII","Errors"
"goögle.com","xn--gogle-kua.com",""
```

## Decode

```text
$ idna -d xn--gogle-kua.com
"ASCII","Unicode","Errors"
"xn--gogle-kua.com","goögle.com",""
```

## Files

```text
$ idna -f unicode.txt
"Unicode","ASCII","Errors"
"goögle.com","xn--gogle-kua.com",""
```

```text
$ idna -df ascii.txt
"ASCII","Unicode","Errors"
"xn--gogle-kua.com","goögle.com",""
```

## JSON output

```text
$ idna goögle.com -o json
{"Arguments":{"goögle.com":{"ascii":"xn--gogle-kua.com","unicode":"goögle.com","errors":""}}}
```

```text
$ idna -d xn--gogle-kua.com -o json
{"Arguments":{"xn--gogle-kua.com":{"ascii":"xn--gogle-kua.com","unicode":"goögle.com","errors":""}}}
```

```text
$ idna goögle.com -o json-pretty
{
  "Arguments": {
    "goögle.com": {
      "ascii": "xn--gogle-kua.com",
      "unicode": "goögle.com",
      "errors": ""
    }
  }
}
```

```text
$ idna -d xn--gogle-kua.com -o json-pretty
{
  "Arguments": {
    "xn--gogle-kua.com": {
      "ascii": "xn--gogle-kua.com",
      "unicode": "goögle.com",
      "errors": ""
    }
  }
}
```

### Files to JSON

```text
$ idna -f unicode.txt -o json
{"File: \"unicode.txt\"":{"goögle.com":{"ascii":"xn--gogle-kua.com","unicode":"goögle.com","errors":""}}}
```

```text
$ idna -df ascii.txt -o json
{"File: \"ascii.txt\"":{"xn--gogle-kua.com":{"ascii":"xn--gogle-kua.com","unicode":"goögle.com","errors":""}}}
```

```text
$ idna -f unicode.txt -o json-pretty
{
  "File: \"unicode.txt\"": {
    "goögle.com": {
      "ascii": "xn--gogle-kua.com",
      "unicode": "goögle.com",
      "errors": ""
    }
  }
}
```

```text
$ idna -df ascii.txt -o json-pretty
{
  "File: \"ascii.txt\"": {
    "xn--gogle-kua.com": {
      "ascii": "xn--gogle-kua.com",
      "unicode": "goögle.com",
      "errors": ""
    }
  }
}
```

