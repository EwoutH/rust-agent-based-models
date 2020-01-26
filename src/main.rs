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

use rand::distributions::{Bernoulli, Distribution};
use std::fs;
// use std::fmt::Write as FmtWrite; // See https://doc.rust-lang.org/std/macro.writeln.html
use std::io::Write as IoWrite; // See https://doc.rust-lang.org/std/macro.writeln.html

// Model properties
#[derive(Clone, Copy)]
enum Health {
    S,
    I
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
    let mut health = slotmap::SlotMap::with_capacity_and_key(2 * n0);

    // Health status of agents in the next time step
    let mut next_health = slotmap::SecondaryMap::with_capacity(health.capacity());

    // Bidirectional links between agents
    let mut links = slotmap::SlotMap::with_capacity_and_key(n0 * n0);

    // This is the seed for a scale-free network: Two agents with a link
    {
        let key0: AgentKey = health.insert(Health::S);
        let key1 = health.insert(Health::S);
        let _link_id: LinkKey = links.insert((key0, key1));
        next_health.insert(key0, Health::S);
        next_health.insert(key1, Health::S);
    }

    let survival_distro = Bernoulli::new(0.3).unwrap();
    let mut ts_file = fs::File::create("ts.csv").expect("Unable to create time series output file");
    writeln!(&mut ts_file, "Time step, Number of agents N, Susceptibles S, Infected I").expect("Error writing time series output file");
    let mut time_step = 0;
    loop {
        // Model measurements
        {
            let mut s = 0;
            let mut i = 0;
            for h in health.values() {
                match h {
                    Health::S => s = s + 1,
                    Health::I => i = i + 1
                }
            }
            writeln!(&mut ts_file, "{},{},{},{}", time_step, health.len(), s, i).expect("Error writing time series output file");
        }

        time_step = time_step + 1;
        if time_step == 1000 {
            break;
        }

        // Model dynamics
        // Infection spreads

        // After spreading the infection, some infectious agents die
        health.retain(|_agent_key, h| match h {
            Health::S => true,
            Health::I => survival_distro.sample(&mut rand::thread_rng())
        });

        // Remaining agents update in parallel

        // Prune network

        // New agents emerge

        // New links emerge
    }

    println!("Hello, world!");
}
