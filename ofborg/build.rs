use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

enum MetricType {
    Ticker(Metric),
    Counter(Metric),
}

impl MetricType {
    fn collector_type(&self) -> String {
        match self {
            MetricType::Ticker(_) => String::from("u64"),
            MetricType::Counter(_) => String::from("u64"),
        }
    }

    fn enum_matcher_types(&self) -> String {
        let fields = self.enum_field_types();

        if !fields.is_empty() {
            format!("{}({})", self.variant(), fields.join(", "))
        } else {
            self.variant()
        }
    }

    fn variant(&self) -> String {
        match self {
            MetricType::Ticker(ref event) => event.variant.clone(),
            MetricType::Counter(ref event) => event.variant.clone(),
        }
    }

    fn metric_type(&self) -> String {
        match self {
            MetricType::Ticker(_) => String::from("counter"),
            MetricType::Counter(_) => String::from("counter"),
        }
    }

    fn metric_name(&self) -> String {
        match self {
            MetricType::Ticker(ref event) => event.metric_name.clone(),
            MetricType::Counter(ref event) => event.metric_name.clone(),
        }
    }

    fn description(&self) -> String {
        match self {
            MetricType::Ticker(ref event) => event.description.clone(),
            MetricType::Counter(ref event) => event.description.clone(),
        }
    }

    fn enum_index_types(&self) -> Vec<String> {
        let event: &Metric = match self {
            MetricType::Ticker(ref i_event) => i_event,
            MetricType::Counter(ref i_event) => i_event,
        };

        let fields: Vec<String> = event
            .fields
            .iter()
            .map(|(_fieldname, fieldtype)| fieldtype.clone())
            .collect();

        fields
    }

    fn enum_field_types(&self) -> Vec<String> {
        let mut extra_fields: Vec<String> = vec![];

        match self {
            MetricType::Ticker(_) => {}
            MetricType::Counter(_) => {
                extra_fields = vec![self.collector_type()];
            }
        }

        let mut fields: Vec<String> = self.enum_index_types();
        fields.append(&mut extra_fields);

        fields
    }

    fn enum_index_names(&self) -> Vec<String> {
        let event: &Metric = match self {
            MetricType::Ticker(ref i_event) => i_event,
            MetricType::Counter(ref i_event) => i_event,
        };

        let fields: Vec<String> = event
            .fields
            .iter()
            .map(|(fieldname, _fieldtype)| fieldname.clone())
            .collect();

        fields
    }

    fn enum_field_names(&self) -> Vec<String> {
        let mut extra_fields: Vec<String> = vec![];

        match self {
            MetricType::Ticker(_) => {}
            MetricType::Counter(_) => {
                extra_fields = vec!["value".to_owned()];
            }
        }

        let mut fields: Vec<String> = self.enum_index_names();
        fields.append(&mut extra_fields);

        fields
    }

    fn record_value(&self) -> String {
        match self {
            MetricType::Ticker(_) => String::from("1"),
            MetricType::Counter(_) => String::from("value"),
        }
    }
}

struct Metric {
    variant: String,
    fields: Vec<(String, String)>, // Vec because it is sorted
    metric_name: String,
    description: String,
}

fn name_to_parts(name: &str) -> Vec<String> {
    let mut parts: Vec<String> = vec![];
    let mut buf = String::from("");
    for c in name.chars() {
        if char::is_uppercase(c) && !buf.is_empty() {
            parts.push(buf.to_owned());
            buf = String::from("");
        }
        buf.push(c);
    }
    if !buf.is_empty() {
        parts.push(buf.to_owned());
        std::mem::drop(buf);
    }

    parts
}

impl Metric {
    pub fn ticker(name: &str, desc: &str, fields: Option<Vec<(&str, &str)>>) -> MetricType {
        let parts = name_to_parts(name);

        MetricType::Ticker(Metric {
            variant: parts.iter().cloned().collect(),
            fields: fields
                .unwrap_or_default()
                .iter()
                .map(|(fieldname, fieldtype)| ((*fieldname).to_string(), (*fieldtype).to_string()))
                .collect(),
            metric_name: parts.join("_").to_lowercase(),
            description: desc.to_owned(),
        })
    }

    pub fn counter(name: &str, desc: &str, fields: Option<Vec<(&str, &str)>>) -> MetricType {
        let parts = name_to_parts(name);

        MetricType::Counter(Metric {
            variant: parts.iter().cloned().collect(),
            fields: fields
                .unwrap_or_default()
                .iter()
                .map(|(fieldname, fieldtype)| ((*fieldname).to_string(), (*fieldtype).to_string()))
                .collect(),
            metric_name: parts.join("_").to_lowercase(),
            description: desc.to_owned(),
        })
    }
}

