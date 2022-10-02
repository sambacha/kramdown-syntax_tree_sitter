# frozen_string_literal: true

require 'kramdown'
require 'test_helper'

PYTHON_MARKDOWN = <<~MARKDOWN
  ~~~python
  print('Hello, World!')
  ~~~
MARKDOWN

PYTHON_NO_HIGHLIGHT_HTML = <<~HTML
  <pre><code class="language-python">print('Hello, World!')
  </code></pre>
HTML

module Kramdown
  class TestSyntaxHighlighting < Minitest::Test
    def test_that_tree_sitter_has_a_version_number
      refute_nil Converter::SyntaxHighlighter::TreeSitter::VERSION
    end

    def test_that_it_can_use_no_highlighting
      actual = Document.new(PYTHON_MARKDOWN, syntax_highlighter: nil).to_html
      assert_equal actual, PYTHON_NO_HIGHLIGHT_HTML
    end
  end
end
