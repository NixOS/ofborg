/// Statistics emitted by Nix when NIX_SHOW_STATS=1

#[derive(Deserialize)]
pub struct EvaluationStats {
    /// Number of CPU seconds spent during evaluation.
    #[serde(rename = "cpuTime")]
    pub cpu_time: f32,

    pub envs: Environments,
    pub list: Lists,
    pub values: Values,
    pub symbols: Symbols,
    pub sets: Sets,
    pub sizes: Sizes,
    pub gc: GarbageCollector,

    #[serde(rename = "nrOpUpdates")]
    pub nr_op_updates: u64,
    #[serde(rename = "nrOpUpdateValuesCopied")]
    pub nr_op_update_values_copied: u64,
    #[serde(rename = "nrThunks")]
    pub nr_thunks: u64,
    #[serde(rename = "nrAvoided")]
    pub nr_avoided: u64,
    #[serde(rename = "nrLookups")]
    pub nr_lookups: u64,
    #[serde(rename = "nrPrimOpCalls")]
    pub nr_prim_op_calls: u64,
    #[serde(rename = "nrFunctionCalls")]
    pub nr_function_calls: u64,
}

#[derive(Deserialize)]
pub struct Environments {
    pub number: u64,
    pub elements: u64,
    pub bytes: u64,
}

#[derive(Deserialize)]
pub struct Lists {
    pub elements: u64,

    /// Number of bytes consumed
    pub bytes: u64,
    pub concats: u64,
}

#[derive(Deserialize)]
pub struct Values {
    pub number: u64,

    /// Number of bytes consumed
    pub bytes: u64,
}

#[derive(Deserialize)]
pub struct Symbols {
    pub number: u64,

    /// Number of bytes consumed
    pub bytes: u64,
}

#[derive(Deserialize)]
pub struct Sets {
    pub number: u64,
    pub elements: u64,

    /// Number of bytes consumed
    pub bytes: u64,
}

#[derive(Deserialize)]
pub struct Sizes {
    #[serde(rename = "Env")]
    pub env: u64,

    #[serde(rename = "Value")]
    pub value: u64,

    #[serde(rename = "Bindings")]
    pub bindings: u64,

    #[serde(rename = "Attr")]
    pub attr: u64,
}

#[derive(Deserialize)]
pub struct GarbageCollector {
    #[serde(rename = "heapSize")]
    pub heap_size: u64,
    #[serde(rename = "totalBytes")]
    pub total_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::EvaluationStats;
    use serde_json;
    #[test]
    fn verify_load() {
        let load: EvaluationStats = serde_json::from_str(
            r#"
{
  "cpuTime": 135.2,
  "envs": {
    "number": 130714125,
    "elements": 183953876,
    "bytes": 3563057008
  },
  "list": {
    "elements": 207421516,
    "bytes": 1659372128,
    "concats": 7194150
  },
  "values": {
    "number": 260454370,
    "bytes": 6250904880
  },
  "symbols": {
    "number": 372918,
    "bytes": 16324262
  },
  "sets": {
    "number": 27310541,
    "bytes": 7134676648,
    "elements": 288174680
  },
  "sizes": {
    "Env": 16,
    "Value": 24,
    "Bindings": 8,
    "Attr": 24
  },
  "nrOpUpdates": 11883339,
  "nrOpUpdateValuesCopied": 208834564,
  "nrThunks": 173325665,
  "nrAvoided": 177840681,
  "nrLookups": 75292052,
  "nrPrimOpCalls": 85571252,
  "nrFunctionCalls": 115193164,
  "gc": {
    "heapSize": 12104687616,
    "totalBytes": 24191819392
  }
}
"#,
        )
        .unwrap();

        assert_eq!(load.cpu_time, 135.2);
        assert_eq!(load.envs.number, 130714125);
        assert_eq!(load.envs.elements, 183953876);
        assert_eq!(load.envs.bytes, 3563057008);

        assert_eq!(load.list.elements, 207421516);
        assert_eq!(load.list.bytes, 1659372128);
        assert_eq!(load.list.concats, 7194150);

        assert_eq!(load.values.number, 260454370);
        assert_eq!(load.values.bytes, 6250904880);

        assert_eq!(load.symbols.number, 372918);
        assert_eq!(load.symbols.bytes, 16324262);

        assert_eq!(load.sets.number, 27310541);
        assert_eq!(load.sets.bytes, 7134676648);
        assert_eq!(load.sets.elements, 288174680);

        assert_eq!(load.sizes.env, 16);
        assert_eq!(load.sizes.value, 24);
        assert_eq!(load.sizes.bindings, 8);
        assert_eq!(load.sizes.attr, 24);

        assert_eq!(load.nr_op_updates, 11883339);
        assert_eq!(load.nr_op_update_values_copied, 208834564);
        assert_eq!(load.nr_thunks, 173325665);
        assert_eq!(load.nr_avoided, 177840681);
        assert_eq!(load.nr_lookups, 75292052);
        assert_eq!(load.nr_prim_op_calls, 85571252);
        assert_eq!(load.nr_function_calls, 115193164);

        assert_eq!(load.gc.heap_size, 12104687616);
        assert_eq!(load.gc.total_bytes, 24191819392);
    }
}
