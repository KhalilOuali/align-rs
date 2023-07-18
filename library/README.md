# align_text: Align text within a specified width

This crate defines a trait `Align` with a method `align_text()`
implemented for two types:

* `Vec<String>` where each String is considered a line
* `String`

You can specify the alignment, the number of columns, whether to wrap long lines, whether to trim lines first, etc.

## Examples
* `align_text(Where::Center, Some((30, false)), true, Bias::Right, true)`

Input lines:
```
"Hello           ",
"            World!",
"   This should center-align     ",
```
Output lines:
```
"             Hello            ",
"            World!            ",
"   This should center-align   ",
```

* `align_text(Where::Right, Some((40, false)), false, Bias::Left, false)`

Input text's lines:
```
"graphic design"
"is my"
"pAsSiOn"
```
* Output text's lines:
```
"             graphic design",
"             is my",
"             pAsSiOn",
```
