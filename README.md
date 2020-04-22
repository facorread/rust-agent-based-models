# rust-agent-based-models

Reliable and efficient agent-based models in Rust

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/rust-agent-based-models
https://gitlab.com/facorread/rust-agent-based-models

## Why?

Agent-based models (ABM) are computational tools of complexity science and complex adaptive systems. ABMs are popular in computational social science, also known as generative social science, as they enable scientists test hypotheses about human behavior by building artificial societies. ABMs are also a typical tool to study coupled human and natural systems.

[Rust] is useful to write reliable software for a wide range of applications such as operating systems and embedded systems. Rust does not rely on complicated tools like interpreters or garbage collectors. Rust encourages the developer to use a data-driven coding discipline, by providing a tool called the borrow checker which acts at compile time to prevent code patterns that invalidate memory and send us bug hunting. Fighting with the borrow checker for a little while is a good way to learn good coding practices.

[Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) is a software architecture you might want to consider for your ABM. ECS is a popular design in computer games, and ABMs can benefit from this design. Catherine West's presentation, ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html) provides the rationale for this software. My design makes the most of the borrow checker and keeps complications at bay. The ABM does not encapsulate or hide the model data; instead, all model code resides at ```main()```. As the model grows and evolves, I want to focus on complexity science as opposed to writing and deleting modules, interfaces, or traits to neatly "organize" my data. I have done that in the past with C++ and it never ended well. Take a look at ```main.rs``` and ask the question, what proportion of this software is about the science, and what proportion is about managing memory and other housekeeping?

After you have used popular frameworks to prototype your ABM, consider using Rust over C or C++ to implement the high-performance version of your ABM.

## Getting started

Install [Rust], [Visual Studio Code], the [Better TOML extension], the [Rust (rls) extension], the [Rust Clippy tool], and the [FFmpeg] binaries for your operating system. Installation of these packages should be straightforward.

[Clone] this repository (`https://github.com/facorread/rust-agent-based-models.git`) and use your favorite terminal to run `cargo run --release`.

The code is an exaggerated example of an infectious model; it implements an infectious disease transmitting between agents, between agents and cells, and across cells.

