---
title: "How I Built This Blog (Part 3): Bringing the Site to Life with CSS and JavaScript"
date: "2025-08-01"
category: "Projects"
tags: ["css", "javascript", "frontend", "ux"]
excerpt: "A website is more than just HTML. In Part 3, I cover the final 10% that makes 90% of the difference: the frontend. Discover how I used CSS variables for theming and plain JavaScript to power interactive filters, sorting, and pagination."
# image: "/images/placeholder-part3.jpg"
---

In the last two parts, we built a powerful engine. Our Rust code can now take markdown files, parse them into structured data, and use Askama templates to generate a complete set of static HTML pages. When we run `axumpress serve`, we get a functional `output/` folder.

But when you open those pages in a browser, they're just... there. They're static. Dead. A modern website needs to feel alive. The buttons should react, the filters should filter, and the layout should be beautiful. This is the story of the final layer: the frontend polish.

### The Styling Foundation: One Stylesheet to Rule Them All

Initially, I had different CSS files for different parts of the site, which was a nightmare to manage. The solution was to create a single, master stylesheet: `static/css/style.css`. This file styles everything, but the rules only apply when the corresponding HTML elements are on the page.

The key to making the design feel cohesive was using **CSS Custom Properties** (also called CSS Variables). At the top of my stylesheet, I defined our "Monokai Pro" inspired theme.

```css
/* A snippet from static/css/style.css */
:root {
  /* Monokai Pro Color Palette */
  --bg-primary: #2d2a2e;
  --bg-secondary: #403e41;
  --text-primary: #fcfcfa;
  --accent-yellow: #ffd866;
  --accent-cyan: #78dce8;
}
```

By using `var(--bg-primary)` throughout the file instead of hardcoding `#2d2a2e`, I can change the entire site's theme just by swapping out these variables. This is the foundation for the theme-picker I built in the `init` command.

### The Interactive Brain: Vanilla JavaScript

While Rust and Askama did the heavy lifting of preparing the content, **JavaScript owns the browser**. My goal was to add modern, interactive features without needing a heavy framework like React or Vue. Plain, vanilla JavaScript was more than powerful enough.

The logic lives in two files: `main.js` for site-wide features and `blog.js` for the blog-specific magic.

#### 1. Filtering and Sorting: The `data-*` Attribute Bridge

This was the biggest challenge. How do you filter posts on a static page? The answer is to let Rust put all the necessary information into the HTML, and let JavaScript handle the showing and hiding.

In our `blog_index.html` template, we added special `data-*` attributes to each post's article tag:
```html
<article 
  class="blog-card" 
  data-category="{{ post.front_matter.category }}"
  data-date="{{ post.front_matter.date }}">
  <!-- ... card content ... -->
</article>
```
These attributes act as a hidden bridge between the backend and frontend. Now, our `blog.js` can read them.

The filtering logic is simple but effective:
1.  On page load, grab *all* the blog card elements into an array.
2.  Add event listeners to the category dropdown, sort dropdown, and search input.
3.  Whenever a filter changes, loop through the **original array of all cards**.
4.  For each card, check if its `data-category` matches the selected category and if its title matches the search query.
5.  Clear the blog grid on the page and append only the cards that passed the filter.

#### 2. Smart Pagination: No More Infinite Scrolling

Showing 50 blog posts on one page is slow and overwhelming. The pagination system works hand-in-hand with the filtering logic.

1.  **State Management:** The script keeps track of the `currentPage` and how many `POSTS_PER_PAGE` to show.
2.  **Slicing the Array:** After the posts are filtered and sorted, instead of showing all of them, we use `array.slice()` to grab just the chunk for the current page (e.g., posts 10 through 19 for page 2).
3.  **Dynamic Buttons:** The script dynamically creates the "Previous," "Next," and page number buttons based on the total number of filtered posts. If there's only one page of results, the pagination disappears entirely.

This client-side approach gives the feel of a dynamic server-rendered application but with the speed and simplicity of a static site.

### The Final Product

With the CSS providing a beautiful and consistent theme, and the JavaScript adding a layer of rich interactivity, the project finally felt complete. It had fulfilled my mentor's challenge: it was a blog, built by me, for me, that I could actually use and be proud of.

The journey from a simple idea to a fully-featured Static Site Generator has been an incredible learning experience. There's always more to do—like adding `axumpress add post` to the CLI—but the foundation is strong, the architecture is clean, and most importantly, it works.

Thanks for following along. I hope this series has been as fun to read as the project was to build.
