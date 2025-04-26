# test doc

some text
 
<pre><code><span class="code-keyword">from</span> <span class="code-variable">dash</span> <span class="code-keyword">import</span> <span class="code-variable">clientside_callback</span>, <span class="code-constructor">Input</span>, <span class="code-constructor">Output</span>
<span class="code-variable">gridid</span> <span class="code-operator">=</span> <span class="code-string">"grid"</span>
<span class="code-function">clientside_callback</span>(
    <span class="code-string">"""
<span class="code-punctuation.bracket">(</span><span class="code-variable">id</span><span class="code-punctuation.bracket">)</span> <span class="code-operator">=&gt;</span> <span class="code-punctuation.bracket">{</span>
  <span class="code-variable">dash_ag_grid</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">getApiAsync</span><span class="code-punctuation.bracket">(</span><span class="code-variable">id</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">then</span><span class="code-punctuation.bracket">(</span><span class="code-punctuation.bracket">(</span><span class="code-variable">api</span><span class="code-punctuation.bracket">)</span> <span class="code-operator">=&gt;</span> <span class="code-punctuation.bracket">{</span>
    <span class="code-variable">api</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">addEventListener</span><span class="code-punctuation.bracket">(</span><span class="code-string">"cellFocused"</span><span class="code-punctuation.delimiter">,</span> <span class="code-punctuation.bracket">(</span><span class="code-variable">params</span><span class="code-punctuation.bracket">)</span> <span class="code-operator">=&gt;</span> <span class="code-punctuation.bracket">{</span>
      <span class="code-variable.builtin">console</span><span class="code-punctuation.delimiter">.</span><span class="code-function.method">log</span><span class="code-punctuation.bracket">(</span><span class="code-variable">params</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
    <span class="code-punctuation.bracket">}</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
  <span class="code-punctuation.bracket">}</span><span class="code-punctuation.bracket">)</span><span class="code-punctuation.delimiter">;</span>
  <span class="code-keyword">return</span> <span class="code-variable">dash_clientside</span><span class="code-punctuation.delimiter">.</span><span class="code-property">no_update</span><span class="code-punctuation.delimiter">;</span>
<span class="code-punctuation.bracket">}</span><span class="code-punctuation.delimiter">;</span>
    """</span>,
    <span class="code-function">Output</span>(<span class="code-variable">gridid</span>, <span class="code-string">"id"</span>),
    <span class="code-function">Input</span>(<span class="code-variable">gridid</span>, <span class="code-string">"id"</span>),
)

</code></pre>


another paragraph
