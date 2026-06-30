/*
 * WaultNet Core
 * Copyright (C) 2026 WaultNet Collective
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 or later of the License.
 */

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub compute_units: u64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, units: u64) -> Self {
        Self { sender, receiver, compute_units: units }
    }
}