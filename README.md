# mordant
mordant is a syntax highlighter for Github-flavored markdown files requiring *absolutely zero javascript*. It takes 
fenced code blocks, and converts them to the appropriate HTML with classes corresponding
to the tree-sitter grammar nodes of the block.
It is named after the substance used to bind dyes to fabric, since it binds pretty colors
to your markdown files.

For example,

```{javascript}
(x) => {
  // do stuff to x ...
  return x;
}
```

will be converted to:

```{html}
<pre><code>
<span class="code-punctuation.bracket">(</span><span class="code-variable">x</span><span class="code-punctuation.bracket">)</span> <span class="code-operator">=></span> <span class="code-punctuation.bracket">{</span>
  <span class="code-comment">// do stuff to x ...</span>
  <span class="code-keyword">return</span> <span class="code-variable">x</span><span class="code-punctuation.delimiter">;</span>
<span class="code-punctuation.bracket">}</span>
</code></pre>
```

This project is in pre-pre-pre alpha. All of the grammars are hard-coded and there's no
way to configure the program without recompiling. There are plans to address these shortcomings.

## Usage
After cloning the repo, execute
```
$ cargo run -- --file $FILE_NAME
```

## Roadmap
- Config
    - [ ] Config file (+ hierarchy).
    - [ ] Import grammars, queries.
-Hygiene
    - [ ] Don't just put everything in `main`.
    - [ ] Tests
- Performance
    - [ ] Multi-Threading?
    - [ ] Cached Languages.
- Docs
    - [ ] Usage examples with other markdown renderers (see [this example](https://github.com/ctdunc/ctdunc.github.io/blob/master/_publish_blog.sh)).
    - [ ] CONTRIBUTING.
