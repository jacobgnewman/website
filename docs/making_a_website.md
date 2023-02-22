---
title: Making a website
---

# Ambition

So I want to make a website. Somewhere for a mix of blogging, writeups, who knows what.
I also want to achieve this without using massive unwieldly javascript frameworks and libraries that I don't understand.

## Objectives

1. I want to be able to write the majority of my website in markdown
2. I want to be able to modify and change functionality as I see fit
3. I want to understand as much of what my website is doing as possible

## The Process

### Markdown

So, starting with the easy stuff, parsing markdown.
I am going to use comrak crate to parse my markdown files.
It's a rust port of the Github fork of cmark, which implements the CommonMark spec
of Markdown.
The Github fork adds Github flavored markdown, which includes extentions for tables,
task list items, and some other handy functionality.
It lets you parse the AST it generates so that I can make changes if down the road
I want to add functionality.

A thing right off the bat that I will want is math equations, good news, theres a
rust crate that binds to katex, so catching math in comrack and passing it to katex
should be the solution.

I preprocess the MD with regex for the math expressions, and replace them with the
generated katex, that is then handed to comrak to do the final conversion

#### Headers

### Serving the website

So here begins the rabbit hole.




