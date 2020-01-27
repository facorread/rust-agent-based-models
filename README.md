# rust-agent-based-models

Reliable and efficient agent-based models in Rust

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/rust-agent-based-models
https://gitlab.com/facorread/rust-agent-based-models

## Why?

Agent-based models (ABM) are a computational tool of complexity science; it has a wide range of applications particularly in the framework of complex adaptive systems. ABMs are popular in computational social science, also known as generative social science or artificial societies. ABMs are also a typycal tool to study coupled human and natural systems.

Rust is useful to write safe, fast software for a wide range of applications such as operating systems and embedded systems. Rust removes complications such as interpreters and garbage collectors. Rust trades safety and speed for a data-driven coding discipline, where a tool called the borrow checker prevents us from using patterns that invalidate memory and hurt our research. Fighting with the borrow checker for a little while is a good way to learn good coding practices.

The Entity-Component-System (ECS) is a software architecture you might want to consider for your ABM. ECS is a popular design in computer games, and ABMs can benefit from this design. Catherine's West presentation, ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html) provides the rationale for this software. My design makes the most of the borrow checker and keeps complications at bay. The ABM does not encapsulate or hide the model data; instead, all model code resides at main(). As the model grows and changes, you will probably want to focus your work on complexity science as opposed as writing and deleting modules, interfaces, or traits to neatly "organize" your data. I have done that in the past and it was hell. Take a look at ```main.rs``` and ask the question, what proportion of this software is about the model, and what proportion is about housekeeping?

After you have used popular frameworks to prototype your ABM, consider using Rust over C or C++ to implement the high-performance version of your ABM.

## What can I do with this software?

Ideally you could make a fork of this software and modify it to build and publish your ABM on GitHub or GitLab. Or you can just make a copy and start an ABM in your own repository. Finally, you can take some of the ideas to inform your own design: ideas on how to generate agents and links, how to make them interact and update, how to remove them, and how does the software handle dangling links and states.

If you want to reuse code, for example, to generate links at several stages within a time step, you will need to store the state variables in a ```world``` struct; see Catherine's presentation.

Use statistical software such as R, Julia, or SPSS to analyze and visualize the output files.

Please send me a short email to let me know if you find any of the ideas in this software useful for your own research or if you believe a different approach is necessary.

## Why not make a crate?

Generally speaking, the code for an ABM is tightly integrated; see, for example, a typical NetLogo model. In this software, model variables such as ```health```, ```next_health```, and ```links``` interact so much with one another that it is not worth it to encapsulate them or restrict access. But this also means that separating the code into library and client is not feasible.

Feel free to develop a crate yourself.

## What comes next?

I will use the principles I learned to implement a virtual landscape in the form of a grid of patches. I am not committing to a specific date for this enhancement.
