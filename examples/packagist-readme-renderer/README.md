# Packagist README Renderer Example

This directory contains a working example that reproduces how Packagist.org renders README.md files.

## What This Does

This implementation replicates Packagist's README rendering pipeline:

1. **Markdown Parsing** - Uses `cebe/markdown` with GithubMarkdown flavor
2. **HTML Sanitization** - Uses Symfony's HtmlSanitizer with Packagist's configuration
3. **Custom Sanitizers** - Implements ReadmeLinkSanitizer and ReadmeImageSanitizer
4. **Post-processing** - Removes first heading and optimizes CDN URLs

## Installation

```bash
cd examples/packagist-readme-renderer
composer install
```

## Usage

### Basic Usage (without repository context)

```bash
php render.php ../../README.md > output.html
```

This renders the README with basic sanitization but without repository-aware link/image transformations.

### Full Usage (with repository context)

```bash
php render.php ../../README.md github.com mondeja/leptos-fluent > output.html
```

This renders the README with full Packagist-style transformations:
- Relative links → Absolute GitHub URLs
- Relative images → Raw GitHub content URLs
- Anchor links prefixed with `user-content-`
- External links get security attributes

### Arguments

```
php render.php <readme-file> [host] [owner/repo]
```

- `<readme-file>`: Path to the README.md file (required)
- `[host]`: Repository host (optional) - `github.com`, `gitlab.com`, or `bitbucket.org`
- `[owner/repo]`: Repository owner and name (optional) - e.g., `mondeja/leptos-fluent`

## Examples

```bash
# Render this project's README
php render.php ../../README.md github.com mondeja/leptos-fluent > leptos-fluent.html

# Render any markdown file
echo "# Hello World\nThis is **markdown**" > test.md
php render.php test.md > test.html

# View in browser
php render.php ../../README.md github.com mondeja/leptos-fluent | python3 -c "import sys; print('<html><head><meta charset=\"utf-8\"><style>body{max-width:800px;margin:50px auto;font-family:sans-serif;line-height:1.6;}code{background:#f4f4f4;padding:2px 6px;border-radius:3px;}pre{background:#f4f4f4;padding:10px;border-radius:5px;overflow-x:auto;}</style></head><body>' + sys.stdin.read() + '</body></html>')" > preview.html
```

## Files

- **render.php** - Main rendering script
- **src/ReadmeLinkSanitizer.php** - Custom sanitizer for links (from Packagist)
- **src/ReadmeImageSanitizer.php** - Custom sanitizer for images (from Packagist)
- **composer.json** - Dependencies configuration

## How It Works

The renderer follows Packagist's exact process:

1. Reads the markdown file
2. Parses it to HTML using `GithubMarkdown` parser
3. Configures HTML sanitizer with allowed elements and attributes
4. Applies custom sanitizers for links and images (if repository context provided)
5. Removes the first `<h1>` or `<h2>` tag
6. Replaces raw GitHub URLs with CDN URLs

## Differences from Packagist

This is a simplified version that:
- ✅ Uses the same markdown parser
- ✅ Uses the same HTML sanitizer configuration
- ✅ Implements the same custom sanitizers
- ✅ Applies the same post-processing
- ❌ Doesn't handle GitHub README API endpoint
- ❌ Doesn't detect base path from GitHub's data-path attribute
- ❌ Doesn't handle all edge cases in the production system

For the complete implementation, see the [Packagist source code](https://github.com/composer/packagist/blob/main/src/Package/Updater.php).

## Testing

You can test the rendering by comparing output with a real Packagist package page:

1. Render a local README: `php render.php README.md github.com owner/repo`
2. Visit the package on Packagist: `https://packagist.org/packages/owner/repo`
3. Compare the rendered HTML

The output should be very similar to what Packagist displays.
