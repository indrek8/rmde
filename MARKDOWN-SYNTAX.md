# Markdown Syntax Reference

**Version:** 1.0
**Last Updated:** 2025-12-08
**Purpose:** Single source of truth for RMDE Markdown parser implementation, testing, and documentation

This document comprehensively documents Markdown syntax based on:
- [CommonMark Spec v0.31.2](https://spec.commonmark.org/) (2024-01-28)
- [GitHub Flavored Markdown (GFM)](https://github.github.com/gfm/)
- [PHP Markdown Extra](https://michelf.ca/projects/php-markdown/extra/)
- [Pandoc Markdown Extensions](https://pandoc.org/MANUAL.html)
- [Extended Syntax Guide](https://www.markdownguide.org/extended-syntax/)
- Real-world usage patterns from Claude AI, ChatGPT, and modern editors

---

## Table of Contents

1. [Principles and Parsing Rules](#principles-and-parsing-rules)
2. [Block Elements](#block-elements)
   - [Headings](#headings)
   - [Paragraphs](#paragraphs)
   - [Blockquotes](#blockquotes)
   - [Lists](#lists)
   - [Code Blocks](#code-blocks)
   - [Thematic Breaks](#thematic-breaks)
   - [Tables](#tables)
   - [HTML Blocks](#html-blocks)
3. [Inline Elements](#inline-elements)
   - [Emphasis and Strong](#emphasis-and-strong)
   - [Code Spans](#code-spans)
   - [Links](#links)
   - [Images](#images)
   - [Autolinks](#autolinks)
   - [Line Breaks](#line-breaks)
4. [Extended Syntax](#extended-syntax)
   - [Footnotes](#footnotes)
   - [Definition Lists](#definition-lists)
   - [Abbreviations](#abbreviations)
   - [Subscript and Superscript](#subscript-and-superscript)
   - [Highlighting](#highlighting)
   - [Math](#math)
5. [Special Characters and Escaping](#special-characters-and-escaping)
6. [Whitespace and Indentation](#whitespace-and-indentation)
7. [LLM-Specific Patterns](#llm-specific-patterns)
8. [Edge Cases and Ambiguities](#edge-cases-and-ambiguities)
9. [Implementation Notes](#implementation-notes)

---

## Principles and Parsing Rules

### Core Principles

1. **Two-Phase Parsing**: Block structure first, then inline elements
2. **Precedence**: Block structure takes precedence over inline markers
3. **Lazy Continuation**: Some elements (blockquotes) allow lazy continuation
4. **No Backtracking**: Parser should not require backtracking for performance
5. **Unicode-Aware**: All valid Unicode code points allowed (except U+0000 → U+FFFD)

### Parsing Order

```
Document
├── Block Structure (phase 1)
│   ├── Blank lines separate blocks
│   ├── Indentation determines nesting
│   └── Markers determine block type
└── Inline Elements (phase 2)
    ├── Process within block content
    ├── Respect code spans (no processing inside)
    └── Handle nested delimiters
```

### Whitespace Rules

- **Tab expansion**: Tabs treated as spaces with tab stop of 4 in block contexts
- **Leading spaces**: Up to 3 spaces allowed before most block markers (4+ = code block)
- **Trailing spaces**: Generally ignored except for hard line breaks (2+ spaces)
- **Blank lines**: Zero characters or only spaces/tabs + line ending

---

## Block Elements

### Headings

#### ATX Headings (# Syntax)

**Standard:** CommonMark

**Syntax:**
```markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6
```

**Rules:**
- 1-6 `#` characters followed by space/tab
- Optional closing `#` sequence (any number), preceded by space/tab
- Up to 3 spaces indentation allowed before opening `#`
- 4+ spaces indentation = code block instead
- Space/tab required after opening `#` (except empty heading)

**Valid:**
```markdown
# Heading
#Heading with optional space
# Heading #
# Heading ##########
   ### Heading with 3 spaces indent
```

**Invalid:**
```markdown
####### Too many hashes (7+)
#5 bolt (no space after #)
    # Four spaces = code block
```

**Output:**
```html
<h1>Heading 1</h1>
<h2>Heading 2</h2>
<!-- etc -->
```

---

#### Setext Headings (Underline Syntax)

**Standard:** CommonMark

**Syntax:**
```markdown
Heading 1
=========

Heading 2
---------
```

**Rules:**
- Text line(s) followed by underline of `=` (level 1) or `-` (level 2)
- Underline must have ≤3 spaces indentation
- Any number of `=` or `-` characters
- Cannot interrupt paragraphs (blank line required)
- Heading text can span multiple lines but no blank lines within
- Takes precedence over thematic breaks

**Valid:**
```markdown
Foo *bar*
=========

Multi
line
heading
---
```

**Invalid:**
```markdown
Foo
Bar
---
(This is a thematic break, not heading - interrupted paragraph)
```

**Edge Cases:**
- Empty setext heading not allowed
- Setext underline with internal spaces/tabs invalid
- `---` alone on line = thematic break, not heading

---

### Paragraphs

**Standard:** CommonMark

**Syntax:**
```markdown
This is a paragraph.

This is another paragraph.
```

**Rules:**
- One or more lines of text not parsed as other block types
- Separated by blank lines
- Hard line breaks: 2+ trailing spaces + newline, or backslash + newline
- Soft line breaks: Single newline → space in output

**Continuation:**
```markdown
This is one
paragraph with a
soft line break.

This is separate.
```

**Output:**
```html
<p>This is one paragraph with a soft line break.</p>
<p>This is separate.</p>
```

---

### Blockquotes

**Standard:** CommonMark

**Syntax:**
```markdown
> This is a blockquote.
> Multiple lines.

> Lazy continuation
works here too.
```

**Rules:**
- `>` optionally followed by space, preceding each line
- Up to 3 spaces indentation before `>`
- Lazy continuation: `>` can be omitted on paragraph continuation lines
- Cannot use lazy continuation for non-paragraph blocks (code, lists, headings)
- Can contain any block elements

**Nesting:**
```markdown
> Level 1
> > Level 2
> > > Level 3
```

**Complex Content:**
```markdown
> # Heading in quote
>
> - List item
> - Another item
>
> ```
> code block
> ```
```

**Edge Cases:**
```markdown
> foo
---
(The --- is outside quote, not setext underline)

> foo
> ---
(The --- is inside quote, setext underline)
```

**Output:**
```html
<blockquote>
<p>This is a blockquote.</p>
</blockquote>
```

---

### Lists

#### Unordered Lists

**Standard:** CommonMark

**Syntax:**
```markdown
- Item 1
- Item 2
- Item 3

* Also valid
* With asterisks

+ Or plus signs
+ Work too
```

**Rules:**
- Markers: `-`, `+`, or `*` followed by 1-4 spaces
- At least one space after marker required
- Marker width + spaces determine indentation for continuation
- Can contain any block elements
- List is tight (no `<p>` tags) if no blank lines between items
- List is loose (with `<p>` tags) if blank lines present

**Nesting:**
```markdown
- Item 1
  - Nested item (2 spaces indent)
  - Another nested
- Item 2
```

**Multiple Paragraphs:**
```markdown
- First paragraph

  Second paragraph (indented to align)

- Another item
```

**Complex Content:**
```markdown
- Item with code

  ```
  code block
  ```

- Item with blockquote

  > quote
```

**Invalid:**
```markdown
-No space after marker
```

---

#### Ordered Lists

**Standard:** CommonMark

**Syntax:**
```markdown
1. First item
2. Second item
3. Third item

1) Parentheses work too
2) Like this
```

**Rules:**
- 1-9 digits followed by `.` or `)`, then 1-4 spaces
- Start number determines first item's number in output
- Subsequent numbers ignored (all increment by 1)
- Cannot be negative (e.g., `-1.` = paragraph)
- 10+ digits not recognized as list marker
- First item interrupting paragraph must start with 1

**Start Number:**
```markdown
5. Start at 5
6. Auto-increments
7. Regardless of written number
```

**Output:**
```html
<ol start="5">
<li>Start at 5</li>
<li>Auto-increments</li>
<li>Regardless of written number</li>
</ol>
```

**Edge Cases:**
```markdown
0. Zero is valid start
00. Leading zeros allowed (start=0)
003. Also valid (start=3)
```

---

#### Task Lists (GFM Extension)

**Standard:** GitHub Flavored Markdown

**Syntax:**
```markdown
- [ ] Unchecked task
- [x] Checked task
- [X] Also checked (capital X)
- [ ] With **formatting**
```

**Rules:**
- Must be list item with `[ ]` or `[x]`/`[X]` at start of content
- Exactly one space between brackets for unchecked
- Exactly one `x` or `X` for checked
- Space after closing bracket required
- Can combine with nested lists

**Nesting:**
```markdown
- [x] Parent task
  - [ ] Subtask 1
  - [x] Subtask 2
```

**Output:**
```html
<ul>
<li><input type="checkbox" disabled> Unchecked task</li>
<li><input type="checkbox" checked disabled> Checked task</li>
</ul>
```

**Invalid:**
```markdown
- [  ] Two spaces
- [] No space
- [o] Wrong character
```

---

### Code Blocks

#### Indented Code Blocks

**Standard:** CommonMark

**Syntax:**
```markdown
    Code line 1
    Code line 2
    Code line 3
```

**Rules:**
- 4+ spaces indentation on each line
- Cannot interrupt paragraphs (blank line required before)
- Removes exactly 4 spaces from each line
- Blank lines preserved
- Trailing spaces included
- First line can have 5+ spaces (extras preserved)

**Example:**
```markdown
Regular text.

    function hello() {
        console.log("Hello");
    }

More text.
```

**Output:**
```html
<pre><code>function hello() {
    console.log("Hello");
}
</code></pre>
```

---

#### Fenced Code Blocks

**Standard:** CommonMark

**Syntax:**
````markdown
```
Code without language
```

```javascript
const greeting = "Hello";
console.log(greeting);
```

~~~python
def hello():
    print("Hello")
~~~
````

**Rules:**
- 3+ backticks or tildes, optionally preceded by up to 3 spaces
- Backtick fences cannot contain backticks in info string
- Tilde fences can contain backticks in info string
- Info string: language name (first word) plus optional metadata
- Closing fence must use same character, ≥ length of opening
- Closing fence must have ≤3 spaces indentation
- Can interrupt paragraphs
- Content indented up to N spaces removed (N = opening fence indent)

**Info String Examples:**
````markdown
```javascript
code
```

```js {highlight: "1,3-5"}
line 1
line 2
```

```rust,no_run
// Metadata after comma
```
````

**Edge Cases:**
````markdown
````js
Four backticks opening
```
Three backticks do not close
````
Needs four backticks to close
````

```js Info string before closing not allowed ```
code
```

~~~~javascript
Tildes can be longer than 3
~~~~
````

**Output:**
```html
<pre><code class="language-javascript">const greeting = "Hello";
console.log(greeting);
</code></pre>
```

---

### Thematic Breaks (Horizontal Rules)

**Standard:** CommonMark

**Syntax:**
```markdown
---

***

___

* * *

- - -

_______________
```

**Rules:**
- 3+ matching `-`, `_`, or `*` characters
- Optional spaces/tabs between characters
- Up to 3 spaces indentation allowed
- Cannot mix characters
- No other content on line
- Can interrupt paragraphs

**Valid:**
```markdown
---
***
___
* * *
- - -
  ***  (with spaces)
```

**Invalid:**
```markdown
-- (only 2)
*-* (mixed characters)
    --- (4 spaces = code block)
text --- (other content)
```

**Output:**
```html
<hr>
```

---

### Tables

**Standard:** GitHub Flavored Markdown

**Syntax:**
```markdown
| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |
```

**Rules:**
- Header row followed by delimiter row with hyphens
- Cells separated by pipe `|` characters
- Leading/trailing pipes optional
- Spaces between pipes and content trimmed
- At least 3 hyphens per delimiter cell
- Block-level elements not allowed in cells

**Alignment:**
```markdown
| Left | Center | Right |
|:-----|:------:|------:|
| L1   | C1     | R1    |
| L2   | C2     | R2    |
```

- `:---` = left-aligned (default)
- `:---:` = center-aligned
- `---:` = right-aligned

**Without Leading/Trailing Pipes:**
```markdown
Header 1 | Header 2
---------|----------
Cell 1   | Cell 2
```

**Escaped Pipes:**
```markdown
| Function | Syntax |
|----------|--------|
| Pipe     | `\|`   |
```

**Output:**
```html
<table>
<thead>
<tr><th>Header 1</th><th>Header 2</th></tr>
</thead>
<tbody>
<tr><td>Cell 1</td><td>Cell 2</td></tr>
</tbody>
</table>
```

**Edge Cases:**
- Empty cells allowed: `| | |`
- Varying column counts: extra cells ignored, missing cells empty
- Inline formatting allowed within cells
- No colspan/rowspan support

---

### HTML Blocks

**Standard:** CommonMark

**Categories:**

#### Type 1: Script/Style/Pre/Textarea
```html
<pre>
content
</pre>

<script>
alert('hi');
</script>
```

**Rules:**
- Start: `<pre`, `<script`, `<style`, `<textarea>` (case-insensitive)
- End: Matching closing tag
- Can contain blank lines

---

#### Type 2: Comments
```html
<!-- Comment
spanning multiple
lines -->
```

**Rules:**
- Start: `<!--`
- End: `-->`

---

#### Type 3: Processing Instructions
```html
<?xml version="1.0"?>
```

**Rules:**
- Start: `<?`
- End: `?>`

---

#### Type 4: Declarations
```html
<!DOCTYPE html>
```

**Rules:**
- Start: `<!` + ASCII letter
- End: `>`

---

#### Type 5: CDATA
```html
<![CDATA[
function matchwo(a,b)
{
if (a < b && a < 0) then {
  return 1;
}
]]>
```

**Rules:**
- Start: `<![CDATA[`
- End: `]]>`

---

#### Type 6: Block Tags
```html
<div>
*Markdown not processed here*
</div>

<div>

*Blank line required for markdown*

</div>
```

**Common tags:** address, article, aside, base, basefont, blockquote, body, caption, center, col, colgroup, dd, details, dialog, dir, div, dl, dt, fieldset, figcaption, figure, footer, form, frame, frameset, h1-h6, head, header, hr, html, iframe, legend, li, link, main, menu, menuitem, nav, noframes, ol, optgroup, option, p, param, search, section, summary, table, tbody, td, textarea, tfoot, th, thead, title, tr, track, ul

**Rules:**
- Start: `<tag>` or `</tag>` + space/tab/newline/`>`/`/>`
- End: Blank line
- Cannot contain markdown unless separated by blank lines

---

#### Type 7: Custom Tags
```html
<custom-element>
content
</custom-element>
```

**Rules:**
- Start: Complete open/close tag + newline
- End: Blank line
- **Cannot interrupt paragraphs** (unlike types 1-6)

---

#### General HTML Rules
- Up to 3 spaces indentation before opening tag
- 4+ spaces = code block
- Types 1-5 can contain blank lines
- Types 6-7 end at first blank line
- Raw HTML passed through in most processors
- Security: Some processors disallow certain tags

---

## Inline Elements

### Emphasis and Strong

**Standard:** CommonMark

#### Emphasis (Italic)

**Syntax:**
```markdown
*emphasis*
_emphasis_
```

**Rules:**
- Single `*` or `_` delimiters
- Left-flanking: followed by non-whitespace
- Right-flanking: preceded by non-whitespace
- `_` cannot open emphasis in middle of word
- `*` more flexible than `_`

**Valid:**
```markdown
This is *emphasis* in text.
This_is_fine.
**double**_nested_**emphasis**
```

**Invalid:**
```markdown
a_b_c (underscores in middle of word)
* emphasis* (no matching delimiters)
```

---

#### Strong (Bold)

**Syntax:**
```markdown
**strong**
__strong__
```

**Rules:**
- Double `**` or `__` delimiters
- Same flanking rules as emphasis
- `__` cannot open/close in middle of word

**Valid:**
```markdown
This is **strong** text.
This**is**fine.
**_combined_**
```

---

#### Combined Emphasis

**Syntax:**
```markdown
***bold italic***
**_bold with italic_**
*__italic with bold__*
___bold italic___
```

**Rules:**
- Can nest emphasis and strong
- Innermost delimiter pair closes first
- Cannot cross boundaries improperly

**Complex Examples:**
```markdown
*This is **nested** emphasis*
**Bold with *italic* inside**
***All bold italic***
**Bold _with_ italic word**
```

**Edge Cases:**
```markdown
*foo *bar* baz* → <em>foo <em>bar</em> baz</em>
***foo** bar* → <em><strong>foo</strong> bar</em>
```

---

#### Strikethrough (GFM Extension)

**Standard:** GitHub Flavored Markdown

**Syntax:**
```markdown
~~strikethrough~~
```

**Rules:**
- Double tilde delimiters
- Same flanking rules as emphasis
- Can combine with other formatting

**Examples:**
```markdown
~~This is deleted~~
**~~Bold strikethrough~~**
*~~Italic strikethrough~~*
```

**Output:**
```html
<del>strikethrough</del>
```

---

### Code Spans

**Standard:** CommonMark

**Syntax:**
```markdown
`code`
``code with `backtick` ``
```

**Rules:**
- Backticks as delimiters
- Opening and closing backtick count must match
- Optional space after opening and before closing backtick
- No emphasis, links, or markdown processing inside
- Entity references not processed
- Backslash escapes not processed
- Literal content preserved

**Examples:**
```markdown
`simple code`
`` `backtick` ``
``` ``double`` ```
`var x = 10;`
```

**With Literal Backticks:**
```markdown
`` ` `` → output: `
``` `` ``` → output: ``
`` `foo` `` → output: `foo`
```

**Output:**
```html
<code>simple code</code>
```

---

### Links

#### Inline Links

**Standard:** CommonMark

**Syntax:**
```markdown
[text](url)
[text](url "title")
[text](<url>)
[text](<url> "title")
```

**Rules:**
- Link text in square brackets
- URL in parentheses
- URL can be enclosed in angle brackets
- Optional title in quotes (single, double) or parentheses
- Title must be separated from URL by whitespace
- Backslash escapes work in URL and title
- Empty URL valid: `[text]()`

**Examples:**
```markdown
[Link](https://example.com)
[Link](https://example.com "Title")
[Link](<https://example.com>)
[Link](/path/to/file)
[Link](../relative/path)
[Link](#anchor)
[Link](mailto:email@example.com)
```

**With Title:**
```markdown
[Link](url "Double quotes")
[Link](url 'Single quotes')
[Link](url (Parentheses))
```

**Complex URLs:**
```markdown
[Link](<url with spaces>)
[Link](url\(with\)parens)
[Link](url "title \"with quotes\"")
```

---

#### Reference Links

**Standard:** CommonMark

**Syntax:**
```markdown
[text][label]
[text][]
[label]

[label]: url
[label]: url "title"
```

**Rules:**
- Full reference: `[text][label]`
- Collapsed reference: `[text][]` (uses text as label)
- Shortcut reference: `[label]` (label is both text and reference)
- Label matching case-insensitive
- First matching definition used
- Definition can appear anywhere in document

**Link Definitions:**
```markdown
[id]: https://example.com
[id]: https://example.com "Title"
[id]: <https://example.com>
[id]: https://example.com
  "Multi-line title"
```

**Definition Rules:**
- Label followed by colon
- Optional whitespace
- URL (optionally in angle brackets)
- Optional title on same or next line
- Cannot interrupt paragraph
- Up to 3 spaces indentation allowed
- Blank line not required after definition

**Examples:**
```markdown
See [Google][g] and [MDN][mdn].

[g]: https://google.com
[mdn]: https://developer.mozilla.org "Mozilla Developer Network"
```

**Implicit:**
```markdown
See [Google] and [MDN].

[google]: https://google.com
[mdn]: https://developer.mozilla.org
```

---

### Images

**Standard:** CommonMark

**Syntax:**
```markdown
![alt text](url)
![alt text](url "title")
![alt text][reference]

[reference]: url "title"
```

**Rules:**
- Same as links but with `!` prefix
- Alt text parsed as inlines (except no links inside)
- All link rules apply

**Examples:**
```markdown
![Logo](logo.png)
![Logo](logo.png "Company Logo")
![Logo][logo-ref]

[logo-ref]: logo.png "Company Logo"
```

**Inline with Sizing (non-standard):**
```markdown
![Image](url){width=300}
![Image](url =250x)
<img src="url" width="300">
```

**Output:**
```html
<img src="url" alt="alt text" title="title">
```

---

### Autolinks

**Standard:** CommonMark

**Syntax:**
```markdown
<https://example.com>
<email@example.com>
```

**Rules:**
- URL or email in angle brackets
- URL cannot contain `<`, `>`, or unescaped backslash
- Backslash escapes NOT processed
- Email must contain `@`
- Converted to clickable link

**Examples:**
```markdown
<https://example.com>
<mailto:email@example.com>
<ssh://user@host.com>
<http://example.com/path?query=value>
```

**Output:**
```html
<a href="https://example.com">https://example.com</a>
```

---

#### Extended Autolinks (GFM Extension)

**Standard:** GitHub Flavored Markdown

**Syntax:**
```markdown
https://example.com
www.example.com
user@example.com
```

**Rules:**
- URLs starting with `http://`, `https://`, or `www.` auto-linked
- Emails with `@` auto-linked
- Must be preceded by valid domain
- Trailing punctuation excluded intelligently

**Examples:**
```markdown
Visit https://example.com for more.
Or check www.example.com.
Email: user@example.com
```

**Smart Punctuation:**
```markdown
See https://example.com. → . not part of URL
Check (https://example.com) → ) not part of URL
```

**Output:**
```html
Visit <a href="https://example.com">https://example.com</a> for more.
```

---

### Line Breaks

**Standard:** CommonMark

#### Hard Line Breaks

**Syntax:**
```markdown
Line 1··
Line 2

Line 1\
Line 2
```
(·· represents 2 trailing spaces)

**Rules:**
- Two or more trailing spaces + newline
- Or backslash + newline
- Creates `<br>` tag
- Does not work in code blocks/spans

**Examples:**
```markdown
First line··
Second line

First line\
Second line
```

**Output:**
```html
First line<br>
Second line
```

---

#### Soft Line Breaks

**Rules:**
- Single newline within paragraph → space in output
- No special syntax
- Default behavior

**Example:**
```markdown
This is one
paragraph.
```

**Output:**
```html
<p>This is one paragraph.</p>
```

---

## Extended Syntax

### Footnotes

**Standard:** PHP Markdown Extra, Pandoc, Python-Markdown

**Syntax:**
```markdown
Here is a footnote reference[^1].

[^1]: This is the footnote content.
```

**Rules:**
- Reference: `[^id]` where id is alphanumeric + dash/underscore
- Definition: `[^id]: content`
- Definition can be placed anywhere
- Multiple paragraphs in footnote need indentation
- Footnotes numbered sequentially in output

**Examples:**
```markdown
First reference[^1] and second[^2].

[^1]: First footnote.

[^2]: Second footnote with multiple paragraphs.

    Second paragraph of footnote.
```

**Inline Footnotes (Pandoc):**
```markdown
Here is an inline footnote^[Inline footnote content].
```

**Output:**
```html
<p>Here is a footnote reference<sup id="fnref:1"><a href="#fn:1">1</a></sup>.</p>

<div class="footnotes">
<ol>
<li id="fn:1">
<p>This is the footnote content. <a href="#fnref:1">↩</a></p>
</li>
</ol>
</div>
```

---

### Definition Lists

**Standard:** PHP Markdown Extra, Pandoc

**Syntax:**
```markdown
Term
: Definition 1
: Definition 2

Another term
:   Definition with indentation
    and multiple lines
```

**Rules:**
- Term on its own line
- Definition starts with `:` followed by space or newline + indentation
- Multiple definitions per term allowed
- Blank line between items optional but creates loose list

**Examples:**
```markdown
Apple
: A fruit
: A tech company

Orange
: A citrus fruit
: A color
```

**Compact:**
```markdown
Term 1
: Definition 1
Term 2
: Definition 2
```

**Loose:**
```markdown
Term 1

: Definition 1

Term 2

: Definition 2
```

**Output:**
```html
<dl>
<dt>Term</dt>
<dd>Definition 1</dd>
<dd>Definition 2</dd>
</dl>
```

---

### Abbreviations

**Standard:** PHP Markdown Extra

**Syntax:**
```markdown
The HTML specification is maintained by W3C.

*[HTML]: Hyper Text Markup Language
*[W3C]: World Wide Web Consortium
```

**Rules:**
- Definition: `*[abbr]: expansion`
- Can appear anywhere in document
- First use in text wrapped in `<abbr>` tag
- Case-sensitive matching

**Example:**
```markdown
This is an HTML document.

*[HTML]: HyperText Markup Language
```

**Output:**
```html
<p>This is an <abbr title="HyperText Markup Language">HTML</abbr> document.</p>
```

---

### Subscript and Superscript

**Standard:** Extended (various implementations)

**Syntax:**
```markdown
H~2~O
x^2^ + y^2^ = r^2^

H<sub>2</sub>O
x<sup>2</sup>
```

**Rules:**
- `~text~` for subscript
- `^text^` for superscript
- For complex content, use HTML tags
- No nesting allowed in tilde/caret syntax

**Examples:**
```markdown
Water is H~2~O
Einstein: E=mc^2^
CO~2~ emissions
x^2^+y^2^=z^2^
```

---

### Highlighting

**Standard:** Extended (Obsidian, other editors)

**Syntax:**
```markdown
==highlighted text==

<mark>highlighted text</mark>
```

**Rules:**
- Double equals for highlighting
- HTML `<mark>` tag also works
- Can combine with other formatting

**Examples:**
```markdown
This is ==very important==.
You can ==**bold highlight**==.
```

**Output:**
```html
<mark>highlighted text</mark>
```

---

### Math

**Standard:** Extended (Pandoc, MathJax, KaTeX)

**Inline Math:**
```markdown
$E = mc^2$
\(E = mc^2\)
```

**Block Math:**
```markdown
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

\[
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
\]
```

**Rules:**
- `$...$` for inline math
- `$$...$$` for display math
- `\(...\)` and `\[...\]` alternatives (LaTeX style)
- Content processed by math renderer (MathJax, KaTeX)

**Examples:**
```markdown
The quadratic formula is $x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}$.

$$
\begin{aligned}
\nabla \times \vec{\mathbf{B}} -\, \frac1c\, \frac{\partial\vec{\mathbf{E}}}{\partial t} &= \frac{4\pi}{c}\vec{\mathbf{j}} \\
\nabla \cdot \vec{\mathbf{E}} &= 4 \pi \rho
\end{aligned}
$$
```

---

## Special Characters and Escaping

### Backslash Escapes

**Standard:** CommonMark

**Escapable Characters:**
```
! " # $ % & ' ( ) * + , - . / : ; < = > ? @ [ \ ] ^ _ ` { | } ~
```

**Rules:**
- Backslash before ASCII punctuation = literal character
- Not processed in code blocks/spans
- Not processed in autolinks
- Not processed in raw HTML
- Processed in URLs, link titles, fenced code info strings

**Examples:**
```markdown
\*Not emphasis\*
\[Not a link\]
\`Not code\`
Backslash: \\
```

**In URLs:**
```markdown
[Link](url\(with\)parens)
```

**Output:**
```
*Not emphasis*
[Not a link]
`Not code`
Backslash: \
```

---

### Entity References

**Standard:** CommonMark (HTML5 entities)

**Syntax:**
```markdown
&copy; &mdash; &nbsp;
&#35; &#x22;
```

**Rules:**
- Named entities: `&name;` (HTML5 entity list)
- Decimal numeric: `&#number;` (1-7 digits)
- Hexadecimal: `&#xHH;` or `&#XHH;` (1-6 digits)
- Not recognized in code blocks/spans
- Cannot replace structural characters

**Common Entities:**
```markdown
&copy; → ©
&mdash; → —
&nbsp; → (non-breaking space)
&lt; → <
&gt; → >
&amp; → &
&quot; → "
```

**Numeric:**
```markdown
&#35; → #
&#x23; → #
&#169; → ©
```

---

### Special Characters

**Reserved/Special:**
- `*` `_` - Emphasis
- `**` `__` - Strong
- `#` - Headings
- `>` - Blockquotes
- `-` `+` `*` - List markers
- `` ` `` - Code
- `[` `]` - Links
- `!` - Images
- `<` `>` - HTML/autolinks
- `\` - Escapes

**To Display Literally:**
- Use backslash escape: `\*`
- Use HTML entity: `&ast;`
- Place in code span: `` `*` ``

---

## Whitespace and Indentation

### Indentation Rules

| Context | Meaning |
|---------|---------|
| 0-3 spaces | Block markers (headings, lists, quotes, etc.) |
| 4+ spaces | Indented code block |
| 0-3 before `>` | Valid blockquote marker |
| 4+ before `>` | Code block containing `>` |
| 0-3 before fence | Valid fence marker |
| List content | Continuation indent (marker width + spaces) |

### Tab Handling

**Rules:**
- Tab = 4 spaces in block structure
- Tab stop calculated from line start
- Literal tabs preserved in code blocks
- Tabs expanded when determining indentation

**Example:**
```
>→foo
```
(tab treated as 3 spaces to reach tab stop at column 4)

### Blank Lines

**Definition:**
- Line with zero characters
- Or line with only spaces/tabs
- Followed by line ending

**Effect:**
- Separate block elements
- Determine list tight/loose
- End certain blocks (thematic breaks, HTML blocks type 6-7)
- Ignored at document start/end

---

## LLM-Specific Patterns

### Claude and ChatGPT Output

**Common Patterns:**

1. **Code Blocks with Language Tags**
   ````markdown
   ```python
   def hello():
       print("Hello, world!")
   ```
   ````

2. **Streaming Output Considerations**
   - Partial code blocks: ` ```python` without closing
   - Incomplete emphasis: `**bold text` without closing `**`
   - Need incremental parsing for real-time display

3. **Artifact Syntax (Claude-specific)**
   ```markdown
   <antThinking>
   Internal reasoning...
   </antThinking>
   ```

4. **Common Metadata in Code Blocks**
   ````markdown
   ```javascript {highlight: "1-3,5"}
   line 1
   line 2
   line 3
   line 4
   line 5
   ```
   ````

### Rendering Considerations

**For LLM Output Streaming:**

1. **Incomplete Syntax**
   - Handle unclosed delimiters gracefully
   - Don't crash on partial markdown
   - Progressive enhancement as more text arrives

2. **Performance**
   - Incremental parsing preferred
   - Avoid re-parsing entire document on each chunk
   - Cache parse trees when possible

3. **Common Artifacts**
   ```markdown
   <antThinking>...</antThinking>
   <antMeta>...</antMeta>
   [thinking]...[/thinking]
   ```

---

## Edge Cases and Ambiguities

### Emphasis and Strong

**Intraword Emphasis:**
```markdown
foo*bar*baz → foo<em>bar</em>baz
foo_bar_baz → foo_bar_baz (no emphasis with underscores)
**foo**bar → <strong>foo</strong>bar
__foo__bar → __foo__bar (no strong with underscores)
```

**Nested:**
```markdown
*foo **bar** baz* → <em>foo <strong>bar</strong> baz</em>
**foo *bar* baz** → <strong>foo <em>bar</em> baz</strong>
***foo*** → <em><strong>foo</strong></em>
```

**Adjacent:**
```markdown
**foo** **bar** → <strong>foo</strong> <strong>bar</strong>
*foo**bar**baz* → <em>foo<strong>bar</strong>baz</em>
```

---

### List and Blockquote Interaction

**List Inside Quote:**
```markdown
> - Item 1
> - Item 2
```

**Quote Inside List:**
```markdown
- Item 1
  > Quote in item
```

**Ambiguous:**
```markdown
> - > Is this nested quote or quote inside list item?
```
→ Quote inside list item inside quote

---

### Setext vs Thematic Break

**Setext Heading:**
```markdown
Foo
---
```

**Thematic Break:**
```markdown
Bar

---
```

**Rule:** Setext heading cannot interrupt paragraph (needs blank line before).

---

### Link vs Emphasis

**Link with Emphasis:**
```markdown
[*text*](url) → Link with emphasis
*[text](url)* → Emphasis with link
```

**Precedence:**
```markdown
[foo](*bar*) → Link to *bar* (not emphasis in URL)
```

---

### Code Span vs Code Block

**Inline:**
```markdown
`code`
```

**Block:**
````markdown
    code

```
code
```
````

**Ambiguous:**
````markdown
    `code` (4 spaces) → code block containing `code`
`    code` (backtick then spaces) → code span
````

---

### HTML vs Markdown

**Block HTML (no markdown):**
```html
<div>
*Not emphasis*
</div>
```

**With Blank Lines (markdown processed):**
```html
<div>

*Emphasis works here*

</div>
```

**Inline HTML:**
```markdown
This <em>works</em> with *markdown*.
```

---

## Implementation Notes

### Parsing Strategy

1. **Block Structure First**
   - Identify block boundaries
   - Determine block types
   - Handle nesting/containment
   - Build block tree

2. **Inline Processing Second**
   - Process each block's content
   - Handle emphasis delimiters
   - Process links
   - Handle escapes
   - Respect code spans (no processing inside)

### Performance Considerations

1. **Large Files**
   - Incremental parsing preferred
   - Lazy evaluation where possible
   - Cache parse results
   - Limit look-ahead/look-behind

2. **Streaming Content**
   - Handle incomplete syntax
   - Progressive rendering
   - Re-parse only changed regions
   - Maintain parse state

3. **Memory Usage**
   - Use rope data structure for large documents
   - Don't copy content unnecessarily
   - Return byte offsets, not substring copies

### Tree-Sitter Integration

**Two-Grammar Approach:**

1. **Block Grammar**
   - Parse document structure
   - Identify block types
   - Determine inline content ranges

2. **Inline Grammar**
   - Parse inline content within blocks
   - Use `ts_parser_set_included_ranges`
   - Process only inline regions

**Incremental Updates:**
- Leverage tree-sitter's incremental parsing
- Pass old tree to next parse
- Re-parse only changed nodes

### Testing Strategy

**Categories:**

1. **CommonMark Spec**
   - 671 examples in v0.31.2
   - Test each example
   - Ensure compliance

2. **GFM Spec**
   - Additional tables, task lists, strikethrough
   - Autolink extensions

3. **Extended Features**
   - Footnotes
   - Definition lists
   - Math blocks

4. **Edge Cases**
   - Nested structures
   - Ambiguous syntax
   - Malformed input

5. **Performance**
   - Large documents (1MB+)
   - Deeply nested structures
   - Streaming/incremental parsing

---

## References

### Primary Standards

- [CommonMark Spec v0.31.2](https://spec.commonmark.org/0.31.2/) (2024-01-28)
- [GitHub Flavored Markdown Spec](https://github.github.com/gfm/)
- [PHP Markdown Extra](https://michelf.ca/projects/php-markdown/extra/)
- [Pandoc User's Manual](https://pandoc.org/MANUAL.html)

### Extended Syntax

- [Markdown Guide - Extended Syntax](https://www.markdownguide.org/extended-syntax/)
- [Python-Markdown Extensions](https://python-markdown.github.io/extensions/)
- [Obsidian Flavored Markdown](https://help.obsidian.md/obsidian-flavored-markdown)

### Parsers and Implementations

- [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- [CommonMark Reference Implementation](https://github.com/commonmark/commonmark-spec)
- [Pandoc vs Multimarkdown](https://github.com/jgm/pandoc/wiki/Pandoc-vs-Multimarkdown)

### LLM Integration

- [Markdown with ChatGPT](https://medium.com/@binarygirl0000/a-complete-guide-to-using-markdown-with-chatgpt-and-ai-models-2a06274db988)
- [llm-ui Code Blocks](https://llm-ui.com/docs/blocks/code/)
- [Rendering Streamed LLM Responses](https://developer.chrome.com/docs/ai/render-llm-responses)

---

## Version History

- **v1.0** (2025-12-08): Initial comprehensive reference
  - CommonMark v0.31.2 coverage
  - GFM extensions
  - Extended syntax (footnotes, definition lists, etc.)
  - LLM-specific patterns (Claude, ChatGPT)
  - Implementation notes for tree-sitter

---

## License

This document is provided as reference for RMDE implementation. Markdown specifications referenced are subject to their respective licenses:
- CommonMark: Creative Commons Attribution-ShareAlike 4.0 International
- GFM: Public domain
- Extended features: Various (see individual specifications)