fn events() -> Vec<MetricType> {
    vec![
        Metric::ticker(
            "StatCollectorLegacyEvent",
            "Number of received legacy events",
            Some(vec![("event", "String")]),
        ),
        Metric::ticker(
            "StatCollectorBogusEvent",
            "Number of received unparseable events",
            None,
        ),
        Metric::ticker("JobReceived", "Number of received worker jobs", None),
        Metric::counter(
            "EvaluationDuration",
            "Amount of time spent running evaluations",
            Some(vec![("branch", "String")]),
        ),
        Metric::ticker(
            "EvaluationDurationCount",
            "Number of timed evaluations performed",
            Some(vec![("branch", "String")]),
        ),
        Metric::ticker(
            "TargetBranchFailsEvaluation",
            "Number of PR evaluations which failed because the target branch failed",
            Some(vec![("branch", "String")]),
        ),
        Metric::ticker(
            "JobDecodeSuccess",
            "Number of successfully decoded jobs",
            None,
        ),
        Metric::ticker(
            "JobDecodeFailure",
            "Number of jobs which failed to parse",
            None,
        ),
        Metric::ticker(
            "IssueAlreadyClosed",
            "Number of jobs for issues which are already closed",
            None,
        ),
        Metric::ticker(
            "IssueFetchFailed",
            "Number of failed fetches for GitHub issues",
            None,
        ),
        Metric::ticker(
            "TaskEvaluationCheckComplete",
            "Number of completed evaluation tasks",
            None,
        ),
        /*
        Metric::counter(
            "TimeElapsed",
            "",
            None
        ),
        Metric::counter(
            "EnvironmentsAllocatedCount",
            "",
            None
        ),
        Metric::counter(
            "EnvironmentsAllocatedBytes",
            "",
            None
        ),
        Metric::counter(
            "ListElementsCount",
            "",
            None
        ),
        Metric::counter(
            "ListElementsBytes",
            "",
            None
        ),
        Metric::counter(
            "ListConcatenations",
            "",
            None
        ),
        Metric::counter(
            "ValuesAllocatedCount",
            "",
            None
        ),
        Metric::counter(
            "ValuesAllocatedBytes",
            "",
            None
        ),
        Metric::counter(
            "SetsAllocatedCount",
            "",
            None
        ),
        Metric::counter(
            "SetsAllocatedBytes",
            "",
            None
        ),
        Metric::counter(
            "RightBiasedUnions",
            "",
            None
        ),
        Metric::counter(
            "ValuesCopiedInRightBiasedUnions",
            "",
            None
        ),
        Metric::counter(
            "SymbolsInSymbolTable",
            "",
            None
        ),
        Metric::counter(
            "SizeOfSymbolTable",
            "",
            None
        ),
        Metric::counter(
            "NumberOfThunks",
            "",
            None
        ),
        Metric::counter(
            "NumberOfThunksAvoided",
            "",
            None
        ),
        Metric::counter(
            "NumberOfAttrLookups",
            "",
            None
        ),
        Metric::counter(
            "NumberOfPrimopCalls",
            "",
            None
        ),
        Metric::counter(
            "NumberOfFunctionCalls",
            "",
            None
        ),
        Metric::counter(
            "TotalAllocations",
            "",
            None
        ),
        Metric::counter(
            "CurrentBoehmHeapSizeBytes",
            "",
            None
        ),
        Metric::counter(
            "TotalBoehmHeapAllocationsBytes",
            "",
            None
        ),
        */
    ]
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("events.rs");
    let mut f = File::create(dest_path).unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    // Write the Event enum, which contains all possible event types
    f.write_all(
        b"
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all=\"kebab-case\")]
pub enum Event {
",
    )
    .unwrap();

    let variants: Vec<String> = events()
        .iter()
        .map(|mtype| format!("  {}", mtype.enum_matcher_types()))
        .collect();

    f.write_all(variants.join(",\n").as_bytes()).unwrap();
    f.write_all(b"\n}\n\n").unwrap();

    f.write_all(
        b"pub fn event_metric_name(event: &Event) -> String {
  match *event {
",
    )
    .unwrap();

    let variants: Vec<String> = events()
        .iter()
        .map(|mtype| {
            let fields: Vec<String> = mtype
                .enum_field_names()
                .iter()
                .map(|_| String::from("_"))
                .collect();

            let variant_match = if !fields.is_empty() {
                format!("{}({})", &mtype.variant(), fields.join(", "))
            } else {
                mtype.variant()
            };

            format!(
                "    Event::{} => String::from(\"{}\")",
                &variant_match,
                &mtype.metric_name(),
            )
        })
        .collect();

    f.write_all(variants.join(",\n").as_bytes()).unwrap();
    f.write_all(b"}\n  }").unwrap();

    // Create a struct to hold all the possible metrics
    f.write_all(
        b"
#[derive(Default, Clone)]
pub struct MetricCollector {
",
    )
    .unwrap();

    let variants: Vec<String> = events()
        .iter()
        .map(|mtype| {
            let mut fields: Vec<String> = mtype.enum_index_types();
            fields.push("String".to_owned()); // Instance
            let fields_str = {
                let s = fields.join(", ");
                if fields.len() > 1 {
                    format!("({s})")
                } else {
                    s
                }
            };

            format!(
                "  {}: Arc<Mutex<HashMap<{},{}>>>",
                mtype.metric_name(),
                fields_str,
                mtype.collector_type(),
            )
        })
        .collect();

    f.write_all(variants.join(",\n").as_bytes()).unwrap();
    f.write_all(b"\n}\n\n").unwrap();

    // Create a struct to hold all the possible metrics
    f.write_all(
        b"

impl MetricCollector {
  pub fn new() -> MetricCollector {
    Default::default()
  }

  pub fn record(&self, instance: String, event: Event) {
    match event {
",
    )
    .unwrap();

    let variants: Vec<String> = events()
        .iter()
        .map(|mtype| {
            let fields: Vec<String> = mtype.enum_field_names();

            let variant_match = if !fields.is_empty() {
                format!("{}({})", &mtype.variant(), fields.join(", "))
            } else {
                mtype.variant()
            };

            let mut index_names: Vec<String> = mtype.enum_index_names();
            index_names.push("instance".to_owned());

            let mut index_fields = index_names.join(", ");
            if index_names.len() > 1 {
                index_fields = format!("({index_fields})");
            }

            format!(
                "
      Event::{} => {{
        let mut accum_table = self.{}
          .lock()
          .expect(\"Failed to unwrap metric mutex for {}\");
        let accum = accum_table
          .entry({})
          .or_insert(0);
        *accum += {};
      }}
 ",
                variant_match,
                &mtype.metric_name(),
                &mtype.metric_name(),
                index_fields,
                &mtype.record_value(),
            )
        })
        .collect();

    f.write_all(variants.join(",\n").as_bytes()).unwrap();
    f.write_all(b"\n    }\n").unwrap();
    f.write_all(b"\n  }\n").unwrap();

    f.write_all(
        b"pub fn prometheus_output(&self) -> String {
    let mut output = String::new();
",
    )
    .unwrap();

    let variants: Vec<String> = events()
        .iter()
        .map(|mtype| {
            let mut index_fields: Vec<String> = mtype.enum_index_names();
            index_fields.push("instance".to_owned());
            let ref_index_fields: Vec<String> = index_fields.clone();

            let for_matcher = if index_fields.len() > 1 {
                format!("({})", ref_index_fields.join(", "))
            } else {
                ref_index_fields.join(", ")
            };

            let key_value_pairs: Vec<String> = index_fields
                .iter()
                .map(|name| format!("            format!(\"{name}=\\\"{{}}\\\"\", {name})",))
                .collect();
            format!(
                "
      output.push_str(\"# HELP ofborg_{} {}\n\");
      output.push_str(\"# TYPE ofborg_{} {}\n\");

      let table = self.{}.lock()
        .expect(\"Failed to unwrap metric mutex for {}\");
      let values: Vec<String> = (*table)
        .iter()
        .map(|({}, value)| {{
          let kvs: Vec<String> = vec![
{}
          ];
          format!(\"ofborg_{}{{{{{{}}}}}} {{}}\", kvs.join(\",\"), value)
        }})
        .collect();
      output.push_str(&values.join(\"\n\"));
      output.push('\\n');
 ",
                &mtype.metric_name(),
                &mtype.description(),
                &mtype.metric_name(),
                &mtype.metric_type(),
                &mtype.metric_name(),
                &mtype.metric_name(),
                for_matcher,
                &key_value_pairs.join(",\n"),
                &mtype.metric_name(),
            )
        })
        .collect();

    f.write_all(variants.join("\n").as_bytes()).unwrap();
    f.write_all(b"output\n  }").unwrap();
    f.write_all(b"\n}").unwrap();
}
