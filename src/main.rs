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
use rand::distributions::{weighted::WeightedIndex, Bernoulli, Distribution};
use slotmap::{SecondaryMap, SlotMap};
use std::fs;
// use std::fmt::Write as FmtWrite; // See https://doc.rust-lang.org/std/macro.writeln.html
use std::io::Write as IoWrite; // See https://doc.rust-lang.org/std/macro.writeln.html

// Model properties
#[derive(Clone, Copy, PartialEq)]
enum Health {
    S,
    I,
}

// Housekeeping
slotmap::new_key_type! {
    struct AgentKey;
    struct LinkKey;
}

fn main() {
    // Model parameter: Initial number of agents
    let n0: usize = 1000;
    let net_k: usize = 7;
    // Model state: Agent health
    let mut health = SlotMap::with_capacity_and_key(2 * n0);
    // Model state: Bidirectional links between agents
    let mut links = slotmap::SlotMap::with_capacity_and_key(n0 * n0);
    // This is the seed for a scale-free network: Two agents with a link
    while health.len() < n0 {
        let _k: AgentKey = health.insert(Health::S);
    }
    let birth_distro = Bernoulli::new(0.01).unwrap();
    let infection_distro = Bernoulli::new(0.3).unwrap();
    let initial_infection_distro = Bernoulli::new(0.3).unwrap();
    let link_distro = Bernoulli::new(0.01).unwrap();
    let recovery_distro = Bernoulli::new(0.3).unwrap();
    let survival_distro = Bernoulli::new(0.3).unwrap();
    let mut ts_file = fs::File::create("ts.csv").expect("Unable to create time series output file");
    writeln!(&mut ts_file, "Time step, Number of agents n, Susceptibles s, Infected i, Maximum network degree d_max, Average degree of susceptibles d_s, Average degree of infectious d_i").expect("Error writing time series output file");
    let mut rng = rand::thread_rng();
    let mut time_step = 0;
    loop {
        println!("\r                                                                         \rtime_step = {}", time_step);
        // Initialization of this time step: Network seed
        if links.is_empty() && health.len() > 1 {
            let mut h_it = health.iter();
            let (key0, _value) = h_it.next().unwrap();
            let (key1, _value) = h_it.next().unwrap();
            let _link_id: LinkKey = links.insert((key0, key1));
        }
        {
            // Initialization of this time step: Network
            let agent_key_vec: Vec<AgentKey> = health.keys().collect();
            let mut weights_vec: Vec<i32> = {
                let mut weights_map = SecondaryMap::with_capacity(health.capacity());
                agent_key_vec.iter().for_each(|&k| {
                    let _ = weights_map.insert(k, 0);
                });
                for (key0, key1) in links.values() {
                    weights_map[*key0] += 1;
                    weights_map[*key1] += 1;
                }
                agent_key_vec.iter().map(|k| weights_map[*k]).collect()
            };
            for agent_idx in 0..agent_key_vec.len() {
                if weights_vec[agent_idx] == 0 || link_distro.sample(&mut rng) {
                    let agent_key = agent_key_vec[agent_idx];
                    let mut weights_tmp = weights_vec.clone();
                    weights_tmp[agent_idx] = 0;
                    for (key0, key1) in links.values() {
                        if *key0 == agent_key {
                            weights_tmp[agent_key_vec.iter().position(|&k| k == *key1).unwrap()] =
                                0;
                        }
                        if *key1 == agent_key {
                            weights_tmp[agent_key_vec.iter().position(|&k| k == *key0).unwrap()] =
                                0;
                        }
                    }
                    let dist_result = WeightedIndex::new(weights_tmp);
                    if dist_result.is_ok() {
                        let mut dist = dist_result.unwrap();
                        let mut k = 0;
                        loop {
                            let friend_idx = dist.sample(&mut rng);
                            links.insert((agent_key, agent_key_vec[friend_idx]));
                            weights_vec[agent_idx] += 1;
                            weights_vec[friend_idx] += 1;
                            k += 1;
                            if k == net_k {
                                break;
                            }
                            // Make friend ineligible for a new link
                            if dist.update_weights(&[(friend_idx, &0)]).is_err() {
                                break;
                            }
                        }
                    }
                }
            }
            // Model measurements
            {
                let mut s = 0;
                let mut i = 0;
                for h in health.values() {
                    match h {
                        Health::S => s += 1,
                        Health::I => i += 1,
                    }
                }
                let d_max = weights_vec.iter().copied().max().unwrap_or(0);
                let d_s = match agent_key_vec
                    .iter()
                    .zip(weights_vec.iter())
                    .filter(|(&k, _w)| health[k] == Health::S)
                    .max_by_key(|(_k, &w)| w)
                {
                    Some((_k, &w)) => w,
                    None => 0,
                };
                let d_i = match agent_key_vec
                    .iter()
                    .zip(weights_vec.iter())
                    .filter(|(&k, _w)| health[k] == Health::I)
                    .max_by_key(|(_k, &w)| w)
                {
                    Some((_k, &w)) => w,
                    None => 0,
                };
                writeln!(
                    &mut ts_file,
                    "{},{},{},{},{},{},{}",
                    time_step,
                    health.len(),
                    s,
                    i,
                    d_max,
                    d_s,
                    d_i
                )
                .expect("Error writing time series output file");
            }
        }
        // Dynamics: Time step
        time_step += 1;
        if time_step == 100 {
            break;
        }
        // Dynamics: infection spreads
        {
            // Model state: Agent health the next time step
            let mut next_health = SecondaryMap::with_capacity(health.capacity());
            for (key0, key1) in links.values().copied() {
                let h0 = health[key0];
                let h1 = health[key1];
                if h0 == Health::S && h1 == Health::I && infection_distro.sample(&mut rng) {
                    next_health.insert(key0, Health::I);
                }
                if h1 == Health::S && h0 == Health::I && infection_distro.sample(&mut rng) {
                    next_health.insert(key1, Health::I);
                }
            }
            if time_step == 1 {
                health.iter().for_each(|(k, &h)| {
                    if h == Health::S && initial_infection_distro.sample(&mut rng) {
                        next_health.insert(k, Health::I);
                    }
                });
            }
            health.iter().for_each(|(k, &h)| {
                if h == Health::I && recovery_distro.sample(&mut rng) {
                    next_health.insert(k, Health::S);
                }
            });
            // Dynamics: After spreading the infection, some infectious agents die
            health.retain(|_agent_key, h| match h {
                Health::S => true,
                Health::I => survival_distro.sample(&mut rng),
            });
            // Dynamics: Remaining agents update in parallel
            next_health.iter().for_each(|(k, &next_h)| {
                if let Some(h) = health.get_mut(k) {
                    *h = next_h;
                }
            });
        }
        // Dynamics: Prune network
        links.retain(|_link_key, (key0, key1)| {
            health.contains_key(*key0) && health.contains_key(*key1)
        });
        // Dynamics: New agents emerge
        let nb = health
            .values()
            .filter(|&&h| h == Health::S && birth_distro.sample(&mut rng))
            .count();
        for _ in 0..nb {
            health.insert(Health::S);
        }
    }
    println!("The dataset is ready.");
}
