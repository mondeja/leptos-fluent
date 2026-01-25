#!/usr/bin/env php
<?php

/**
 * Packagist README Renderer
 * 
 * This script reproduces how Packagist.org renders README.md files.
 * It uses the same libraries and approach as the actual Packagist implementation.
 * 
 * Usage:
 *   php render.php README.md [github.com] [owner/repo]
 *   php render.php README.md github.com mondeja/leptos-fluent
 */

require_once __DIR__ . '/vendor/autoload.php';

use cebe\markdown\GithubMarkdown;
use Symfony\Component\HtmlSanitizer\HtmlSanitizer;
use Symfony\Component\HtmlSanitizer\HtmlSanitizerConfig;
use PackagistReadmeRenderer\ReadmeLinkSanitizer;
use PackagistReadmeRenderer\ReadmeImageSanitizer;

// Parse arguments
$readmeFile = $argv[1] ?? 'README.md';
$host = $argv[2] ?? null;
$ownerRepo = $argv[3] ?? '';

// Check if file exists
if (!file_exists($readmeFile)) {
    echo "Error: File '$readmeFile' not found.\n";
    echo "Usage: php render.php <readme-file> [host] [owner/repo]\n";
    echo "Example: php render.php README.md github.com mondeja/leptos-fluent\n";
    exit(1);
}

// Read README content
$readmeContent = file_get_contents($readmeFile);

// Step 1: Parse markdown to HTML using GithubMarkdown flavor
$parser = new GithubMarkdown();
$html = $parser->parse($readmeContent);

// Step 2: Configure HTML sanitizer (same as Packagist)
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

$config = new HtmlSanitizerConfig();
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

// Add custom sanitizers if host is provided
if ($host !== null && $ownerRepo !== '') {
    $basePath = ''; // Could be extracted from README path if needed
    $config = $config
        ->withAttributeSanitizer(new ReadmeLinkSanitizer($host, $ownerRepo, $basePath))
        ->withAttributeSanitizer(new ReadmeImageSanitizer($host, $ownerRepo, $basePath));
}

// Step 3: Sanitize HTML
$sanitizer = new HtmlSanitizer($config);
$sanitizedHtml = $sanitizer->sanitizeFor('body', $html);

// Step 4: Post-processing (same as Packagist)
// Remove first h1/h2 (usually the project name)
$sanitizedHtml = preg_replace('{^<(h[12])[^>]*>.*?</\1>}', '', $sanitizedHtml);

// Replace raw GitHub URLs with CDN (optimization done by PackageReadme entity)
$sanitizedHtml = str_replace(
    ['<img src="https://raw.github.com/', '<img src="https://raw.githubusercontent.com/'],
    '<img src="https://rawcdn.githack.com/',
    $sanitizedHtml
);

// Output the result
echo $sanitizedHtml;
