# zed-mcp-server-2slides

Zed extension that wraps the [`2slides-mcp`](https://www.npmjs.com/package/2slides-mcp) MCP server.

## Configuration

This MCP server requires your 2slides API key. Follow these steps to obtain it:

1. Visit [2slides.com/api](https://2slides.com/api) to get your API key
2. Sign up or log in to your 2slides account
3. Generate or copy your **API Key**

In your Zed settings:

```json
{
    "context_servers": {
        "mcp-server-2slides": {
          "settings": {
              "api_key": "YOUR_2SLIDES_API_KEY"
          }
        }
    }
}
```

## Available Tools

The 2slides MCP server provides the following tools:

- `slides_generate` - Generate slides with a theme, user input, and response language
- `jobs_get` - Get job status and results by job ID
- `themes_search` - Search for available themes

For more information, visit [2slides.com](https://2slides.com) and the [2slides-mcp repository](https://github.com/2slides/2slides-mcp).
