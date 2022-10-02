# frozen_string_literal: true

require_relative 'lib/kramdown/syntax_tree_sitter/version'

Gem::Specification.new do |spec|
  spec.name = 'kramdown-syntax_tree_sitter'
  spec.version = Kramdown::Converter::SyntaxHighlighter::TreeSitter::VERSION
  spec.summary = 'Syntax highlight code with Tree-Sitter in Kramdown.'
  spec.author = 'Andrew T. Biehl'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 2.7'
  spec.homepage = 'https://github.com/andrewtbiehl/kramdown-syntax_tree_sitter'

  spec.add_runtime_dependency 'kramdown', '~> 2.0'

  spec.add_development_dependency 'minitest', '~> 5.0'
  spec.add_development_dependency 'rake', '~> 13.0'
  spec.add_development_dependency 'rubocop', '~> 1.36'
  spec.add_development_dependency 'rubocop-minitest', '~> 0.22'
  spec.add_development_dependency 'rubocop-rake', '~> 0.6'

  spec.files = Dir['LICENSE.txt', 'README.md', 'lib/**/*.rb']
end
