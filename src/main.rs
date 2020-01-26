/* RustAgentModels: Reliable and efficient agent-based models in Rust

    Copyright 2020 Fabio A. Correa Duran facorread@gmail.com

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#[derive(Clone, Copy)]
enum Health {
    S,
    I,
    R
}

fn main() {
    // Model parameters
    let N0: usize = 1000; // Initial number of agents

    // Model data: agents, patches, links
    // This software uses the Entity-Component-System architecture
    let mut health = slotmap::SlotMap::with_capacity(2 * N0);
    health.insert(Health::S);

    println!("Hello, world!");
}
