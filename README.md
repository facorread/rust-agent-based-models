# rust-agent-based-models

Reliable and efficient agent-based models in Rust

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/rust-agent-based-models

https://gitlab.com/facorread/rust-agent-based-models

## What is an agent-based model?

Agent-based models (ABM) are computer programs that define agents, virtual entities that imitate the decision-making processes and interactions of real people, animals, neurons, computers, or other individuals. ABMs have a wide range of applications. For example, an ABM can study a flock of birds. The behavior of each virtual bird can be as simple as just trying to fly in close proximity to the nearest neighbor; the software can show that this birds combine their behavior to generate the complex, adaptive patterns of flight of real flocks. An ABM can also study economic behavior: in a virtual society, sellers of goods set a price based on limited information they have about the market, and buyers may bargain based on their own limited information about the market. Even with simple rules, an economic ABM can generate complex patterns that can be useful to understand real macroeconomic trends.

ABMs are computational tools of complexity science. There is a wide range of scholarly literature using ABMs to explore collective behavior of humans, animals, companies, and other complex systems on many scales, from individual to global. ABMs are particularly useful to understand complex adaptive systems; they help social and ecological scientists test hypotheses about human behavior by building artificial societies and virtual human-natural systems. Numerous software suites exist to develop ABMs, such as [NetLogo] and [Repast]. There is also a wide range of video games and board games that are agent-based models.

## Why rust-agent-based-models?

[Rust] is useful to write reliable and efficient software for a wide range of applications such as operating systems, high-performance computing, embedded systems, and the Web. Rust does not rely on complicated tools like interpreters or garbage collectors. Rust encourages a data-driven coding discipline informed by the borrow checker, a compile-time tool to maintain memory integrity. Fighting with the borrow checker for a little while is a good strategy to learn good coding practices.

[Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) is a software architecture to define agents in computer memory. Here is a quick example: a vector of `n` health state variables, a vector of `n` energy levels, a vector of `n` body sizes, and a vector of `n` geographic coordinates exist as four independent data structures implementing `n` entities or agents. The first agent in the system has an index 0, and its four components are `health[0]`, `energy[0]`, `volume[0]`, `coord[0]`. Each agent consists of four components maintained in separate memory regions. The program performs better by processing agent decisions and interactions independently, focusing on just one or two of the four vectors at a time; this is a significant memory speedup. The ECS architecture stands in contrast to the more popular [Object-Oriented Programming (OOP)](https://en.wikipedia.org/wiki/Object-oriented_programming) paradigm, which defines only one vector of `n` agents, where each agent has its four components in the same region of memory.

ECS is a popular design in video games, based on its superior performance and its capacity to adapt to the dynamics of engineering such complicated pieces of software. Catherine West's presentation, ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html) provides the rationale and the broad strokes of an ECS architecture in Rust.

The ECS architecture can give your ABM the flexibility to help you focus on discovery and innovation. This repository implements an agent-based model as an entity-component system that defines a group of agents, a landscape, and an infectious disease. This ABM makes the most of the borrow checker and keeps complications to a minimum. The ABM does not encapsulate or hide model components; instead, all model code resides at ```main()```. As the model grows and evolves, you want to focus on complexity science as opposed to writing and deleting modules, interfaces, or traits to neatly "organize" the model. I have years of experience with the OOP paradigm on C++ and I never managed to get it right. Take a look at ```main.rs``` and ask the question, what proportion of this software is about the science, and what proportion is about managing memory and other housekeeping?

After you have prototyped your ABM on [NetLogo], [Repast], or other popular framework, consider using an ECS design for your high-performance ABM on Rust. Why not use Rust for your whole workflow?

## Getting started

Consider installing the [Chocolatey] package manager to set up a streamlined upgrade experience. After installing Chocolatey, use the following command:

```powershell
choco install git jpegview visualstudio-installer visualstudio2019buildtools vscode
```

This should set you up with [Visual Studio Code], the [FFmpeg] binaries, [git], and [jpegview]. Follow the [Rust] standard installation procedure and then use `rustup` to install the [Rust Clippy tool]. Use Visual Studio Code to install the [Better TOML extension] and the [Rust (rls) extension].

[Clone this repository] (`https://github.com/facorread/rust-agent-based-models.git`) and use your favorite terminal to run `cargo run --release`. Follow the instructions on screen.

The code is an exaggerated example of an infectious model; it implements an infectious disease transmitting between agents, between agents and cells, and across cells.

See an [example video here](https://youtu.be/oYeHyFl1-HY)

Consider using the [wasm-agent-based-models] repository to create interactive versions of your model.

## What can I do with this software?

Ideally you would fork this repository to build your own model; this would give you easy access to bug fixes and enhancements via `git pull` and `git merge`. As an alternative, you can just copy this code and start an ABM in your own repository. Finally, you can implement the ideas in this repository as part of your own design.

If you want to reuse code, for example, to generate links at several stages within a time step, you will need to store the state variables in a `world` struct; see Catherine's presentation.

Use statistical software such as R, Julia, or SPSS to analyze and visualize the output files.

Please send me a short email to let me know if you find any of these ideas useful for your own research or if you believe a different approach or software architecture is necessary.

## Advanced usage

This software uses [Cargo's features] and [Rust's conditional compilation] to enable and disable graphics, the social network, and the landscape. By default, `cargo.toml` activates the `all-graphics` feature. This default is the best option for most cases. This is also the only configuration that undergoes automated testing at GitHub and GitLab.

The selections made at `cargo.toml` enable or disable code at `main.rs` through the `#[cfg(feature = )]` attributes. For example, `#[cfg(feature = "net")]` in `main.rs` enables the social network, including the `links` container and the network dynamics. It is useful to be able to turn the network off when development focuses on the landscape or other aspect of the model. By turning the network on or off, and re-running the program, it is possible to catch errors, compare component dynamics, and visualize different component outcomes. Turning off unnecessary components can also speed up the model. For example, activating `default = ["no-graphics"]` in `Cargo.toml` is useful to perform a parameter sweep on thousands of scenarios and save simulated data in `csv` format without producing numerous `png` images.

The features at Cargo.toml, namely landscape and net, are arbitrary examples based on the structure of the model. Take finer control of development, memory, and performance of your model by introducing features you can disable or enable with just a line of code.

## Why not make a crate?

Generally speaking, the code for an ABM is tightly integrated; take a NetLogo model for example: turtles, patches, and links have no privacy or encapsulation. In this repository, model variables such as ```health```, ```next_health```, ```cell_health```, and ```links``` are integrated as well; it is not worth it to try encapsulate them or restrict access. But this also means that separating the code into library and client is not very practical.

Some individual components of an ABM can exist in independent crates. One of them is Orson Peters' [`slotmap`](https://github.com/orlp/slotmap), an efficient memory manager that reuses space left behind by dying agents. The other is my [`wrapping_coords2d`](https://crates.io/crates/wrapping_coords2d) crate, a utility to manage the landscape by mapping a 2D grid of cells into a vector. Both x and y coordinates wrap around the limits of the grid. As an alternative, you can use [`ameda`](https://docs.rs/ameda/latest/ameda) to manage the landscape without wrapping.

## Why is this software so slow?

By default, [Visual Studio Code] runs the program using the `debug` profile. The profile provides vscode with the essential information to examine and step into model code and memory; in this profile, the [`plotters`] crate conducts a multitude of checks for memory and data safety to carefully produce the model figures; this takes significant time and makes the software slow. To improve development time, model protoypes should use very few scenarios and a short time span. Alternatively, a `return()` instruction can exit the program before producing any figures. Finally, the `no-graphics` feature can be activated in `cargo.toml`. See the *Advanced usage* section above for more details.

When the model is ready to work with more scenarios and long time spans, the best option is to use the `release` profile. For this, the command `cargo run --release` can run on the Command Prompt, PowerShell, or the [Visual Studio Code] console. This profile produces figures and output data significantly faster than the `debug` profile.

## Does this software use `unsafe` code?

Not explicitly.

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

Wilensky, U. 1999. [NetLogo]. Center for Connected Learning and Computer-Based Modeling, Northwestern University. Evanston, IL.

## References

North, MJ, NT Collier, J Ozik, E Tatara, M Altaweel, CM Macal, M Bragen, and P Sydelko, "Complex Adaptive Systems Modeling with [Repast] Simphony", Complex Adaptive Systems Modeling, Springer, Heidelberg, FRG (2013). https://doi.org/10.1186/2194-3206-1-3

West, Catherine (2018), ["Using Rust for Game Development,"](https://kyren.github.io/2018/09/14/rustconf-talk.html), RustConf 2018 closing keynote.

Antelmi A., Cordasco G., D’Auria M., De Vinco D., Negro A., Spagnuolo C. (2019, October) On Evaluating Rust as a Programming Language for the Future of Massive Agent-Based Simulations. In: Tan G., Lehmann A., Teo Y., Cai W. (eds) Methods and Applications for Modeling and Simulation of Complex Systems. In Asian Simulation Conference AsiaSim 2019 (pp. 15-28). Communications in Computer and Information Science, vol 1094. Springer, Singapore. https://doi.org/10.1007/978-981-15-1078-6_2

[Better TOML extension]:https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
[Cargo's features]:https://doc.rust-lang.org/cargo/reference/features.html
[Chocolatey]:https://chocolatey.org/
[Clone this repository]:https://code.visualstudio.com/docs/editor/versioncontrol#_cloning-a-repository
[FFmpeg]:https://www.ffmpeg.org/download.html
[git]:http://git-scm.com
[jpegview]:https://sourceforge.net/projects/jpegview
[NetLogo]:http://ccl.northwestern.edu/netlogo
[`plotters`]:https://crates.io/crates/plotters
[`rayon`]:https://github.com/rayon-rs/rayon
[Repast]:https://repast.github.io/
[Rust]:https://www.rust-lang.org
[Rust (rls) extension]:https://marketplace.visualstudio.com/items?itemName=rust-lang.rust
[Rust Clippy tool]:https://github.com/rust-lang/rust-clippy
[Rust's conditional compilation]:https://doc.rust-lang.org/reference/conditional-compilation.html
[Visual Studio Code]:https://code.visualstudio.com/
[wasm-agent-based-models]:https://github.com/facorread/wasm-agent-based-models
