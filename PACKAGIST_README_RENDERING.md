# Packagist README Rendering

This document explains how Packagist.org renders README.md files from package repositories and provides a way to reproduce the rendering locally.

## Overview

Based on research of the [composer/packagist](https://github.com/composer/packagist) repository, Packagist uses a multi-step process to render README files:

1. **Markdown Parsing**: Uses `cebe/markdown` library's `GithubMarkdown` parser
2. **HTML Sanitization**: Uses Symfony's `HtmlSanitizer` component
3. **Custom Attribute Sanitizers**: Applies custom logic for links and images
4. **Post-processing**: Removes first `<h1>` or `<h2>` tag and applies optimizations

## Key Components

### 1. Markdown Parser

Packagist uses the **cebe/markdown** library with the `GithubMarkdown` flavor:

```php
use cebe\markdown\GithubMarkdown;

$parser = new GithubMarkdown();
$html = $parser->parse($markdownContent);
```

**Library**: [`cebe/markdown`](https://github.com/cebe/markdown) - A fast and extensible markdown parser

### 2. HTML Sanitization

The rendered HTML is sanitized using **Symfony HtmlSanitizer** with strict rules:

**Allowed Elements**:
- Text formatting: `p`, `br`, `small`, `strong`, `b`, `em`, `i`, `strike`, `sub`, `sup`, `ins`, `del`
- Lists: `ol`, `ul`, `li`, `dl`, `dd`, `dt`
- Headings: `h1`, `h2`, `h3`, `h4`, `h5`, `h6`
- Code: `pre`, `code`, `samp`, `kbd`
- Quotes: `q`, `blockquote`, `abbr`, `cite`
- Tables: `table`, `thead`, `tbody`, `tr`, `td`, `th`
- Other: `span`, `summary`, `details`

**Allowed Attributes**:
- `img`: `src`, `title`, `alt`, `width`, `height`
- `a`: `href`, `target`, `id`
- `td`, `th`: `colspan`, `rowspan`
- `details`: `open`
- `align` for: `th`, `td`, `p`, `h1-h6`
- `class` for all elements

**Link Schemes**: Only `https`, `http`, and `mailto`

**Security Rules**:
- All links get `rel="nofollow noindex noopener external ugc"`
- External links open in `_blank`
- Relative links and media are allowed

### 3. Custom Sanitizers

#### ReadmeLinkSanitizer

Handles link transformations:
- Converts relative links to absolute URLs based on repository host (GitHub, GitLab)
- Adds `user-content-` prefix to anchor IDs
- Rewrites private GitHub user images to public CDN URLs

#### ReadmeImageSanitizer

Handles image transformations:
- Converts relative image paths to raw content URLs:
  - GitHub: `https://raw.github.com/{owner}/{repo}/HEAD/{basePath}{image}`
  - GitLab: `https://gitlab.com/{owner}/{repo}/-/raw/HEAD/{basePath}{image}`
  - Bitbucket: `https://bitbucket.org/{owner}/{repo}/raw/HEAD/{basePath}{image}`
- Rewrites private GitHub user images to public CDN URLs

### 4. Post-Processing

After sanitization:
1. Removes the first `<h1>` or `<h2>` element (usually the project name)
2. Replaces raw GitHub URLs with CDN URLs:
   - `https://raw.github.com/` → `https://rawcdn.githack.com/`
   - `https://raw.githubusercontent.com/` → `https://rawcdn.githack.com/`

## Local Reproduction

### Requirements

- PHP 8.1 or higher
- Composer

### Installation

Create a new directory and install dependencies:

```bash
mkdir packagist-readme-renderer
cd packagist-readme-renderer
composer require cebe/markdown symfony/html-sanitizer composer/pcre
```

### Renderer Script

Create a file named `render_readme.php`:

```php
<?php

require_once 'vendor/autoload.php';

use cebe\markdown\GithubMarkdown;
use Symfony\Component\HtmlSanitizer\HtmlSanitizer;
use Symfony\Component\HtmlSanitizer\HtmlSanitizerConfig;

// Read README content
$readmeContent = file_get_contents($argv[1] ?? 'README.md');

// Parse markdown to HTML
$parser = new GithubMarkdown();
$html = $parser->parse($readmeContent);

// Configure HTML sanitizer
$elements = [
    'p', 'br', 'small',
    'strong', 'b', 'em', 'i', 'strike',
    'sub', 'sup', 'ins', 'del',
    'ol', 'ul', 'li',
    'h1', 'h2', 'h3', 'h4', 'h5', 'h6',
    'dl', 'dd', 'dt',
    'pre', 'code', 'samp', 'kbd',
    'q', 'blockquote', 'abbr', 'cite',
    'table', 'thead', 'tbody', 'tr',
    'span', 'summary',
];

$config = HtmlSanitizerConfig::new();
foreach ($elements as $el) {
    $config = $config->allowElement($el);
}

$config = $config
    ->allowElement('img', ['src', 'title', 'alt', 'width', 'height'])
    ->allowElement('a', ['href', 'target', 'id'])
    ->allowElement('td', ['colspan', 'rowspan'])
    ->allowElement('th', ['colspan', 'rowspan'])
    ->allowElement('details', ['open'])
    ->allowAttribute('align', ['th', 'td', 'p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6'])
    ->allowAttribute('class', '*')
    ->allowLinkSchemes(['https', 'http', 'mailto'])
    ->forceAttribute('a', 'rel', 'nofollow noindex noopener external ugc')
    ->allowRelativeLinks()
    ->allowRelativeMedias()
    ->withMaxInputLength(10_000_000);

$sanitizer = new HtmlSanitizer($config);
$sanitizedHtml = $sanitizer->sanitizeFor('body', $html);

// Remove first h1/h2 (like Packagist does)
$sanitizedHtml = preg_replace('{^<(h[12])[^>]*>.*?</\1>}', '', $sanitizedHtml);

// Output result
echo $sanitizedHtml;
```

### Usage

```bash
# Render a README.md file
php render_readme.php README.md > output.html

# Or pipe markdown content
echo "# Hello World" | php render_readme.php /dev/stdin > output.html
```

### Advanced: Full Packagist-style Rendering

For full Packagist-compatible rendering including custom link/image sanitizers, you would need to:

1. Implement `ReadmeLinkSanitizer` class
2. Implement `ReadmeImageSanitizer` class
3. Add them to the config:

```php
$config = $config
    ->withAttributeSanitizer(new ReadmeLinkSanitizer($host, $ownerRepo, $basePath))
    ->withAttributeSanitizer(new ReadmeImageSanitizer($host, $ownerRepo, $basePath));
```

See the full implementation in Packagist's source code:
- [`src/HtmlSanitizer/ReadmeLinkSanitizer.php`](https://github.com/composer/packagist/blob/main/src/HtmlSanitizer/ReadmeLinkSanitizer.php)
- [`src/HtmlSanitizer/ReadmeImageSanitizer.php`](https://github.com/composer/packagist/blob/main/src/HtmlSanitizer/ReadmeImageSanitizer.php)

## Summary

Packagist's README rendering process:

1. ✅ Uses **cebe/markdown** with GithubMarkdown flavor for parsing
2. ✅ Uses **Symfony HtmlSanitizer** for security and cleanup
3. ✅ Applies custom sanitizers for repository-aware link/image processing
4. ✅ Removes first heading and optimizes CDN URLs

The rendering is designed to be secure (no XSS), repository-aware (converts relative paths), and consistent with GitHub-flavored markdown.

## References

- [Packagist Repository](https://github.com/composer/packagist)
- [Packagist Updater.php](https://github.com/composer/packagist/blob/main/src/Package/Updater.php) - Main rendering logic
- [cebe/markdown](https://github.com/cebe/markdown) - Markdown parser
- [Symfony HtmlSanitizer](https://symfony.com/doc/current/html_sanitizer.html) - HTML sanitization component
