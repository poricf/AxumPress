---
title: "How I Built This Blog (Final Part): A Thank You, and Why I Fell in Love with Rust"
date: "2025-08-05"
category: "Thoughts"
tags: ["rust", "reflection", "mentorship", "conclusion"]
excerpt: "The journey is complete (for now). In this final part, I want to extend a huge thank you to my mentor, reflect on the biggest challenges, and share why Rust's unique approach to programming has completely won me over."
# No image for this one, to test the placeholder bar again.
---

We did it. From a simple challenge to a fully functional static site generator, AxumPress is alive. We've covered the data pipeline, the CLI, the templating engine, and the frontend polish. Now, for this final part, I want to step back from the code and talk about the journey itself.

### A Foundation of Gratitude

First and foremost, none of this would have happened without the guidance and encouragement of my mentor, Matthew Harwood (Aurter). His simple challenge—"Build your own blog"—was the spark that ignited this entire project. It's a testament to the power of good mentorship: a simple nudge in the right direction can set you on a path of incredible growth and discovery. So, to Matthew, thank you. This project is a direct result of your wisdom.

### The Challenges That Shaped the Code

Building anything from scratch is a series of facing walls and finding ways to climb over them. My biggest challenge was bridging the gap between Rust's backend logic and the frontend's need for data. My initial attempts were clumsy, leading to a tangled mess of string replacements. The breakthrough came when I established a clean, disciplined architecture:

1.  **Parse, Don't Pass:** The markdown processor's only job became parsing files into a perfect `Vec<Post>`. It doesn't know or care about HTML templates.
2.  **Generate, Don't Think:** The generator's only job is to take that `Vec<Post>` and pass it to a template. It doesn't know how the data was created.
3.  **Display, Don't Process:** The Askama template's only job is to display the data it's given. It contains no complex logic.

This separation of concerns was the key that unlocked the whole project.

### Why I Truly Love Rust

Before this project, I just wanted to learn Rust. Now, I can say I genuinely love writing in it, and it comes down to one core concept: **confidence**.

Being new to Rust, the initial learning curve was definitely challenging. I would try to write code like it was Python and make mistakes, only to be met with a compiler error that felt like a cryptic puzzle. But that's where the real learning began. The learning is not flat; there is always a new concept to grasp. My process became a cycle: hit an error, read the docs, watch a YouTube video, ask GPT for a different perspective, and then, finally, that "aha!" moment. It was tough, but cool.

The Rust compiler is famously strict, but that strictness isn't a punishment; it's a partnership. The things you fight with in the beginning—the borrow checker, ownership, lifetimes—are the very things that give you superpowers later. When I refactored my code to pass data around, the compiler held my hand, forcing me to think about who *owns* the data at every step. The moment it compiles, you feel an incredible sense of security. You know that an entire class of memory bugs is simply impossible. You can build with confidence, knowing your foundation is rock-solid.

### What's Next?

This is just the beginning. Now that I have a platform, I'm excited to write more—about Rust, about technology, and about the things I'm building.

I will also continue to improve this site. I'm already dreaming of adding a command like `axumpress add post` to make content creation even easier. The beauty of owning your own tools is that you can shape them however you see fit.

If you're curious to see the code that powers all of this, or if you want to use AxumPress for your own site, the entire project is open source. I would be honored if you took a look.

**[Check out the AxumPress GitHub Repository Here](https://github.com/poricf/axumpress)**

Thank you for joining me on this journey. Now, let's go build something.
```