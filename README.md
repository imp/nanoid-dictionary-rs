# nanoid-dictionary

Popular alphabets for use with `nanoid::nanoid!()` macro

```
use nanoid_dictionary::NOLOOKALIKES;
use nanoid::nanoid;

fn main() {
    let id = nanoid!(21, NOLOOKALIKES);
    println!("{}", id);
}
```