See an [example video here](https://youtu.be/oYeHyFl1-HY)

## What can I do with this software?

Ideally you would fork this repository to build your own model; this would give you easy access to bug fixes and enhancements via `git pull` and `git merge`. As an alternative, you can just copy this code and start an ABM in your own repository. Finally, you can implement the ideas in this repository as part of your own design.

If you want to reuse code, for example, to generate links at several stages within a time step, you will need to store the state variables in a `world` struct; see Catherine's presentation.

Use statistical software such as R, Julia, or SPSS to analyze and visualize the output files.

Please send me a short email to let me know if you find any of these ideas useful for your own research or if you believe a different approach or software architecture is necessary.

## Advanced usage

This software uses [Cargo's features] and [Rust's conditional compilation] to enable and disable graphics, social network, and landscape. By default, `cargo.toml` activates the `all-graphics` feature. This default is the best option for most cases. This is also the only configuration that undergoes automated testing at GitHub and GitLab.

The selections made at `cargo.toml` enable or disable code at `main.rs` through the `#[cfg(feature = )]` attributes. For example, `#[cfg(feature = "net")]` in `main.rs` enables the social network, including the `links` container and the network dynamics. It is useful to be able to turn the network off when development focuses on the landscape or other aspect of the model. By turning the network on or off, and re-running the program, it is possible to catch errors, compare component dynamics, and visualize different component outcomes. Turning off unnecessary components can also speed up the model. For example, activating `default = ["no-graphics"]` in `Cargo.toml` is useful to perform a parameter sweep on thousands of scenarios and save simulated data in `csv` format without producing numerous `png` images.

The features at Cargo.toml, namely landscape and net, are arbitrary examples based on the structure of the model. Take finer control of development, memory, and performance of your model by introducing features you can disable or enable with just a line of code.

## Why not make a crate?

Generally speaking, the code for an ABM is tightly integrated; take a NetLogo model for example: turtles, patches, and links have no privacy or encapsulation. In this repository, model variables such as ```health```, ```next_health```, ```cell_health```, and ```links``` are integrated as well; it is not worth it to try encapsulate them or restrict access. But this also means that separating the code into library and client is not very practical.

Some individual components of an ABM can exist in independent crates. One of them is Orson Peters' [`slotmap`](https://github.com/orlp/slotmap), an efficient memory manager that reuses space left behind by dying agents. The other is my [`wrapping_coords2d`](https://crates.io/crates/wrapping_coords2d) crate, a utility to manage the landscape by mapping a 2D grid of cells into a vector. Both x and y coordinates wrap around the limits of the grid. As an alternative, you can use [`ameda`](https://docs.rs/ameda/latest/ameda) to manage the landscape without wrapping.

## Why is the software so slow?

By default, [Visual Studio Code] runs the program using the `debug` profile. The profile provides vscode with the essential information to examine and step into model code and memory; in this profile, the [`plotters`] crate conducts a multitude of checks for memory and data safety to carefully produce the model figures; this takes significant time and makes the software slow. To improve development time, model protoypes should use very few scenarios and a short time span. Alternatively, a `return()` instruction can exit the program before producing any figures. Finally, the `no-graphics` feature can be activated in `cargo.toml`. See the *Advanced usage* section above for more details.

When the model is ready to work with more scenarios and long time spans, the best option is to use the `release` profile. For this, the command `cargo run --release` can run on the Command Prompt, PowerShell, or the [Visual Studio Code] console. This profile produces figures and output data significantly faster than the `debug` profile.

## Does this repository use `unsafe` code?

No.

## The scenarios and time_series data structure does not seem to conform to the ECS design.

That is okay. These object-oriented structures are useful for parallel computation using the [`rayon`] crate. These structures represent just a small portion of execution time.

## Other ABM designs and links

AJ (Jay, Zencodes, ajjaic), [`ameda` - Manipulate 2D grid indices](https://docs.rs/ameda/latest/ameda)

Carmine Spanguolo, [Rust-AB -- An Agent Based Simulation engine in Rust](https://github.com/spagnuolocarmine/abm)

Diggory Hardy, [`rand` - A Rust library for random number generation](https://github.com/rust-random/rand)

Francis Tseng, [`rust-sim`- Sketches for rust agent-based modeling framework](https://github.com/frnsys/rust-sim)

Hao Hou, [`plotters`] - A Rust drawing library

Josh Stone, Niko Matsakis, [`Rayon`] - A data parallelism library for Rust

Nikolay Kim, [`actix` - Actor framework for Rust](https://github.com/actix/actix)

Orson Peters, [`slotmap` - Data structure for Rust](https://github.com/orlp/slotmap)

Wilensky, U. 1999. [NetLogo](http://ccl.northwestern.edu/netlogo/). Center for Connected Learning and Computer-Based Modeling, Northwestern University. Evanston, IL.

## References

West, Catherine (2018), ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html), RustConf 2018 closing keynote.

Antelmi A., Cordasco G., D’Auria M., De Vinco D., Negro A., Spagnuolo C. (2019, October) On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations. In: Tan G., Lehmann A., Teo Y., Cai W. (eds) Methods and Applications for Modeling and Simulation of Complex Systems. In Asian Simulation Conference AsiaSim 2019 (pp. 15-28). Communications in Computer and Information Science, vol 1094. Springer, Singapore. https://doi.org/10.1007/978-981-15-1078-6_2

[Better TOML extension]:https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
[Cargo's features]:https://doc.rust-lang.org/cargo/reference/features.html
[Clone]:https://code.visualstudio.com/docs/editor/versioncontrol#_cloning-a-repository
[FFmpeg]:https://www.ffmpeg.org/download.html
[`plotters`]:https://crates.io/crates/plotters
[`rayon`]:https://github.com/rayon-rs/rayon
[Rust]:https://www.rust-lang.org
[Rust (rls) extension]:https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
[Rust Clippy tool]:https://github.com/rust-lang/rust-clippy
[Rust's conditional compilation]:https://doc.rust-lang.org/reference/conditional-compilation.html
[Visual Studio Code]:https://code.visualstudio.com/
