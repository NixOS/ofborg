//! Statistics emitted by Nix when NIX_SHOW_STATS=1
use separator::Separatable;

use std::collections::HashMap;

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

pub struct EvaluationStatsDiff<'a> {
    left: &'a EvaluationStats,
    right: &'a EvaluationStats,
}

impl<'a> EvaluationStatsDiff<'a> {
    pub fn compare(
        left: &'a EvaluationStats,
        right: &'a EvaluationStats,
    ) -> EvaluationStatsDiff<'a> {
        EvaluationStatsDiff { left, right }
    }

    pub fn markdown(&self) -> String {
        struct Row {
            before: String,
            after: String,
            diff: String,
            diff_pct: String,
        }

        impl Row {
            fn from_u64(left: u64, right: u64) -> Row {
                let (diff, direction): (u64, _) = match left.cmp(&right) {
                    std::cmp::Ordering::Greater => (left - right, "↘ "),
                    std::cmp::Ordering::Less => (right - left, "↗ "),
                    std::cmp::Ordering::Equal => (0, ""),
                };

                let diff_pct: String = if diff > 0 {
                    format!(
                        "{:.2}%",
                        ((right as f64) - (left as f64)) / (left as f64) * 100.0
                    )
                } else {
                    String::from("")
                };

                Row {
                    before: left.separated_string(),
                    after: right.separated_string(),
                    diff: format!("{direction}{}", diff.separated_string()),
                    diff_pct,
                }
            }

            fn from_f32(left: f32, right: f32) -> Row {
                let (diff, direction): (f32, _) = match left
                    .partial_cmp(&right)
                    .unwrap_or(std::cmp::Ordering::Equal)
                {
                    std::cmp::Ordering::Greater => (left - right, "↘ "),
                    std::cmp::Ordering::Less => (right - left, "↗ "),
                    std::cmp::Ordering::Equal => (0 as f32, ""),
                };

                let diff_pct: String = if diff > 0 as _ {
                    format!(
                        "{:.2}%",
                        ((right as f64) - (left as f64)) / (left as f64) * 100.0
                    )
                } else {
                    String::from("")
                };

                Row {
                    before: format!("{left:.2}"),
                    after: format!("{right:.2}"),
                    diff: format!("{direction}{diff:.2}"),
                    diff_pct,
                }
            }
        }

        let mut data: HashMap<&str, Row> = HashMap::new();
        data.insert(
            "cpuTime",
            Row::from_f32(self.left.cpu_time, self.right.cpu_time),
        );

        data.insert(
            "envs-number",
            Row::from_u64(self.left.envs.number, self.right.envs.number),
        );
        data.insert(
            "envs-elements",
            Row::from_u64(self.left.envs.elements, self.right.envs.elements),
        );
        data.insert(
            "envs-bytes",
            Row::from_u64(self.left.envs.bytes, self.right.envs.bytes),
        );

        data.insert(
            "list-elements",
            Row::from_u64(self.left.list.elements, self.right.list.elements),
        );
        data.insert(
            "list-bytes",
            Row::from_u64(self.left.list.bytes, self.right.list.bytes),
        );
        data.insert(
            "list-concats",
            Row::from_u64(self.left.list.concats, self.right.list.concats),
        );

        data.insert(
            "values-number",
            Row::from_u64(self.left.values.number, self.right.values.number),
        );
        data.insert(
            "values-bytes",
            Row::from_u64(self.left.values.bytes, self.right.values.bytes),
        );

        data.insert(
            "symbols-number",
            Row::from_u64(self.left.symbols.number, self.right.symbols.number),
        );
        data.insert(
            "symbols-bytes",
            Row::from_u64(self.left.symbols.bytes, self.right.symbols.bytes),
        );

        data.insert(
            "sets-number",
            Row::from_u64(self.left.sets.number, self.right.sets.number),
        );
        data.insert(
            "sets-bytes",
            Row::from_u64(self.left.sets.bytes, self.right.sets.bytes),
        );
        data.insert(
            "sets-elements",
            Row::from_u64(self.left.sets.elements, self.right.sets.elements),
        );

        data.insert(
            "sizes-Env",
            Row::from_u64(self.left.sizes.env, self.right.sizes.env),
        );
        data.insert(
            "sizes-Value",
            Row::from_u64(self.left.sizes.value, self.right.sizes.value),
        );
        data.insert(
            "sizes-Bindings",
            Row::from_u64(self.left.sizes.bindings, self.right.sizes.bindings),
        );
        data.insert(
            "sizes-Attr",
            Row::from_u64(self.left.sizes.attr, self.right.sizes.attr),
        );

        data.insert(
            "nrOpUpdates",
            Row::from_u64(self.left.nr_op_updates, self.right.nr_op_updates),
        );
        data.insert(
            "nrOpUpdateValuesCopied",
            Row::from_u64(
                self.left.nr_op_update_values_copied,
                self.right.nr_op_update_values_copied,
            ),
        );
        data.insert(
            "nrThunks",
            Row::from_u64(self.left.nr_thunks, self.right.nr_thunks),
        );
        data.insert(
            "nrAvoided",
            Row::from_u64(self.left.nr_avoided, self.right.nr_avoided),
        );
        data.insert(
            "nrLookups",
            Row::from_u64(self.left.nr_lookups, self.right.nr_lookups),
        );
        data.insert(
            "nrPrimOpCalls",
            Row::from_u64(self.left.nr_prim_op_calls, self.right.nr_prim_op_calls),
        );
        data.insert(
            "nrFunctionCalls",
            Row::from_u64(self.left.nr_function_calls, self.right.nr_function_calls),
        );
        data.insert(
            "gc-heapSize",
            Row::from_u64(self.left.gc.heap_size, self.right.gc.heap_size),
        );
        data.insert(
            "gc-totalBytes",
            Row::from_u64(self.left.gc.total_bytes, self.right.gc.total_bytes),
        );

        let (keylen, beforelen, afterlen, difflen, diff_pctlen): (
            usize,
            usize,
            usize,
            usize,
            usize,
        ) = data.iter().fold(
            (0, 0, 0, 0, 0),
            |(keylen, before, after, diff, diff_pct), (key, row)| {
                (
                    std::cmp::max(keylen, key.chars().count()),
                    std::cmp::max(before, row.before.chars().count()),
                    std::cmp::max(after, row.after.chars().count()),
                    std::cmp::max(diff, row.diff.chars().count()),
                    std::cmp::max(diff_pct, row.diff_pct.chars().count()),
                )
            },
        );

        let mut keys = data.keys().cloned().collect::<Vec<&str>>();
        keys.sort_unstable();

        let rows = keys
            .into_iter()
            .map(|key| {
                let Row {
                    before,
                    after,
                    diff,
                    diff_pct,
                } = &data[&key];
                let key = format!("**{key}**");
                let keywidth = keylen + 4;
                format!("| {key:<keywidth$} | {before:>beforelen$} | {after:>afterlen$} | {diff:<difflen$} | {diff_pct:>diff_pctlen$} |")
            })
            .collect::<Vec<String>>();

        let header = format!(
"
|{key:^keywidth$}|{before:^beforewidth$}|{after:^afterwidth$}|{diff:^diffwidth$}|{diff_pct:^diff_pctwidth$}|
|{keydash:-<keywidth$}|{beforedash:->beforewidth$}|{afterdash:->afterwidth$}|{diffdash:-<diffwidth$}|{diff_pctdash:->diff_pctwidth$}|
",
            key="stat", keywidth=(keylen + 6),
            before="before", beforewidth=(beforelen + 2),
            after="after", afterwidth=(afterlen + 2),
            diff="Δ", diffwidth=(difflen + 2),
            diff_pct="Δ%", diff_pctwidth=(diff_pctlen + 2),
            keydash=":", beforedash=":", afterdash=":", diffdash=":", diff_pctdash=":"
        );

        format!("{}\n{}", header.trim(), rows.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluationStats;
    use super::EvaluationStatsDiff;
    use serde_json;

    const EXAMPLE: &str = r#"
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
"#;

    const EXAMPLE2: &str = r#"
{
  "cpuTime": 132.897,
  "envs": {
    "number": 124766593,
    "elements": 177627124,
    "bytes": 3417282480
  },
  "list": {
    "elements": 204449868,
    "bytes": 1635598944,
    "concats": 6988658
  },
  "values": {
    "number": 244542804,
    "bytes": 5869027296
  },
  "symbols": {
    "number": 372917,
    "bytes": 16324250
  },
  "sets": {
    "number": 27307373,
    "bytes": 7133945368,
    "elements": 288145266
  },
  "sizes": {
    "Env": 16,
    "Value": 24,
    "Bindings": 8,
    "Attr": 24
  },
  "nrOpUpdates": 11881928,
  "nrOpUpdateValuesCopied": 208814478,
  "nrThunks": 167655588,
  "nrAvoided": 170493166,
  "nrLookups": 75275349,
  "nrPrimOpCalls": 80373629,
  "nrFunctionCalls": 109822957,
  "gc": {
    "heapSize": 11433721856,
    "totalBytes": 23468008832
  }
}
"#;

    #[test]
    fn verify_load() {
        let load: EvaluationStats = serde_json::from_str(EXAMPLE).unwrap();

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

    fn diff_text(left: &str, right: &str) {
        println!("left:\n{left}");
        println!("right:\n{right}");

        let lines = left.split('\n').zip(right.split('\n'));

        for (idx, (linea, lineb)) in lines.enumerate() {
            assert_eq!(linea, lineb, "Line {}", idx);
        }
    }

    #[test]
    fn markdown() {
        let left: EvaluationStats = serde_json::from_str(EXAMPLE).unwrap();
        let right: EvaluationStats = serde_json::from_str(EXAMPLE2).unwrap();

        diff_text(
            &EvaluationStatsDiff::compare(&left, &right).markdown(),
            r#"
|            stat            |     before     |     after      |       Δ       |   Δ%   |
|:---------------------------|---------------:|---------------:|:--------------|-------:|
| **cpuTime**                |         135.20 |         132.90 | ↘ 2.30        | -1.70% |
| **envs-bytes**             |  3,563,057,008 |  3,417,282,480 | ↘ 145,774,528 | -4.09% |
| **envs-elements**          |    183,953,876 |    177,627,124 | ↘ 6,326,752   | -3.44% |
| **envs-number**            |    130,714,125 |    124,766,593 | ↘ 5,947,532   | -4.55% |
| **gc-heapSize**            | 12,104,687,616 | 11,433,721,856 | ↘ 670,965,760 | -5.54% |
| **gc-totalBytes**          | 24,191,819,392 | 23,468,008,832 | ↘ 723,810,560 | -2.99% |
| **list-bytes**             |  1,659,372,128 |  1,635,598,944 | ↘ 23,773,184  | -1.43% |
| **list-concats**           |      7,194,150 |      6,988,658 | ↘ 205,492     | -2.86% |
| **list-elements**          |    207,421,516 |    204,449,868 | ↘ 2,971,648   | -1.43% |
| **nrAvoided**              |    177,840,681 |    170,493,166 | ↘ 7,347,515   | -4.13% |
| **nrFunctionCalls**        |    115,193,164 |    109,822,957 | ↘ 5,370,207   | -4.66% |
| **nrLookups**              |     75,292,052 |     75,275,349 | ↘ 16,703      | -0.02% |
| **nrOpUpdateValuesCopied** |    208,834,564 |    208,814,478 | ↘ 20,086      | -0.01% |
| **nrOpUpdates**            |     11,883,339 |     11,881,928 | ↘ 1,411       | -0.01% |
| **nrPrimOpCalls**          |     85,571,252 |     80,373,629 | ↘ 5,197,623   | -6.07% |
| **nrThunks**               |    173,325,665 |    167,655,588 | ↘ 5,670,077   | -3.27% |
| **sets-bytes**             |  7,134,676,648 |  7,133,945,368 | ↘ 731,280     | -0.01% |
| **sets-elements**          |    288,174,680 |    288,145,266 | ↘ 29,414      | -0.01% |
| **sets-number**            |     27,310,541 |     27,307,373 | ↘ 3,168       | -0.01% |
| **sizes-Attr**             |             24 |             24 | 0             |        |
| **sizes-Bindings**         |              8 |              8 | 0             |        |
| **sizes-Env**              |             16 |             16 | 0             |        |
| **sizes-Value**            |             24 |             24 | 0             |        |
| **symbols-bytes**          |     16,324,262 |     16,324,250 | ↘ 12          | -0.00% |
| **symbols-number**         |        372,918 |        372,917 | ↘ 1           | -0.00% |
| **values-bytes**           |  6,250,904,880 |  5,869,027,296 | ↘ 381,877,584 | -6.11% |
| **values-number**          |    260,454,370 |    244,542,804 | ↘ 15,911,566  | -6.11% |
"#
            .trim_start(),
        );
    }
}
