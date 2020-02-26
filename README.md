# rust-agent-based-models

Reliable and efficient agent-based models in Rust

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/rust-agent-based-models
https://gitlab.com/facorread/rust-agent-based-models

## Why?

Agent-based models (ABM) are computational tools of complexity science and complex adaptive systems. ABMs are popular in computational social science, also known as generative social science, as they enable scientists test hypotheses about human behavior by building artificial societies. ABMs are also a typical tool to study coupled human and natural systems.

Rust is useful to write reliable software for a wide range of applications such as operating systems and embedded systems. Rust does not rely on complicated tools like interpreters or garbage collectors. Rust encourages the developer to use a data-driven coding discipline, by providing a tool called the borrow checker which acts on compile time to prevent code patterns that invalidate memory and send us bug hunting. Fighting with the borrow checker for a little while is a good way to learn good coding practices.

[Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) is a software architecture you might want to consider for your ABM. ECS is a popular design in computer games, and ABMs can benefit from this design. Catherine's West presentation, ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html) provides the rationale for this software. My design makes the most of the borrow checker and keeps complications at bay. The ABM does not encapsulate or hide the model data; instead, all model code resides at ```main()```. As the model grows and evolves, I want to focus on complexity science as opposed to writing and deleting modules, interfaces, or traits to neatly "organize" my data. I have done that in the past with C++ and it never ended well. Take a look at ```main.rs``` and ask the question, what proportion of this software is about the science, and what proportion is about managing memory and other housekeeping?

After you have used popular frameworks to prototype your ABM, consider using Rust over C or C++ to implement the high-performance version of your ABM.

## What can I do with this software?

Ideally you would fork this repository to build your own model; this would give you easy access to bug fixes and enhancements via `git pull` and `git merge`. As an alternative, you can just copy this code and start an ABM in your own repository. Finally, you can implement the ideas in this repository as part of your own design.

If you want to reuse code, for example, to generate links at several stages within a time step, you will need to store the state variables in a ```world``` struct; see Catherine's presentation.

Use statistical software such as R, Julia, or SPSS to analyze and visualize the output files.

Please send me a short email to let me know if you find any of the ideas in this software useful for your own research or if you believe a different approach is necessary.

## Why not make a crate?

Generally speaking, the code for an ABM is tightly integrated; see, for example, a typical NetLogo model: turtles, patches, and links have no privacy or encapsulation. In this repository, model variables such as ```health```, ```next_health```, ```cell_health```, and ```links``` interact so much with one another that it is not worth it to encapsulate them or restrict access. But this also means that separating the code into library and client is not feasible.

Some individual components of the ABM can exist in independent crates. One of them is Orson Peters' [`slotmap`](https://github.com/orlp/slotmap), an efficient memory manager that reuses space left behind by dying agents. The other is my [`wrapping_coords2d`](https://crates.io/crates/wrapping_coords2d) crate, a utility to manage the landscape by mapping a 2D grid of cells into a vector. Both x and y coordinates wrap around the limits of the grid. As an alternative, you can use [`ameda`](https://docs.rs/ameda/latest/ameda) to manage the landscape without wrapping.

## What comes next?

Future improvements include plotting the landscape, histograms, and timelines with the [`plotters`](https://crates.io/crates/plotters) crate. In another improvement, I will do parameter sweeps, also known as BehaviorSpace in NetLogo. I am not committing to specific dates for these enhancements.

## Does this repository use `unsafe` code?

No.

## Other ABM designs and links

[Actix - Actor framework for Rust](https://github.com/actix/actix)

Carmine Spanguolo, [Rust-AB -- An Agent Based Simulation engine in Rust](https://github.com/spagnuolocarmine/abm)

Francis Tseng, [`rust-sim`- Sketches for rust agent-based modeling framework](https://github.com/frnsys/rust-sim)

Orson Peters, [`slotmap` - Data structure for Rust](https://github.com/orlp/slotmap)

Diggory Hardy, [`rand` - A Rust library for random number generation](https://github.com/rust-random/rand)

AJ (Jay, Zencodes, ajjaic), [`ameda` - Manipulate 2D grid indices](https://docs.rs/ameda/latest/ameda)

Wilensky, U. 1999. [NetLogo](http://ccl.northwestern.edu/netlogo/). Center for Connected Learning and Computer-Based Modeling, Northwestern University. Evanston, IL.

## References

West, Catherine (2018), ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html), RustConf 2018 closing keynote.

Antelmi A., Cordasco G., D’Auria M., De Vinco D., Negro A., Spagnuolo C. (2019, October) On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations. In: Tan G., Lehmann A., Teo Y., Cai W. (eds) Methods and Applications for Modeling and Simulation of Complex Systems. In Asian Simulation Conference AsiaSim 2019 (pp. 15-28). Communications in Computer and Information Science, vol 1094. Springer, Singapore. https://doi.org/10.1007/978-981-15-1078-6_2
