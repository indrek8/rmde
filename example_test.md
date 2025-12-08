# Comprehensive Markdown Syntax Test File

This file tests all syntax from MARKDOWN-SYNTAX.md for the RMDE parser.

---

## 1. ATX Headings

# Heading Level 1
## Heading Level 2
### Heading Level 3
#### Heading Level 4
##### Heading Level 5
###### Heading Level 6

### With Closing Hashes ###
## Heading ##########

   ### Heading with 3 spaces indent

---

## 2. Setext Headings

Setext Heading Level 1
======================

Setext Heading Level 2
----------------------

Multi-line
Setext Heading
--------------

---

## 3. Paragraphs and Line Breaks

This is a paragraph with
soft line breaks that will
become spaces in output.

This is another paragraph separated by a blank line.

Hard line break with two spaces:
This line starts after a hard break.

Hard line break with backslash:\
This line also starts after a hard break.

---

## 4. Blockquotes

> Simple blockquote.

> Multi-line blockquote.
> With continuation.

> Lazy continuation
works here too without the > marker.

> ### Heading in blockquote
>
> - List item in blockquote
> - Another item

> Level 1 quote
> > Level 2 nested quote
> > > Level 3 deeply nested

> Blockquote with code:
>
> ```python
> print("Hello from inside a quote")
> ```

---

## 5. Unordered Lists

- Item with dash
- Another item
- Third item

* Item with asterisk
* Another item

+ Item with plus
+ Another item

### Nested Lists

- Parent item
  - Nested item (2 spaces)
    - Deeply nested (4 spaces)
  - Back to level 2
- Back to level 1

### List with Multiple Paragraphs

- First paragraph of item.

  Second paragraph of same item (indented).

- Another item.

### List with Code Block

- Item with code:

  ```javascript
  function hello() {
    console.log("Hello");
  }
  ```

- Item with blockquote:

  > Quote inside list item

---

## 6. Ordered Lists

1. First item
2. Second item
3. Third item

1) Parenthesis style
2) Also works
3) Like this

### Start Number

5. Starting at 5
6. Continues from there
7. Auto-increment

### Leading Zeros

0. Zero start
00. Also zero (leading zeros)
003. Start at 3

---

## 7. Task Lists (GFM)

- [ ] Unchecked task
- [x] Checked task
- [X] Also checked (capital X)
- [ ] Task with **bold text**

### Nested Task Lists

- [x] Parent task complete
  - [ ] Subtask 1 incomplete
  - [x] Subtask 2 complete
- [ ] Another parent task

---

## 8. Code Blocks

### Indented Code Block

Regular paragraph before code.

    function indentedCode() {
        return "Four spaces indent";
    }

Regular paragraph after code.

### Fenced Code Blocks

```
No language specified
```

```javascript
const greeting = "Hello";
console.log(greeting);
```

```python
def hello():
    print("Hello, world!")

hello()
```

```rust
fn main() {
    println!("Hello, Rust!");
}
```

~~~bash
echo "Tilde fence works too"
~~~

### Long Fence

````markdown
Four backticks opening allows
```
three backticks inside
```
without closing
````

### Code Block with Metadata

```js {highlight: "1,3-5"}
line 1 - highlighted
line 2
line 3 - highlighted
line 4 - highlighted
line 5 - highlighted
```

---

## 9. Thematic Breaks (Horizontal Rules)

Three dashes:

---

Three asterisks:

***

Three underscores:

___

With spaces:

* * *

- - -

Many characters:

_______________

---

## 10. Tables (GFM)

### Simple Table

| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |

### Aligned Table

| Left | Center | Right |
|:-----|:------:|------:|
| L1   | C1     | R1    |
| L2   | C2     | R2    |
| L3   | C3     | R3    |

### Without Leading/Trailing Pipes

Header 1 | Header 2
---------|----------
Cell 1   | Cell 2

### Table with Formatting

| Feature | Syntax | Example |
|---------|--------|---------|
| Bold | `**text**` | **bold** |
| Italic | `*text*` | *italic* |
| Code | `` `code` `` | `code` |
| Link | `[text](url)` | [link](url) |

### Empty Cells

| A | B | C |
|---|---|---|
| 1 |   | 3 |
|   | 2 |   |

---

## 11. Emphasis (Italic)

*Single asterisks for emphasis*

_Single underscores for emphasis_

This is *inline* emphasis in a sentence.

Emphasis can span *multiple
lines* in a paragraph.

---

## 12. Strong (Bold)

**Double asterisks for strong**

__Double underscores for strong__

This is **inline** strong in a sentence.

---

## 13. Combined Emphasis

***Bold and italic with asterisks***

___Bold and italic with underscores___

**_Bold with nested italic_**

*__Italic with nested bold__*

This is ***all bold italic*** text.

**Bold with *italic* word inside**

*Italic with **bold** word inside*

---

## 14. Strikethrough (GFM)

~~Strikethrough text~~

This has ~~deleted~~ text inline.

**~~Bold strikethrough~~**

*~~Italic strikethrough~~*

~~**Strikethrough bold**~~

---

## 15. Code Spans

`Simple code span`

``Code span with `backtick` inside``

``` ``Double backticks`` inside triple ```

This is `inline code` in a sentence.

`var x = 10;`

Code with entity: `&copy;` is literal.

---

## 16. Links

### Inline Links

