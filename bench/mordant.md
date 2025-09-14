---
author: Connor Duncan
copyright: Copyright 2025, Connor Duncan
date: 2025-04-23
title: mordant: Actually Static Syntax Highlighting for Markdown
---
<br/>
TL;DR---I wrote a syntax highlighter for markdown files that produces very customizable output.
Check it out on [github](https://github.com/ctdunc/mordant)!


- [What is mordant?](#what-is-mordant)
- [Why not just use highlight.js or Hugo like a normal person?](#why-not-just-use-highlightjs-or-hugo-like-a-normal-person)
- [Look at these cool examples](#look-at-these-cool-examples)
  - [SQL Literals in Rust](#sql-literals-in-rust)
  - [Scheme inside of mordant.toml](#scheme-inside-of-mordanttoml)
  - [Regex in JavaScript in Python](#regex-in-javascript-in-python)
- [How can I try mordant for myself?](#how-can-i-try-mordant-for-myself)
  - [Installation](#installation)
  - [Language Setup](#language-setup)
    - [From Source](#from-source)
    - [Adding a new builtin language](#adding-a-new-builtin-language)
  - [Styling](#styling)
  - [A dirt simple build pipeline](#a-dirt-simple-build-pipeline)
- [Wrapping up](#wrapping-up)

# What is mordant?
mordant is a program that takes fenced code blocks in Markdown files, and uses tree-sitter to replace them with
inline html containing enough information to get editor-quality syntax highlighting, with *only css*.
Absolutely no JavaScript required! Because mordant uses tree-sitter under the hood, it supports 
[*language injections*](https://tree-sitter.github.io/tree-sitter/3-syntax-highlighting.html#language-injection).
All of the code blocks in this post were highlighted using mordant.

Some syntax highlighters do support a limited form of language injection. For example, on GitHub,
the following code block will highlight the contents of `<script>, <style>` tags as JavaScript and css respectively.
For example, here's a side-by-side comparison of mordant (top) and github's (bottom) highlighting for a fenced `html` code
block.

```html
<script>
  console.log('hello world');
</script>
<style>
  .my-class {
    color: #000;
  }
</style>
```

***

![github highlight example](/static/res/github-highlight.png)

<details box-="square">
<summary> view generated html </summary>

```html
<pre><code><span class="code-tag.delimiter">&lt;</span><span class="code-tag">script</span><span class="code-tag.delimiter">&gt;</span>
  <span class="code-variable.builtin">console</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">log</span><span class="code-punctuation.bracket">(</span><span class="code-string">'hello world'</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
<span class="code-tag.delimiter">&lt;/</span><span class="code-tag">script</span><span class="code-tag.delimiter">&gt;</span>
<span class="code-tag.delimiter">&lt;</span><span class="code-tag">style</span><span class="code-tag.delimiter">&gt;</span>
  <span class="code-punctuation.delimiter">.</span><span class="code-type">my-class</span> <span class="code-punctuation.bracket">{</span>
    <span class="code-property">color</span><span class="code-punctuation.delimiter">:</span> <span class="code-string"><span class="code-punctuation.delimiter">#</span>000</span><span class="code-punctuation.delimiter">;</span>
  <span class="code-punctuation.bracket">}</span>
<span class="code-tag.delimiter">&lt;/</span><span class="code-tag">style</span><span class="code-tag.delimiter">&gt;</span>
```

</details>

The name mordant derives from a class of substances used to bind dyes to fabric. Like it's 
[eponym](https://en.wikipedia.org/wiki/Mordant), mordant binds beautiful colors to your fenced code blocks. 

Special thanks to the maintainers of [topiary](https://github.com/tweag/topiary) for the inspiration (and direction)
in creating this project. Highly recommend checking topiary out! It's a really straightforward way to write
code formatters for languages that don't already have them. I've used it to write formatters for some proprietary 
DSLs that we use at work, and it was a breeze to get started with.

# Why not just use highlight.js or Hugo like a normal person?

The joke answer is that I have [suckless](https://suckless.org/) brainrot, and consequently want to keep
my website free of JavaScript. Also, writing something in Rust I'd actually use seemed like a fun project. I was right.

The better answer is that highlight.js is a great library, with support for a *ton* of languages, but doesn't look
very easy to extend. Coupled with my inherent dislike of JavaScript for blogging and the fact that I've already invested a decent
chunk of time in learning tree-sitter, it just didn't seem like a good fit for my website. 
Since I plan to continue my [series of posts](./dash-clientside-lsp.html) describing tools to make [Dash](https://github.com/plotly/dash) 
more usable when writing performant, interactive applications (beyond just dashboarding), it seemed natural
to broaden my tree-sitter horizons beyond neovim.

# Look at these cool examples
I'm just going to flex here. Please enjoy the pretty colors, do not focus on whether or not 
this code will actually compile/run. It probably won't, it's just here to look pretty.

## SQL Literals in Rust
An example taken from the [sqlx docs](https://docs.rs/sqlx/latest/sqlx/macro.query.html).
I could definitely be smarter about how I wrote this query, but this gets the idea across.

```rust
// let mut conn = <impl sqlx::Executor>;
let account = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
    .fetch_one(&mut conn)
    .await?;

// anonymous struct has `#[derive(Debug)]` for convenience
println!("{account:?}");
println!("{}: {}", account.id, account.name);
```
<details box-="square">
<summary> mordant.toml used for these highlights </summary>

Note the inline injection of scheme!
```toml
# minimal mordant.toml
[languages.sql]
name= "sql"

[languages.rust]
name = "rust"
injections_query = { query = '''
(macro_invocation 
  macro: (scoped_identifier
	name: (identifier) @macro_name
  ) 
  (token_tree
	(string_literal 
	(string_content) @injection.content
  ))
  (#eq? @macro_name "query")
  (#set! injection.language "sql")
)
''' }
```

</details>

<details box-="square">
  <summary> generated html </summary>

```html
<pre><code><span class="code-comment">// let mut conn = &lt;impl sqlx::Executor&gt;;</span>
<span class="code-keyword">let</span> account = sqlx<span class="code-punctuation.delimiter">::</span>query!<span class="code-punctuation.bracket">(</span><span class="code-string">"<span class="code-keyword">select</span> <span class="code-punctuation.bracket">(</span><span class="code-string">1</span><span class="code-punctuation.bracket">)</span> <span class="code-keyword">as</span> <span class="code-variable">id</span><span class="code-punctuation.delimiter">,</span> <span class="code-string">'Herp Derpinson'</span> <span class="code-keyword">as</span> <span class="code-variable">name</span>"</span><span class="code-punctuation.bracket">)</span>
    <span class="code-punctuation.delimiter">.</span><span class="code-function.method">fetch_one</span><span class="code-punctuation.bracket">(</span><span class="code-operator">&amp;</span><span class="code-keyword">mut</span> conn<span class="code-punctuation.bracket">)</span>
    <span class="code-punctuation.delimiter">.</span><span class="code-keyword">await</span>?<span class="code-punctuation.delimiter">;</span>

<span class="code-comment">// anonymous struct has `#[derive(Debug)]` for convenience</span>
<span class="code-function.macro">println</span><span class="code-function.macro">!</span><span class="code-punctuation.bracket">(</span><span class="code-string">"{account:?}"</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
<span class="code-function.macro">println</span><span class="code-function.macro">!</span><span class="code-punctuation.bracket">(</span><span class="code-string">"{}: {}"</span><span class="code-punctuation.delimiter">,</span> account<span class="code-punctuation.delimiter">.</span>id<span class="code-punctuation.delimiter">,</span> account<span class="code-punctuation.delimiter">.</span>name<span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>

</code></pre>
```

</details>

## Scheme inside of mordant.toml
I know that every query key will correspond to a scheme query.
Note that I am using the source files distributed with nvim-treesitter, since there 
is no (updated) Rust crate for scheme or toml. YMMV if you aren't using nvim-treesitter,
but the upstream source of these files is [here](https://github.com/nvim-treesitter/nvim-treesitter/tree/master),
and downloading the source files is not much trouble.

```toml
[languages.toml]
name = "toml"
language = { path = "~/.local/share/nvim/lazy/nvim-treesitter/parser/toml.so" }
highlights_query = { path = "~/.local/share/nvim/lazy/nvim-treesitter/queries/toml/highlights.scm" }
injections_query = { query = '''
(pair 
  (bare_key) @key
  (string) @injection.content
  (#eq? @key "query")
  (#set! injection.language "scheme")
)
''' }

[languages.scheme]
name = "scheme"
language = { path = "~/.local/share/nvim/lazy/nvim-treesitter/parser/scheme.so" }
highlights_query = { path = "~/.local/share/nvim/lazy/nvim-treesitter/queries/scheme/highlights.scm" }
```

<details box-="square">
  <summary> generated html </summary>

```html
<pre><code><span class="code-punctuation.bracket">[</span><span class="code-property">languages</span><span class="code-punctuation.delimiter">.</span><span class="code-property">toml</span><span class="code-punctuation.bracket">]</span>
<span class="code-property">name</span> <span class="code-operator">=</span> <span class="code-string">"toml"</span>
<span class="code-property">language</span> <span class="code-operator">=</span> <span class="code-punctuation.bracket">{</span> <span class="code-property">path</span> <span class="code-operator">=</span> <span class="code-string">"~/.local/share/nvim/lazy/nvim-treesitter/parser/toml.so"</span> <span class="code-punctuation.bracket">}</span>
<span class="code-property">highlights_query</span> <span class="code-operator">=</span> <span class="code-punctuation.bracket">{</span> <span class="code-property">path</span> <span class="code-operator">=</span> <span class="code-string">"~/.local/share/nvim/lazy/nvim-treesitter/queries/toml/highlights.scm"</span> <span class="code-punctuation.bracket">}</span>
<span class="code-property">injections_query</span> <span class="code-operator">=</span> <span class="code-punctuation.bracket">{</span> <span class="code-property">query</span> <span class="code-operator">=</span> <span class="code-string">'''
<span class="code-punctuation.bracket">(</span><span class="code-function">pair</span> 
  <span class="code-punctuation.bracket">(</span><span class="code-function">bare_key</span><span class="code-punctuation.bracket">)</span> <span class="code-variable">@key</span>
  <span class="code-punctuation.bracket">(</span><span class="code-function.builtin">string</span><span class="code-punctuation.bracket">)</span> <span class="code-variable">@injection.content</span>
  <span class="code-punctuation.bracket">(</span>#eq? <span class="code-variable">@key</span> <span class="code-string">"query"</span><span class="code-punctuation.bracket">)</span>
  <span class="code-punctuation.bracket">(</span>#set! <span class="code-variable">injection.language</span> <span class="code-string">"scheme"</span><span class="code-punctuation.bracket">)</span>
<span class="code-punctuation.bracket">)</span>
'''</span> <span class="code-punctuation.bracket">}</span>

<span class="code-punctuation.bracket">[</span><span class="code-property">languages</span><span class="code-punctuation.delimiter">.</span><span class="code-property">scheme</span><span class="code-punctuation.bracket">]</span>
<span class="code-property">name</span> <span class="code-operator">=</span> <span class="code-string">"scheme"</span>
<span class="code-property">language</span> <span class="code-operator">=</span> <span class="code-punctuation.bracket">{</span> <span class="code-property">path</span> <span class="code-operator">=</span> <span class="code-string">"~/.local/share/nvim/lazy/nvim-treesitter/parser/scheme.so"</span> <span class="code-punctuation.bracket">}</span>
<span class="code-property">highlights_query</span> <span class="code-operator">=</span> <span class="code-punctuation.bracket">{</span> <span class="code-property">path</span> <span class="code-operator">=</span> <span class="code-string">"~/.local/share/nvim/lazy/nvim-treesitter/queries/scheme/highlights.scm"</span> <span class="code-punctuation.bracket">}</span>

</code></pre>
```

</details>

## Regex in JavaScript in Python
Here's an example where we use the injections query for [Dash clientside_callbacks](./dash-clientside-treesitter.html),
along with the query for regular expressions within JavaScript to demonstrate multi-level injections within a single code block.

```python
from dash import Input, Output, clientside_callback
# test password strength
clientside_callback(
  """
(password) => {
  // is my password strong?
  const is_strong = /(?=(.*[0-9]))(?=.*[\!@#$%^&*()\\[\]{}\-_+=~`|:;"'<>,./?])(?=.*[a-z])(?=(.*[A-Z]))(?=(.*)).{8,}/;
  return is_strong.test(password);
}
  """,
  Output("is-strong", "data"),
  Input("test-is-strong", "value"),
)
```

<details box-="square">
  <summary> mordant.toml </summary>

```toml
[languages.python]
name = "python"
language = { path = "~/.local/share/nvim/lazy/nvim-treesitter/parser/python.so", symbol_name = "tree_sitter_python" }
# there's currently a bug where multiple captures on the same node aren't processed corrrectly, so
# I removed all of the @spell captures & moved it.
highlights_query = { path = "./_mordant/queries/python/highlights.scm" }
injections_query = { query = '''
(call
  (identifier) @name (#eq? @name clientside_callback) 
  (argument_list 
    (
      (string (string_content) @injection.content 
        (#set! injection.include-children)
        (#set! injection.language "javascript")
      )
    )
  )
)
''' }

[languages.regex]
name = "regex"
language = { path = "~/.local/share/nvim/lazy/nvim-treesitter/parser/regex.so" }
highlights_query = { path = "~/.local/share/nvim/lazy/nvim-treesitter/queries/regex/highlights.scm" }

[languages.javascript]
name = "javascript"
# this has the regex injection built-in. currently, mordant does not support
# nvim-treesitter-style ;extends modelines.
injections_query = { path = "~/.local/share/nvim/lazy/nvim-treesitter/queries/ecma/injections.scm" }
```

</details>

<details box-="square">
  <summary> generated html </summary>

```html
<pre><code><span class="code-keyword.import">from</span> <span class="code-module">dash</span> <span class="code-keyword.import">import</span> <span class="code-variable">Input</span><span class="code-punctuation.delimiter">,</span> <span class="code-variable">Output</span><span class="code-punctuation.delimiter">,</span> <span class="code-variable">clientside_callback</span>
<span class="code-comment"># test password strength</span>
<span class="code-function.call">clientside_callback</span><span class="code-punctuation.bracket">(</span>
  <span class="code-string">"""
<span class="code-punctuation.bracket">(</span><span class="code-variable">password</span><span class="code-punctuation.bracket">)</span> <span class="code-operator">=&gt;</span> <span class="code-punctuation.bracket">{</span>
  <span class="code-comment">// is my password strong?</span>
  <span class="code-keyword">const</span> <span class="code-variable">is_strong</span> <span class="code-operator">=</span> <span class="code-string.special"><span class="code-operator">/</span><span class="code-punctuation.bracket">(?</span><span class="code-operator">=</span><span class="code-punctuation.bracket">(</span><span class="code-variable.builtin">.</span><span class="code-operator">*</span><span class="code-punctuation.bracket">[</span><span class="code-constant">0</span><span class="code-operator">-</span><span class="code-constant">9</span><span class="code-punctuation.bracket">]</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">(?</span><span class="code-operator">=</span><span class="code-variable.builtin">.</span><span class="code-operator">*</span><span class="code-punctuation.bracket">[</span><span class="code-string.regexp">\!</span><span class="code-constant">@</span><span class="code-constant">#</span><span class="code-constant">$</span><span class="code-constant">%</span><span class="code-constant">^</span><span class="code-constant">&amp;</span><span class="code-constant">*</span><span class="code-constant">(</span><span class="code-constant">)</span><span class="code-string.regexp">\\</span><span class="code-constant">[</span><span class="code-string.regexp">\]</span><span class="code-constant">{</span><span class="code-constant">}</span><span class="code-string.regexp">\-</span><span class="code-constant">_</span><span class="code-constant">+</span><span class="code-constant">=</span><span class="code-constant">~</span><span class="code-constant">`</span><span class="code-constant">|</span><span class="code-constant">:</span><span class="code-constant">;</span><span class="code-constant">"</span><span class="code-constant">'</span><span class="code-constant">&lt;</span><span class="code-constant">&gt;</span><span class="code-constant">,</span><span class="code-constant">.</span><span class="code-constant">/</span><span class="code-constant">?</span><span class="code-punctuation.bracket">]</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">(?</span><span class="code-operator">=</span><span class="code-variable.builtin">.</span><span class="code-operator">*</span><span class="code-punctuation.bracket">[</span><span class="code-constant">a</span><span class="code-operator">-</span><span class="code-constant">z</span><span class="code-punctuation.bracket">]</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">(?</span><span class="code-operator">=</span><span class="code-punctuation.bracket">(</span><span class="code-variable.builtin">.</span><span class="code-operator">*</span><span class="code-punctuation.bracket">[</span><span class="code-constant">A</span><span class="code-operator">-</span><span class="code-constant">Z</span><span class="code-punctuation.bracket">]</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">(?</span><span class="code-operator">=</span><span class="code-punctuation.bracket">(</span><span class="code-variable.builtin">.</span><span class="code-operator">*</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.bracket">)</span><span class="code-variable.builtin">.</span><span class="code-punctuation.bracket">{</span><span class="code-number">8</span><span class="code-punctuation.delimiter">,</span><span class="code-punctuation.bracket">}</span><span class="code-operator">/</span></span><span class="code-punctuation.delimiter">;</span>
  <span class="code-keyword">return</span> <span class="code-variable">is_strong</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">test</span><span class="code-punctuation.bracket">(</span><span class="code-variable">password</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
<span class="code-punctuation.bracket">}</span>
  """</span><span class="code-punctuation.delimiter">,</span>
  <span class="code-function.call">Output</span><span class="code-punctuation.bracket">(</span><span class="code-string">"is-strong"</span><span class="code-punctuation.delimiter">,</span> <span class="code-string">"data"</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">,</span>
  <span class="code-function.call">Input</span><span class="code-punctuation.bracket">(</span><span class="code-string">"test-is-strong"</span><span class="code-punctuation.delimiter">,</span> <span class="code-string">"value"</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">,</span>
<span class="code-punctuation.bracket">)</span>

</code></pre>
```

</details>

# How can I try mordant for myself?
## Installation
Currently, mordant is only available [on github](https://github.com/ctdunc/mordant). After cloning the repository,
you can install it with support for all built-in languages:

```
$ cargo install --path /path/to/mordant --features=languages_all
```

If you'd prefer only some languages, individual languages are gated behind the obvious feature flag---e.g. to install
Rust and SQL:

```
$ cargo install --path /path/to/mordant --features=rust,sql
```

## Language Setup
The [README](https://github.com/ctdunc/mordant?tab=readme-ov-file#adding-new-languages) will contain the most up-to-date
documentation on configuring languages.
If you would rather read it here (I think my website is prettier), I'll repeat 
my README advice (as of April 23, 2025).

### From Source
If you are looking to work with a proprietary language that you don't want to contribute upstream,
or need to work with a language that does not have an existing rust crate, it is also possible to include languages
directly from `.so` files.
For example, consider adding a language: `foolang`.
To add it from source, I can add the following to my `mordant.toml`:

```toml
# nvim-treesitter-location = "..." # this is optional, and currently not working. free to ignore.
[languages.foolang]
# name is used to match @injection.language captures to the actual tree-sitter grammar.
# it is required.
name = "foolang" 
# symbol_name is optional. If not provided, default to `tree_sitter_{name}`.
language = { path = "/path/to/source/file.so", symbol_name = "tree_sitter_foo_foolang" } 
# all queries are configured with the same options. setting path will look for the query file, 
# setting query will use that text directly.
highlights_query = { path = "/path/to/queries/highlights.scm" }

# injections_query and locals_query are optional.
injections_query = { query = "(_) . @injection.content (#set! injection.language \"barbazscript\")" }
```

### Adding a new builtin language
Builtin languages are housed at `src/user_config/treesitter_util.rs`, in the functions `get_builtin_highlights`
and `get_builtin_language`. If your desired language already has a rust crate (which many do), you should simply add the `LANGUAGE`
and `HIGHLIGHT_QUERY` from that crate to the match statement, and gate it behind your language feature.

For example, if I wanted to add `foolang` to mordant, I would add the following cases to my match statements in
`src/user_config/treesitter_util.rs`:

```rust
pub fn get_builtin_language(name: &str) -> MordantConfigResult<Language> {
    match name {
        /* ... other languages ... */
        #[cfg(feature="foolang")]
        "foolang" => {
            return Ok(tree_sitter_foolang::LANGUAGE.into());
        }
        /* ... other languages ... */
    }
}
pub fn get_builtin_highlights(name: &str) -> MordantConfigResult<String> {
    match name {
        /* ... other languages ... */
        #[cfg(feature="foolang")]
        "foolang" => {
            return Ok(tree_sitter_foolang::HIGHLIGHTS_QUERY.into());
        }
        /* ... other languages ... */
    }
}
// optionally, add a locals query as well. Not all languages support this,
// so it is not required.
```

Then, add `foolang` as a feature to `Cargo.toml`

```toml
#...

[dependencies]
# ...
tree-sitter-foolang = {version = "*", optional = true}

[features]
# ...
foolang = ["dep:tree-sitter-foolang"]
```

If you find yourself building a language into mordant, *please* consider 
[opening a pull request](https://github.com/ctdunc/mordant/pulls). I'd love to 
build out the roster!

## Styling
mordant does not come with any styling built in, it merely
applies classes to the captured treesitter nodes, so that you can use CSS selectors to style them
as desired.
Highlight names are converted to `class="code-{highlight_name}"` attributes 
on `<span>` tags.
For example a node captured by `@variable` would be wrapped in `<span class="code-variable">{node_contents}</span>`.

<details box-="square">
  <summary> supported highlight names as of April 23, 2025. </summary>
  
  <ul marker-="tree">
    <li>variable</li>
    <li>variable.builtin</li>
    <li>variable.parameter</li>
    <li>variable.parameter.builtin</li>
    <li>variable.member</li>
    <li>constant</li>
    <li>constant.builtin</li>
    <li>constant.macro</li>
    <li>module</li>
    <li>module.builtin</li>
    <li>label</li>
    <li>string</li>
    <li>string.documentation</li>
    <li>string.regexp</li>
    <li>string.escape</li>
    <li>string.special</li>
    <li>string.special.symbol</li>
    <li>string.special.path</li>
    <li>string.special.url</li>
    <li>character</li>
    <li>character.special</li>
    <li>boolean</li>
    <li>number</li>
    <li>number.float</li>
    <li>type</li>
    <li>type.builtin</li>
    <li>type.definition</li>
    <li>attribute</li>
    <li>attribute.builtin</li>
    <li>property</li>
    <li>function</li>
    <li>function.builtin</li>
    <li>function.call</li>
    <li>function.macro</li>
    <li>function.method</li>
    <li>function.method.call</li>
    <li>constructor</li>
    <li>operator</li>
    <li>keyword</li>
    <li>keyword.coroutine</li>
    <li>keyword.function</li>
    <li>keyword.operator</li>
    <li>keyword.import</li>
    <li>keyword.type</li>
    <li>keyword.modifier</li>
    <li>keyword.repeat</li>
    <li>keyword.return</li>
    <li>keyword.debug</li>
    <li>keyword.exception</li>
    <li>keyword.conditional</li>
    <li>keyword.conditional.ternary</li>
    <li>keyword.directive</li>
    <li>keyword.directive.define</li>
    <li>punctuation.delimiter</li>
    <li>punctuation.bracket</li>
    <li>punctuation.special</li>
    <li>comment</li>
    <li>comment.documentation</li>
    <li>comment.error</li>
    <li>comment.warning</li>
    <li>comment.todo</li>
    <li>comment.note</li>
    <li>markup.strong</li>
    <li>markup.italic</li>
    <li>markup.strikethrough</li>
    <li>markup.underline</li>
    <li>markup.heading</li>
    <li>markup.heading.1</li>
    <li>markup.heading.2</li>
    <li>markup.heading.3</li>
    <li>markup.heading.4</li>
    <li>markup.heading.5</li>
    <li>markup.heading.6</li>
    <li>markup.quote</li>
    <li>markup.math</li>
    <li>markup.link</li>
    <li>markup.link.label</li>
    <li>markup.link.url</li>
    <li>markup.raw</li>
    <li>markup.raw.block</li>
    <li>markup.list</li>
    <li>markup.list.checked</li>
    <li>markup.list.unchecked</li>
    <li>diff.plus</li>
    <li>diff.minus</li>
    <li>diff.delta</li>
    <li>tag</li>
    <li>tag.builtin</li>
    <li>tag.attribute</li>
    <li>tag.delimiter</li>
  </ul>
</details>

<details box-="square">
  <summary> css for the code blocks on this page </summary>



```css
@media (prefers-color-scheme: dark) {
  :root {
    --bg_h: #1d2021;
    --bg:   #282828;
    --bg_s: #32302f;
    --bg1:  #3c3836;
    --bg2:  #504945;
    --bg3:  #665c54;
    --bg4:  #7c6f64;
    

    --fg_h: #f9f5d7;
    --fg_s: #f2e5bc;
    --fg:  #fbf1c7;
    --fg1: #ebdbb2;
    --fg2: #d5c4a1;
    --fg3: #bdae93;
    --fg4: #a89984;

    --red:    #fb4934;
    --green:  #b8bb26;
    --yellow: #fabd2f;
    --blue:   #83a598;
    --purple: #d3869b;
    --aqua:   #8ec07c;
    --gray:   #928374;
    --orange: #fe8019;
    
    --dark-red: #722529;
    --dark-green: #62693e;
    --dark-aqua: #49503b;
  }
  pre > code {
    background-color: var(--bg_h);
  }
}

@media (prefers-color-scheme: light) {
  :root {
    --fg_h: #1d2021;
    --fg:   #282828;
    --fg_s: #32302f;
    --fg1:  #3c3836;
    --fg2:  #504945;
    --fg3:  #665c54;
    --fg4:  #7c6f64;

    --bg_h: #f9f5d7;
    --bg_s: #f2e5bc;
    --bg:  #fbf1c7;
    --bg1: #ebdbb2;
    --bg2: #d5c4a1;
    --bg3: #bdae93;
    --bg4: #a89984;

    --red: #9d0006;
    --green: #79740e;
    --yellow: #b57614;
    --blue: #076678;
    --purple: #8f3f71;
    --aqua: #427b58;
    --orange: #af3a03;
    
    --dark-red: #fc9487;
    --dark-green: #d5d39b;
    --dark-aqua: #49503b;
  }
  pre > code {
    background-color: var(--bg_s);
  }
}
 
pre > code
{
    overflow-x: auto;
    max-width: min(80em, 95vw);
    display:block;
    color: var(--fg_h);
    padding-left: 1em;
    padding-top: 1em;
}
code { color: var(--fg1);}
.code-variable {
  color: var(--fg1);
}
.code-variable\.builtin {
  color: var(--orange); 
}
.code-variable\.parameter {
  color: var(--blue);
}
.code-variable\.parameter\.builtin {
  color: var(--orange); 

}
.code-variable\.member {
  color: var(--orange); 

}
.code-constant {
  color: var(--purple);
}
.code-constant\.builtin {

  color: var(--orange); 
}
.code-constant\.macro {
  color:var(--aqua);
}
.code-module {
  color: var(--fg1);

}
.code-module\.builtin {
  color: var(--orange); 

}
.code-label {
  color: var(--red);
}
.code-string {
  color: var(--green);
}
.code-string\.documentation {
  color: var(--green);

}
.code-string\.regexp {
  color: var(--green);

}
.code-string\.escape {

  color: var(--orange); 
}
.code-string\.special {

  color: var(--orange); 
}
.code-string\.special\.symbol {
  color: var(--blue);
}
.code-string\.special\.path {
  color: var(--blue);
  text-decoration: underline;
}
.code-string\.special\.url {
  color: var(--blue);
  text-decoration: underline;

}
.code-character {
  color: var(--purple);
}
.code-character\.special {
  color: var(--orange); 

}
.code-boolean {
  color: var(--purple);

}
.code-number {
  color: var(--purple);

}
.code-number\.float {
  color: var(--purple);

}
.code-type {
  color: var(--green);

}
.code-type\.builtin {
  color: var(--green);

}
.code-type\.definition {
  color: var(--green);

}
.code-attribute {
  color:var(--aqua);

}
.code-attribute\.builtin {
  color: var(--orange); 

}
.code-property {
  color: var(--blue);

}
.code-function {
  color: var(--green);
  font-weight: bold;
}
.code-function\.builtin {
  color: var(--orange); 

}
.code-function\.call {
  color: var(--green);
  font-weight: bold;
}
.code-function\.macro {
  color:var(--aqua);

}
.code-function\.method {
  color: var(--green);
  font-weight: bold;

}
.code-function\.method\.call {
  color: var(--green);
  font-weight: bold;

}
.code-constructor {
  color: var(--orange); 

}
.code-operator {
  color: var(--orange); 

}
.code-keyword {

  color: var(--red);
}
.code-keyword\.coroutine {
  color: var(--red);

}
.code-keyword\.function {
  color: var(--red);

}
.code-keyword\.operator {

  color: var(--red);
}
.code-keyword\.import {
  color:var(--aqua);

}
.code-keyword\.type {
  color: var(--red);

}
.code-keyword\.modifier {
  color: var(--red);

}
.code-keyword\.repeat {
  color: var(--red);

}
.code-keyword\.return {
  color: var(--red);

}
.code-keyword\.debug {
  color: var(--orange); 

}
.code-keyword\.exception {
  color: var(--red);

}
.code-keyword\.conditional {
  color: var(--red);

}
.code-keyword\.conditional\.ternary {
  color: var(--red);

}
.code-keyword\.directive {
  color:var(--aqua);

}
.code-keyword\.directive\.define {
  color:var(--aqua);

}
.code-punctuation\.delimiter {

  color: var(--orange); 
}
.code-punctuation\.bracket {
  color: var(--orange); 

}
.code-punctuation\.special {
  color: var(--orange); 

}
.code-comment {
  color: var(--gray);
}
.code-comment\.documentation {
  color: var(--gray);
}
.code-comment\.error {
  color: var(--bg);
  background-color: var(--red);
  font-weight: bold;
}
.code-comment\.warning {
  color: var(--red);
}
.code-comment\.todo {
  color: var(--fg);
  background-color: var(--yellow);
  font-weight:bold;
}
.code-comment\.note {
  color: var(--orange); 

}
.code-markup\.strong {
  color: var(--fg1);
  font-weight: bold;
}
.code-markup\.italic {
  color: var(--fg1);
  font-style: italic;
}
.code-markup\.strikethrough {
  text-decoration: line-through;
  color: var(--fg1);
}
.code-markup\.underline {
  color: var(--fg1);
  text-decoration: underline;
}
.code-markup\.heading {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.1 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.2 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.3 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.4 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.5 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.heading\.6 {
  color: var(--green);
  font-weight: bold;

}
.code-markup\.quote {
  color: var(--fg1);

}
.code-markup\.math {
  color: var(--orange); 

}
.code-markup\.link {
  color: var(--blue);
  text-decoration: underline;

}
.code-markup\.link\.label {
  color: var(--orange); 

}
.code-markup\.link\.url {
  color: var(--blue);
  text-decoration: underline;
}
.code-markup\.raw {
  color: var(--green);
}
.code-markup\.raw\.block {
  color: var(--green);

}
.code-markup\.list {
  color: var(--orange); 

}
.code-markup\.list\.checked {

  color: var(--green);
}
.code-markup\.list\.unchecked {
  color: var(--gray);
}
.code-diff\.plus {
  background-color: var(--dark-green);
}
.code-diff\.minus {
  background-color: var(--dark-red);
}
.code-diff\.delta {
  background-color: var(--dark-aqua);
}
.code-tag {
  color: var(--orange); 

}
.code-tag\.builtin {
  color: var(--orange); 

}
.code-tag\.attribute {
  color: var(--blue);
}
.code-tag\.delimiter {
  color: var(--orange); 

}
```

</details>

If you end up using mordant, and  writing your own theme, please 
[open a pull request](https://github.com/ctdunc/mordant/pulls)! It'd be great to have more themes available.

## A dirt simple build pipeline
Finally, it seems wise to give an example demonstrating how I use mordant and [lowdown](https://kristaps.bsd.lv/lowdown/)
(a super simple markdown translator written in C) to build this website!
This is the whole step:

```sh
# make a temp directory for the mordant-ified md files.
mkdir _blog_intermediate/

for file in ./_blog/**.md; do
	mordant $file -c ./_mordant.toml > ./_blog_intermediate/$(basename $file)
	# we need the no skip/escape html options, since we want to use inline tags.
	lowdown --template=./_template.html \
	  -s ./_blog_intermediate/$(basename $file) \
	  -thtml \
	  --html-no-skiphtml \
	  --html-no-escapehtml > ./blog/$(basename $file .md).html
done
rm -rf _blog_intermediate/
```

I just run this whenever I want to update my blog, and let GitHub pages do the rest. I imagine that as I get 
more familiar with lowdown it'll be fairly easy to extend this to publishing an rss/atom feed and being smarter about
creating a sitemap. These are problems for me when (if) I have more readers.

# Wrapping up
All in, I mostly wrote this for fun. I'm *really* pleased with the results though, and don't see myself 
switching blogging strategies any time in the near future.

If you think this project looks interesting, [star it on github](https://github.com/ctdunc/mordant)! I plan to continue
development.
My immediate roadmap contains the following items:

1. Better error messages/improved error handling.
2. More languages!
3. Utilities for theme generation.
4. Package manager for tree-sitter grammars?
5. Improved compatibility with nvim-treesitter queries (modelines, multiple capture names on one query).

If you'd like to take a stab at any of these, feel free to open an issue, or reach out to me on my socials: 
[github](https://github.com/ctdunc), [twitter](https://x.com/_ctdunc), [linkedin](https://www.linkedin.com/in/connortduncan/).
You can also email me, but I'll refer you to my homepage, lest the robots crawl my email and fill it with (even more) spam.

Thanks for reading, and happy highlighting!
