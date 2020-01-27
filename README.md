# rust-agent-based-models

Reliable and efficient agent-based models in Rust

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/rust-agent-based-models
https://gitlab.com/facorread/rust-agent-based-models

## Why?

Agent-based models (ABM) are a computational tool of complexity science, particularly in the frameworks of complex adaptive systems, coupled human and natural systems, and computational social science.

Rust is useful to write safe, fast software that removes most complications. The necessary compromise is the borrow checker, which prevents us from using patterns that and hurt our research. Fighting with the borrow checker for a little while makes us adopt good practices.

Consider Rust over C or C++ for your next ABM.

## What is the design?

ABMs have similarities with computer games. I used Catherine's West presentation, "Using Rust for Game Development," as a starting point for the architecture of this software. The highlights are the Entity-Component-System (ECS) architecture and working to turn the borrow checker into your friend.

## Why not make a crate?

The code for an ABM is tightly integrated. In this repository, the health, next_health, and links variables interact so much with one another that it is not worth it to encapsalate o restrict access. This also means that separating the code into library and client is not feasible. In order to maintain a tidy environment, your best approach is to keep the modeling components as public entities in ```fn main()```.