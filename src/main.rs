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

///! This software uses the Entity-Component-System (ECS) architecture and other principles discussed at https://kyren.github.io/2018/09/14/rustconf-talk.html

// Model properties
#[derive(Clone, Copy)]
enum Health {
    S,
    I,
    R
}

// Housekeeping
slotmap::new_key_type! {
    struct AgentKey;
    struct LinkKey;
}

fn main() {
    // Model parameters
    // Initial number of agents
    let n0: usize = 1000;

    // Health status of agents
    // Index: Agent id 
    let mut health = slotmap::SlotMap::with_capacity_and_key(2 * n0);

    // Bidirectional links between agents
    let mut links = slotmap::SlotMap::with_capacity_and_key(n0 * n0);

    // This is the seed for a scale-free network: Two agents with a link
    {
        let id0: AgentKey = health.insert(Health::S);
        let id1 = health.insert(Health::S);
        let _link_id: LinkKey = links.insert((id0, id1));
        // let mut degree = slotmap::SecondaryMap::with_capacity(2 * n0);
        // degree.insert(id0, 1);
        // degree.insert(id1, 1);
        // while health.len() < n0 {
        //     let newId = health.insert(Health::S);

        // }
    }

    println!("Hello, world!");
}
