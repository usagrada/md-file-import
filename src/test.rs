use super::*;

#[test]
fn test_match_code() {
  let before = r"
```txt
// comment
  #import(test/test.txt)
```

hello
ここは消されたくない

#@import(world.md)

```text
#@import(world.md)
```

";
  // match_code(before);
  assert_eq!(
    parse_file(before, ""),
    r"
```txt
// comment
this is test.txt

is this ok?

```

hello
ここは消されたくない

#@import(world.md)

```text
#@import(world.md)
```

"
  );
}
