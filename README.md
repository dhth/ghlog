<p align="center">
  <h1 align="center">ghlog</h1>
  <p align="center">
    <a href="https://github.com/dhth/ghlog/actions/workflows/main.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/ghlog/main.yml?style=flat-square"></a>
  </p>
</p>

`ghlog` lets you view a GitHub user's recent activity.

> View a demo of `ghlog`'s output [here][activity].

> [!NOTE]
> ghlog is alpha software. Its interface and behavior might change in the near
> future.

🤔 Motivation
---

I sometimes need a quick and easy way to get to the last few things I worked on
on GitHub. Sadly, GitHub doesn't really offer a good UI for this. As such, I
built a simple tool for this. Besides my own activity, I sometimes use it to
keep up with what other people I follow are working on.

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/ghlog
```

⚡️ Usage
---

```text
$ ghlog run -h

Fetch and display events for a GitHub user

Usage: ghlog run [OPTIONS] <USERNAME>

Arguments:
  <USERNAME>  GitHub username to run for

Options:
  -l, --limit <LIMIT>             Maximum number of events to show [default: 20]
  -f, --format <FORMAT>           Output format to use [default: terminal] [possible values: html, markdown, plain, terminal]
      --html-template <TEMPLATE>  HTML template to use [default: terminal] [possible values: editorial, notebook, terminal, zine]
  -h, --help                      Print help (see more with '--help')
```

📄 Output Formats
---

`ghlog` can show results in several formats.

| format   | description                  |
|----------|------------------------------|
| html     | A static HTML report         |
| markdown | Markdown list with links     |
| plain    | Plain unstyled text          |
| terminal | ANSI-colored text with links |

### HTML output

`ghlog` offers 4 built-in templates for the HTML output.

#### Editorial

Serif typography with a magazine-style layout.

| Dark                      | Light                      |
| ------------------------- | -------------------------- |
| ![output][editorial-dark] | ![output][editorial-light] |

#### Notebook

Handwritten typography on a dotted-paper background.

| Dark                     | Light                     |
| ------------------------ | ------------------------- |
| ![output][notebook-dark] | ![output][notebook-light] |

#### Terminal

Monospaced layout resembling a terminal window.

| Dark                     | Light                     |
| ------------------------ | ------------------------- |
| ![output][terminal-dark] | ![output][terminal-light] |

#### Zine

Sans-serif display type with colored labels per event kind.

| Dark                 | Light                 |
| -------------------- | --------------------- |
| ![output][zine-dark] | ![output][zine-light] |

[activity]: https://dhth.github.io/activity/
[editorial-dark]: https://tools.dhruvs.space/images/ghlog/v0-1-0/editorial-dark.png
[editorial-light]: https://tools.dhruvs.space/images/ghlog/v0-1-0/editorial-light.png
[notebook-dark]: https://tools.dhruvs.space/images/ghlog/v0-1-0/notebook-dark.png
[notebook-light]: https://tools.dhruvs.space/images/ghlog/v0-1-0/notebook-light.png
[terminal-dark]: https://tools.dhruvs.space/images/ghlog/v0-1-0/terminal-dark.png
[terminal-light]: https://tools.dhruvs.space/images/ghlog/v0-1-0/terminal-light.png
[zine-dark]: https://tools.dhruvs.space/images/ghlog/v0-1-0/zine-dark.png
[zine-light]: https://tools.dhruvs.space/images/ghlog/v0-1-0/zine-light.png