[Simple link](https://example.com)

[Link with title](https://example.com "Example Title")

[Link in angle brackets](<https://example.com>)

[Relative link](/path/to/file)

[Anchor link](#headings)

[Email link](mailto:email@example.com)

### Reference Links

[Full reference][ref1]

[Collapsed reference][]

[Shortcut reference]

[ref1]: https://example.com "Reference Title"
[collapsed reference]: https://example.com

### Links with Formatting

[**Bold link text**](https://example.com)

[*Italic link text*](https://example.com)

[`Code link text`](https://example.com)

---

## 17. Images

![Alt text](https://via.placeholder.com/150)

![Image with title](https://via.placeholder.com/150 "Placeholder Image")

![Reference image][img-ref]

[img-ref]: https://via.placeholder.com/100 "Reference Image"

### Image in Link

[![Clickable image](https://via.placeholder.com/100)](https://example.com)

---

## 18. Autolinks

### Standard Autolinks

<https://example.com>

<http://example.com/path?query=value>

<mailto:email@example.com>

<ssh://user@host.com>

### Extended Autolinks (GFM)

Visit https://example.com for more info.

Check out www.example.com today.

Email us at user@example.com for support.

---

## 19. HTML Blocks

<div class="container">
This is raw HTML content.
</div>

<!-- HTML Comment -->

<p>Inline HTML paragraph</p>

<div>

*Markdown works here with blank lines*

</div>

<details>
<summary>Expandable section</summary>

Content inside details tag.

</details>

---

## 20. Footnotes

Here is a footnote reference[^1].

Another footnote[^note].

Inline footnote^[This is an inline footnote].

[^1]: This is the footnote content.

[^note]: This is another footnote with a longer name.

    It can have multiple paragraphs with indentation.

---

## 21. Definition Lists

Term 1
: Definition for term 1

Term 2
: First definition for term 2
: Second definition for term 2

Apple
: A fruit that grows on trees
: A technology company

Orange
: A citrus fruit
: A color

---

## 22. Abbreviations

The HTML specification is maintained by the W3C.

*[HTML]: Hyper Text Markup Language
*[W3C]: World Wide Web Consortium

---

## 23. Subscript and Superscript

Water formula: H~2~O

Einstein's equation: E=mc^2^

Chemical: CO~2~ emissions

Quadratic: x^2^ + y^2^ = z^2^

---

## 24. Highlighting

This is ==very important== text.

You can combine ==**bold highlighting**== too.

And ==*italic highlighting*== works.

---

## 25. Math

### Inline Math

The quadratic formula is $x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}$.

Einstein's famous equation: $E = mc^2$

Alternative syntax: \(a^2 + b^2 = c^2\)

### Block Math

$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

$$
\begin{aligned}
\nabla \cdot \vec{E} &= \frac{\rho}{\epsilon_0} \\
\nabla \cdot \vec{B} &= 0
\end{aligned}
$$

Alternative block syntax:

\[
\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}
\]

---

## 26. Escaping Special Characters

\*Not emphasis\*

\[Not a link\]

\`Not code\`

\# Not a heading

\> Not a blockquote

\- Not a list

Backslash: \\

---

## 27. Entity References

Named: &copy; &mdash; &nbsp; &lt; &gt; &amp; &quot;

Decimal: &#35; &#169; &#8212;

Hexadecimal: &#x23; &#xA9; &#x2014;

---

## 28. Edge Cases

### Intraword Emphasis

foo*bar*baz (asterisks work)

foo_bar_baz (underscores don't mid-word)

**foo**bar works

__foo__bar doesn't work mid-word

### Adjacent Emphasis

**foo** **bar** (separate strong)

*foo**bar**baz* (nested)

### Empty Elements

****

____

`` (empty code span not rendered)

---

## 29. Complex Nesting

### List with Quote with Code

- Item with quote:

  > Quote containing code:
  >
  > ```python
  > print("deeply nested")
  > ```

### Quote with List with Nested List

> - Level 1 in quote
>   - Level 2 in quote
>     - Level 3 in quote
> - Back to level 1

### Table in List

- Here's a table:

  | A | B |
  |---|---|
  | 1 | 2 |

---

## 30. LLM-Specific Patterns

### Streaming Considerations

Unclosed code block (handled gracefully):

```python
# This might be streaming
def partial_function():
    pass

Unclosed emphasis: **this is bold but

### Artifact Syntax

<antThinking>
Internal reasoning that might appear in output.
</antThinking>

---

## 31. Performance Test Section

This section contains repeated content for performance testing.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. **Bold text** and *italic text* mixed together. Here's some `inline code` and a [link](https://example.com).

> Blockquote with **formatting** and `code`.

- List item 1 with **bold**
- List item 2 with *italic*
- List item 3 with `code`
- List item 4 with [link](url)

| Col A | Col B | Col C |
|-------|-------|-------|
| **B** | *I*   | `C`   |
| [L]() | ~~S~~ | ^S^   |

---

## Summary

This file tests:

1. ATX Headings (6 levels)
2. Setext Headings (2 levels)
3. Paragraphs and line breaks
4. Blockquotes (simple and nested)
5. Unordered lists (-, *, +)
6. Ordered lists (., ))
7. Task lists
8. Indented code blocks
9. Fenced code blocks
10. Thematic breaks
11. Tables with alignment
12. Emphasis (italic)
13. Strong (bold)
14. Combined emphasis
15. Strikethrough
16. Code spans
17. Inline links
18. Reference links
19. Images
20. Autolinks
21. HTML blocks
22. Footnotes
23. Definition lists
24. Abbreviations
25. Subscript/Superscript
26. Highlighting
27. Math (inline and block)
28. Escape sequences
29. Entity references
30. Edge cases
31. Complex nesting
32. LLM patterns

---

*End of test file*
