# About

Encode/decode Unicode domain names to/from IDNA ASCII

# Usage

```text
$ idna -h
!run:../target/release/idna -h
```

```text
$ idna -V
!run:../target/release/idna -V
```

# Examples

## Encode

```text
$ idna goögle.com
!run:../target/release/idna goögle.com
```

## Decode

```text
$ idna -d xn--gogle-kua.com
!run:../target/release/idna -d xn--gogle-kua.com
```

## JSON output

```text
$ idna goögle.com -o json
!run:../target/release/idna goögle.com -o json
```

```text
$ idna -d xn--gogle-kua.com -o json
!run:../target/release/idna -d xn--gogle-kua.com -o json
```

```text
$ idna goögle.com -o json-pretty
!run:../target/release/idna goögle.com -o json-pretty
```

```text
$ idna -d xn--gogle-kua.com -o json-pretty
!run:../target/release/idna -d xn--gogle-kua.com -o json-pretty
```

## Files

```text
$ idna -f unicode.txt
!run:../target/release/idna -f ../unicode.txt
```

```text
$ idna -df ascii.txt
!run:../target/release/idna -df ../ascii.txt
```

