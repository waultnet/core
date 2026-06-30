/*
 * Waultnet Core
 * Copyright (C) 2026 Waultnet Collective
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 or later of the License.
 */

use waultnet_core::transaction::Transaction;

fn main() {
    println!("WaultNet Core initialization...");
    let tx = Transaction::new("node_a".to_string(), "node_b".to_string(), 100);
    println!("Example transaction created: {:?}", tx);
}