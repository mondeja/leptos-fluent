#!/usr/bin/env php
<?php

/**
 * HTML Viewer for Packagist README Renderer
 * 
 * This script wraps the rendered HTML in a full HTML document with
 * styling similar to Packagist's README display.
 * 
 * Usage:
 *   php view.php README.md [github.com] [owner/repo] > output.html
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
    exit(1);
}

// Read and render README
$readmeContent = file_get_contents($readmeFile);
$parser = new GithubMarkdown();
$html = $parser->parse($readmeContent);

$elements = [
    'p', 'br', 'small', 'strong', 'b', 'em', 'i', 'strike',
    'sub', 'sup', 'ins', 'del', 'ol', 'ul', 'li',
    'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'dl', 'dd', 'dt',
    'pre', 'code', 'samp', 'kbd', 'q', 'blockquote', 'abbr', 'cite',
    'table', 'thead', 'tbody', 'tr', 'span', 'summary',
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

if ($host !== null && $ownerRepo !== '') {
    $basePath = '';
    $config = $config
        ->withAttributeSanitizer(new ReadmeLinkSanitizer($host, $ownerRepo, $basePath))
        ->withAttributeSanitizer(new ReadmeImageSanitizer($host, $ownerRepo, $basePath));
}

$sanitizer = new HtmlSanitizer($config);
$sanitizedHtml = $sanitizer->sanitizeFor('body', $html);
$sanitizedHtml = preg_replace('{^<(h[12])[^>]*>.*?</\1>}', '', $sanitizedHtml);
$sanitizedHtml = str_replace(
    ['<img src="https://raw.github.com/', '<img src="https://raw.githubusercontent.com/'],
    '<img src="https://rawcdn.githack.com/',
    $sanitizedHtml
);

?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Packagist README Preview</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 900px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        
        .container {
            background-color: white;
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
        
        h1, h2, h3, h4, h5, h6 {
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: 600;
            line-height: 1.25;
            border-bottom: 1px solid #eaecef;
            padding-bottom: 0.3em;
        }
        
        h1 { font-size: 2em; }
        h2 { font-size: 1.5em; }
        h3 { font-size: 1.25em; }
        
        p {
            margin-bottom: 16px;
        }
        
        code {
            background-color: #f6f8fa;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
            font-size: 85%;
        }
        
        pre {
            background-color: #f6f8fa;
            padding: 16px;
            overflow-x: auto;
            border-radius: 6px;
            line-height: 1.45;
            margin-bottom: 16px;
        }
        
        pre code {
            background-color: transparent;
            padding: 0;
            font-size: 100%;
        }
        
        a {
            color: #0366d6;
            text-decoration: none;
        }
        
        a:hover {
            text-decoration: underline;
        }
        
        blockquote {
            padding: 0 1em;
            color: #6a737d;
            border-left: 4px solid #dfe2e5;
            margin: 0 0 16px 0;
        }
        
        table {
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 16px;
        }
        
        table th,
        table td {
            padding: 6px 13px;
            border: 1px solid #dfe2e5;
        }
        
        table thead {
            background-color: #f6f8fa;
        }
        
        table tr:nth-child(even) {
            background-color: #f6f8fa;
        }
        
        ul, ol {
            margin-bottom: 16px;
            padding-left: 2em;
        }
        
        li {
            margin-bottom: 4px;
        }
        
        img {
            max-width: 100%;
            height: auto;
        }
        
        details {
            margin-bottom: 16px;
        }
        
        summary {
            cursor: pointer;
            font-weight: 600;
            padding: 8px 0;
        }
        
        summary:hover {
            color: #0366d6;
        }
        
        hr {
            height: 0.25em;
            padding: 0;
            margin: 24px 0;
            background-color: #e1e4e8;
            border: 0;
        }
        
        .header {
            text-align: center;
            margin-bottom: 30px;
            padding-bottom: 20px;
            border-bottom: 2px solid #e1e4e8;
        }
        
        .header h1 {
            color: #0366d6;
            border-bottom: none;
            margin: 0;
        }
        
        .header p {
            color: #6a737d;
            margin-top: 8px;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📦 Packagist README Preview</h1>
            <p>Rendered with Packagist-style rendering</p>
        </div>
        <?= $sanitizedHtml ?>
    </div>
</body>
</html>
