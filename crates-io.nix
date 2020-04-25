{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# aho-corasick-0.5.3

  crates.aho_corasick."0.5.3" = deps: { features?(features_.aho_corasick."0.5.3" deps {}) }: buildRustCrate {
    crateName = "aho-corasick";
    version = "0.5.3";
    description = "Fast multiple substring searching with finite state machines.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "1igab46mvgknga3sxkqc917yfff0wsjxjzabdigmh240p5qxqlnn";
    libName = "aho_corasick";
    crateBin =
      [{  name = "aho-corasick-dot"; }];
    dependencies = mapFeatures features ([
      (crates."memchr"."${deps."aho_corasick"."0.5.3"."memchr"}" deps)
    ]);
  };
  features_.aho_corasick."0.5.3" = deps: f: updateFeatures f (rec {
    aho_corasick."0.5.3".default = (f.aho_corasick."0.5.3".default or true);
    memchr."${deps.aho_corasick."0.5.3".memchr}".default = true;
  }) [
    (features_.memchr."${deps."aho_corasick"."0.5.3"."memchr"}" deps)
  ];


# end
# aho-corasick-0.6.9

  crates.aho_corasick."0.6.9" = deps: { features?(features_.aho_corasick."0.6.9" deps {}) }: buildRustCrate {
    crateName = "aho-corasick";
    version = "0.6.9";
    description = "Fast multiple substring searching with finite state machines.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "1lj20py6bvw3y7m9n2nqh0mkshfl1kjfp72lfika9gpkrp2r204l";
    libName = "aho_corasick";
    crateBin =
      [{  name = "aho-corasick-dot";  path = "src/main.rs"; }];
    dependencies = mapFeatures features ([
      (crates."memchr"."${deps."aho_corasick"."0.6.9"."memchr"}" deps)
    ]);
  };
  features_.aho_corasick."0.6.9" = deps: f: updateFeatures f (rec {
    aho_corasick."0.6.9".default = (f.aho_corasick."0.6.9".default or true);
    memchr."${deps.aho_corasick."0.6.9".memchr}".default = true;
  }) [
    (features_.memchr."${deps."aho_corasick"."0.6.9"."memchr"}" deps)
  ];


# end
# amq-proto-0.1.0

  crates.amq_proto."0.1.0" = deps: { features?(features_.amq_proto."0.1.0" deps {}) }: buildRustCrate {
    crateName = "amq-proto";
    version = "0.1.0";
    description = "AMQP/RabbitMQ protocol implementation";
    authors = [ "Andrii Dmytrenko <refresh.xss@gmail.com>" ];
    sha256 = "0333fsph61q9nxbx6h8hdxjmpabjm9vmsfc6q5agy801x35r4ml9";
    dependencies = mapFeatures features ([
      (crates."bit_vec"."${deps."amq_proto"."0.1.0"."bit_vec"}" deps)
      (crates."byteorder"."${deps."amq_proto"."0.1.0"."byteorder"}" deps)
      (crates."enum_primitive"."${deps."amq_proto"."0.1.0"."enum_primitive"}" deps)
      (crates."env_logger"."${deps."amq_proto"."0.1.0"."env_logger"}" deps)
      (crates."error_chain"."${deps."amq_proto"."0.1.0"."error_chain"}" deps)
      (crates."log"."${deps."amq_proto"."0.1.0"."log"}" deps)
    ]);
  };
  features_.amq_proto."0.1.0" = deps: f: updateFeatures f (rec {
    amq_proto."0.1.0".default = (f.amq_proto."0.1.0".default or true);
    bit_vec."${deps.amq_proto."0.1.0".bit_vec}".default = true;
    byteorder."${deps.amq_proto."0.1.0".byteorder}".default = true;
    enum_primitive."${deps.amq_proto."0.1.0".enum_primitive}".default = true;
    env_logger."${deps.amq_proto."0.1.0".env_logger}".default = true;
    error_chain."${deps.amq_proto."0.1.0".error_chain}".default = true;
    log."${deps.amq_proto."0.1.0".log}".default = true;
  }) [
    (features_.bit_vec."${deps."amq_proto"."0.1.0"."bit_vec"}" deps)
    (features_.byteorder."${deps."amq_proto"."0.1.0"."byteorder"}" deps)
    (features_.enum_primitive."${deps."amq_proto"."0.1.0"."enum_primitive"}" deps)
    (features_.env_logger."${deps."amq_proto"."0.1.0"."env_logger"}" deps)
    (features_.error_chain."${deps."amq_proto"."0.1.0"."error_chain"}" deps)
    (features_.log."${deps."amq_proto"."0.1.0"."log"}" deps)
  ];


# end
# antidote-1.0.0

  crates.antidote."1.0.0" = deps: { features?(features_.antidote."1.0.0" deps {}) }: buildRustCrate {
    crateName = "antidote";
    version = "1.0.0";
    description = "Poison-free versions of the standard library Mutex and RwLock types";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "1x2wgaw603jcjwsfvc8s2rpaqjv0aqj8mvws2ahhkvfnwkdf7icw";
  };
  features_.antidote."1.0.0" = deps: f: updateFeatures f (rec {
    antidote."1.0.0".default = (f.antidote."1.0.0".default or true);
  }) [];


# end
# autocfg-0.1.1

  crates.autocfg."0.1.1" = deps: { features?(features_.autocfg."0.1.1" deps {}) }: buildRustCrate {
    crateName = "autocfg";
    version = "0.1.1";
    description = "Automatic cfg for Rust compiler features";
    authors = [ "Josh Stone <cuviper@gmail.com>" ];
    sha256 = "0pzhbmwg46y04n89vn8yi0z1q8m3yh9gmfi8b9wn72zwk60f1rx2";
  };
  features_.autocfg."0.1.1" = deps: f: updateFeatures f (rec {
    autocfg."0.1.1".default = (f.autocfg."0.1.1".default or true);
  }) [];


# end
# backtrace-0.3.13

  crates.backtrace."0.3.13" = deps: { features?(features_.backtrace."0.3.13" deps {}) }: buildRustCrate {
    crateName = "backtrace";
    version = "0.3.13";
    description = "A library to acquire a stack trace (backtrace) at runtime in a Rust program.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" "The Rust Project Developers" ];
    sha256 = "1xx0vjdih9zqj6vp8l69n0f213wmif5471prgpkp24jbzxndvb1v";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."backtrace"."0.3.13"."cfg_if"}" deps)
      (crates."rustc_demangle"."${deps."backtrace"."0.3.13"."rustc_demangle"}" deps)
    ])
      ++ (if (kernel == "linux" || kernel == "darwin") && !(kernel == "fuchsia") && !(kernel == "emscripten") && !(kernel == "darwin") && !(kernel == "ios") then mapFeatures features ([
    ]
      ++ (if features.backtrace."0.3.13".backtrace-sys or false then [ (crates.backtrace_sys."${deps."backtrace"."0.3.13".backtrace_sys}" deps) ] else [])) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."backtrace"."0.3.13"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."backtrace"."0.3.13"."winapi"}" deps)
    ]) else []);

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."backtrace"."0.3.13"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."backtrace"."0.3.13" or {});
  };
  features_.backtrace."0.3.13" = deps: f: updateFeatures f (rec {
    autocfg."${deps.backtrace."0.3.13".autocfg}".default = true;
    backtrace = fold recursiveUpdate {} [
      { "0.3.13"."addr2line" =
        (f.backtrace."0.3.13"."addr2line" or false) ||
        (f.backtrace."0.3.13".gimli-symbolize or false) ||
        (backtrace."0.3.13"."gimli-symbolize" or false); }
      { "0.3.13"."backtrace-sys" =
        (f.backtrace."0.3.13"."backtrace-sys" or false) ||
        (f.backtrace."0.3.13".libbacktrace or false) ||
        (backtrace."0.3.13"."libbacktrace" or false); }
      { "0.3.13"."coresymbolication" =
        (f.backtrace."0.3.13"."coresymbolication" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false); }
      { "0.3.13"."dbghelp" =
        (f.backtrace."0.3.13"."dbghelp" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false); }
      { "0.3.13"."dladdr" =
        (f.backtrace."0.3.13"."dladdr" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false); }
      { "0.3.13"."findshlibs" =
        (f.backtrace."0.3.13"."findshlibs" or false) ||
        (f.backtrace."0.3.13".gimli-symbolize or false) ||
        (backtrace."0.3.13"."gimli-symbolize" or false); }
      { "0.3.13"."gimli" =
        (f.backtrace."0.3.13"."gimli" or false) ||
        (f.backtrace."0.3.13".gimli-symbolize or false) ||
        (backtrace."0.3.13"."gimli-symbolize" or false); }
      { "0.3.13"."libbacktrace" =
        (f.backtrace."0.3.13"."libbacktrace" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false); }
      { "0.3.13"."libunwind" =
        (f.backtrace."0.3.13"."libunwind" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false); }
      { "0.3.13"."memmap" =
        (f.backtrace."0.3.13"."memmap" or false) ||
        (f.backtrace."0.3.13".gimli-symbolize or false) ||
        (backtrace."0.3.13"."gimli-symbolize" or false); }
      { "0.3.13"."object" =
        (f.backtrace."0.3.13"."object" or false) ||
        (f.backtrace."0.3.13".gimli-symbolize or false) ||
        (backtrace."0.3.13"."gimli-symbolize" or false); }
      { "0.3.13"."rustc-serialize" =
        (f.backtrace."0.3.13"."rustc-serialize" or false) ||
        (f.backtrace."0.3.13".serialize-rustc or false) ||
        (backtrace."0.3.13"."serialize-rustc" or false); }
      { "0.3.13"."serde" =
        (f.backtrace."0.3.13"."serde" or false) ||
        (f.backtrace."0.3.13".serialize-serde or false) ||
        (backtrace."0.3.13"."serialize-serde" or false); }
      { "0.3.13"."serde_derive" =
        (f.backtrace."0.3.13"."serde_derive" or false) ||
        (f.backtrace."0.3.13".serialize-serde or false) ||
        (backtrace."0.3.13"."serialize-serde" or false); }
      { "0.3.13"."std" =
        (f.backtrace."0.3.13"."std" or false) ||
        (f.backtrace."0.3.13".default or false) ||
        (backtrace."0.3.13"."default" or false) ||
        (f.backtrace."0.3.13".libbacktrace or false) ||
        (backtrace."0.3.13"."libbacktrace" or false); }
      { "0.3.13".default = (f.backtrace."0.3.13".default or true); }
    ];
    backtrace_sys."${deps.backtrace."0.3.13".backtrace_sys}".default = true;
    cfg_if."${deps.backtrace."0.3.13".cfg_if}".default = true;
    libc."${deps.backtrace."0.3.13".libc}".default = (f.libc."${deps.backtrace."0.3.13".libc}".default or false);
    rustc_demangle."${deps.backtrace."0.3.13".rustc_demangle}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.backtrace."0.3.13".winapi}"."dbghelp" = true; }
      { "${deps.backtrace."0.3.13".winapi}"."minwindef" = true; }
      { "${deps.backtrace."0.3.13".winapi}"."processthreadsapi" = true; }
      { "${deps.backtrace."0.3.13".winapi}"."winnt" = true; }
      { "${deps.backtrace."0.3.13".winapi}".default = true; }
    ];
  }) [
    (features_.cfg_if."${deps."backtrace"."0.3.13"."cfg_if"}" deps)
    (features_.rustc_demangle."${deps."backtrace"."0.3.13"."rustc_demangle"}" deps)
    (features_.autocfg."${deps."backtrace"."0.3.13"."autocfg"}" deps)
    (features_.backtrace_sys."${deps."backtrace"."0.3.13"."backtrace_sys"}" deps)
    (features_.libc."${deps."backtrace"."0.3.13"."libc"}" deps)
    (features_.winapi."${deps."backtrace"."0.3.13"."winapi"}" deps)
  ];


# end
# backtrace-sys-0.1.28

  crates.backtrace_sys."0.1.28" = deps: { features?(features_.backtrace_sys."0.1.28" deps {}) }: buildRustCrate {
    crateName = "backtrace-sys";
    version = "0.1.28";
    description = "Bindings to the libbacktrace gcc library\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "1bbw8chs0wskxwzz7f3yy7mjqhyqj8lslq8pcjw1rbd2g23c34xl";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."backtrace_sys"."0.1.28"."libc"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."cc"."${deps."backtrace_sys"."0.1.28"."cc"}" deps)
    ]);
  };
  features_.backtrace_sys."0.1.28" = deps: f: updateFeatures f (rec {
    backtrace_sys."0.1.28".default = (f.backtrace_sys."0.1.28".default or true);
    cc."${deps.backtrace_sys."0.1.28".cc}".default = true;
    libc."${deps.backtrace_sys."0.1.28".libc}".default = (f.libc."${deps.backtrace_sys."0.1.28".libc}".default or false);
  }) [
    (features_.libc."${deps."backtrace_sys"."0.1.28"."libc"}" deps)
    (features_.cc."${deps."backtrace_sys"."0.1.28"."cc"}" deps)
  ];


# end
# base64-0.9.3

  crates.base64."0.9.3" = deps: { features?(features_.base64."0.9.3" deps {}) }: buildRustCrate {
    crateName = "base64";
    version = "0.9.3";
    description = "encodes and decodes base64 as bytes or utf8";
    authors = [ "Alice Maz <alice@alicemaz.com>" "Marshall Pierce <marshall@mpierce.org>" ];
    sha256 = "11hhz8ln4zbpn2h2gm9fbbb9j254wrd4fpmddlyah2rrnqsmmqkd";
    dependencies = mapFeatures features ([
      (crates."byteorder"."${deps."base64"."0.9.3"."byteorder"}" deps)
      (crates."safemem"."${deps."base64"."0.9.3"."safemem"}" deps)
    ]);
  };
  features_.base64."0.9.3" = deps: f: updateFeatures f (rec {
    base64."0.9.3".default = (f.base64."0.9.3".default or true);
    byteorder."${deps.base64."0.9.3".byteorder}".default = true;
    safemem."${deps.base64."0.9.3".safemem}".default = true;
  }) [
    (features_.byteorder."${deps."base64"."0.9.3"."byteorder"}" deps)
    (features_.safemem."${deps."base64"."0.9.3"."safemem"}" deps)
  ];


# end
# base64-0.10.0

  crates.base64."0.10.0" = deps: { features?(features_.base64."0.10.0" deps {}) }: buildRustCrate {
    crateName = "base64";
    version = "0.10.0";
    description = "encodes and decodes base64 as bytes or utf8";
    authors = [ "Alice Maz <alice@alicemaz.com>" "Marshall Pierce <marshall@mpierce.org>" ];
    sha256 = "1h9pfgvdl40d1l5hlrb5fg8rqkl86hz07i22vgdcpjbissw8sisj";
    dependencies = mapFeatures features ([
      (crates."byteorder"."${deps."base64"."0.10.0"."byteorder"}" deps)
    ]);
  };
  features_.base64."0.10.0" = deps: f: updateFeatures f (rec {
    base64."0.10.0".default = (f.base64."0.10.0".default or true);
    byteorder."${deps.base64."0.10.0".byteorder}".default = true;
  }) [
    (features_.byteorder."${deps."base64"."0.10.0"."byteorder"}" deps)
  ];


# end
# bit-vec-0.4.4

  crates.bit_vec."0.4.4" = deps: { features?(features_.bit_vec."0.4.4" deps {}) }: buildRustCrate {
    crateName = "bit-vec";
    version = "0.4.4";
    description = "A vector of bits";
    authors = [ "Alexis Beingessner <a.beingessner@gmail.com>" ];
    sha256 = "06czykmn001z6c3a4nsrpc3lrj63ga0kzp7kgva9r9wylhkkqpq9";
    features = mkFeatures (features."bit_vec"."0.4.4" or {});
  };
  features_.bit_vec."0.4.4" = deps: f: updateFeatures f (rec {
    bit_vec."0.4.4".default = (f.bit_vec."0.4.4".default or true);
  }) [];


# end
# bitflags-0.9.1

  crates.bitflags."0.9.1" = deps: { features?(features_.bitflags."0.9.1" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "0.9.1";
    description = "A macro to generate structures which behave like bitflags.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "18h073l5jd88rx4qdr95fjddr9rk79pb1aqnshzdnw16cfmb9rws";
    features = mkFeatures (features."bitflags"."0.9.1" or {});
  };
  features_.bitflags."0.9.1" = deps: f: updateFeatures f (rec {
    bitflags = fold recursiveUpdate {} [
      { "0.9.1"."example_generated" =
        (f.bitflags."0.9.1"."example_generated" or false) ||
        (f.bitflags."0.9.1".default or false) ||
        (bitflags."0.9.1"."default" or false); }
      { "0.9.1".default = (f.bitflags."0.9.1".default or true); }
    ];
  }) [];


# end
# bitflags-1.0.4

  crates.bitflags."1.0.4" = deps: { features?(features_.bitflags."1.0.4" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "1.0.4";
    description = "A macro to generate structures which behave like bitflags.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1g1wmz2001qmfrd37dnd5qiss5njrw26aywmg6yhkmkbyrhjxb08";
    features = mkFeatures (features."bitflags"."1.0.4" or {});
  };
  features_.bitflags."1.0.4" = deps: f: updateFeatures f (rec {
    bitflags."1.0.4".default = (f.bitflags."1.0.4".default or true);
  }) [];


# end
# byteorder-0.5.3

  crates.byteorder."0.5.3" = deps: { features?(features_.byteorder."0.5.3" deps {}) }: buildRustCrate {
    crateName = "byteorder";
    version = "0.5.3";
    description = "Library for reading/writing numbers in big-endian and little-endian.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "0zsr6b0m0yl5c0yy92nq7srfpczd1dx1xqcx3rlm5fbl8si9clqx";
    features = mkFeatures (features."byteorder"."0.5.3" or {});
  };
  features_.byteorder."0.5.3" = deps: f: updateFeatures f (rec {
    byteorder = fold recursiveUpdate {} [
      { "0.5.3"."std" =
        (f.byteorder."0.5.3"."std" or false) ||
        (f.byteorder."0.5.3".default or false) ||
        (byteorder."0.5.3"."default" or false); }
      { "0.5.3".default = (f.byteorder."0.5.3".default or true); }
    ];
  }) [];


# end
# byteorder-1.2.7

  crates.byteorder."1.2.7" = deps: { features?(features_.byteorder."1.2.7" deps {}) }: buildRustCrate {
    crateName = "byteorder";
    version = "1.2.7";
    description = "Library for reading/writing numbers in big-endian and little-endian.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "0wsl8in6jk2v1n8s8jz0pjd99mjr2isbf981497pgavwg6i11q5h";
    features = mkFeatures (features."byteorder"."1.2.7" or {});
  };
  features_.byteorder."1.2.7" = deps: f: updateFeatures f (rec {
    byteorder = fold recursiveUpdate {} [
      { "1.2.7"."std" =
        (f.byteorder."1.2.7"."std" or false) ||
        (f.byteorder."1.2.7".default or false) ||
        (byteorder."1.2.7"."default" or false); }
      { "1.2.7".default = (f.byteorder."1.2.7".default or true); }
    ];
  }) [];


# end
# cc-1.0.28

  crates.cc."1.0.28" = deps: { features?(features_.cc."1.0.28" deps {}) }: buildRustCrate {
    crateName = "cc";
    version = "1.0.28";
    description = "A build-time dependency for Cargo build scripts to assist in invoking the native\nC compiler to compile native C code into a static archive to be linked into Rust\ncode.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "07harxg2cjw75qvnq637z088w9qaa0hgj0nmcm6yh9in8m2swl19";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."cc"."1.0.28" or {});
  };
  features_.cc."1.0.28" = deps: f: updateFeatures f (rec {
    cc = fold recursiveUpdate {} [
      { "1.0.28"."rayon" =
        (f.cc."1.0.28"."rayon" or false) ||
        (f.cc."1.0.28".parallel or false) ||
        (cc."1.0.28"."parallel" or false); }
      { "1.0.28".default = (f.cc."1.0.28".default or true); }
    ];
  }) [];


# end
# cfg-if-0.1.6

  crates.cfg_if."0.1.6" = deps: { features?(features_.cfg_if."0.1.6" deps {}) }: buildRustCrate {
    crateName = "cfg-if";
    version = "0.1.6";
    description = "A macro to ergonomically define an item depending on a large number of #[cfg]\nparameters. Structured like an if-else chain, the first matching branch is the\nitem that gets emitted.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "11qrix06wagkplyk908i3423ps9m9np6c4vbcq81s9fyl244xv3n";
  };
  features_.cfg_if."0.1.6" = deps: f: updateFeatures f (rec {
    cfg_if."0.1.6".default = (f.cfg_if."0.1.6".default or true);
  }) [];


# end
# chrono-0.4.6

  crates.chrono."0.4.6" = deps: { features?(features_.chrono."0.4.6" deps {}) }: buildRustCrate {
    crateName = "chrono";
    version = "0.4.6";
    description = "Date and time library for Rust";
    authors = [ "Kang Seonghoon <public+rust@mearie.org>" "Brandon W Maister <quodlibetor@gmail.com>" ];
    sha256 = "0cxgqgf4lknsii1k806dpmzapi2zccjpa350ns5wpb568mij096x";
    dependencies = mapFeatures features ([
      (crates."num_integer"."${deps."chrono"."0.4.6"."num_integer"}" deps)
      (crates."num_traits"."${deps."chrono"."0.4.6"."num_traits"}" deps)
    ]
      ++ (if features.chrono."0.4.6".time or false then [ (crates.time."${deps."chrono"."0.4.6".time}" deps) ] else []));
    features = mkFeatures (features."chrono"."0.4.6" or {});
  };
  features_.chrono."0.4.6" = deps: f: updateFeatures f (rec {
    chrono = fold recursiveUpdate {} [
      { "0.4.6"."clock" =
        (f.chrono."0.4.6"."clock" or false) ||
        (f.chrono."0.4.6".default or false) ||
        (chrono."0.4.6"."default" or false); }
      { "0.4.6"."time" =
        (f.chrono."0.4.6"."time" or false) ||
        (f.chrono."0.4.6".clock or false) ||
        (chrono."0.4.6"."clock" or false); }
      { "0.4.6".default = (f.chrono."0.4.6".default or true); }
    ];
    num_integer."${deps.chrono."0.4.6".num_integer}".default = (f.num_integer."${deps.chrono."0.4.6".num_integer}".default or false);
    num_traits."${deps.chrono."0.4.6".num_traits}".default = (f.num_traits."${deps.chrono."0.4.6".num_traits}".default or false);
    time."${deps.chrono."0.4.6".time}".default = true;
  }) [
    (features_.num_integer."${deps."chrono"."0.4.6"."num_integer"}" deps)
    (features_.num_traits."${deps."chrono"."0.4.6"."num_traits"}" deps)
    (features_.time."${deps."chrono"."0.4.6"."time"}" deps)
  ];


# end
# core-foundation-0.2.3

  crates.core_foundation."0.2.3" = deps: { features?(features_.core_foundation."0.2.3" deps {}) }: buildRustCrate {
    crateName = "core-foundation";
    version = "0.2.3";
    description = "Bindings to Core Foundation for OS X";
    authors = [ "The Servo Project Developers" ];
    sha256 = "1g0vpya5h2wa0nlz4a74jar6y8z09f0p76zbzfqrm3dbfsrld1pm";
    dependencies = mapFeatures features ([
      (crates."core_foundation_sys"."${deps."core_foundation"."0.2.3"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."core_foundation"."0.2.3"."libc"}" deps)
    ]);
  };
  features_.core_foundation."0.2.3" = deps: f: updateFeatures f (rec {
    core_foundation."0.2.3".default = (f.core_foundation."0.2.3".default or true);
    core_foundation_sys."${deps.core_foundation."0.2.3".core_foundation_sys}".default = true;
    libc."${deps.core_foundation."0.2.3".libc}".default = true;
  }) [
    (features_.core_foundation_sys."${deps."core_foundation"."0.2.3"."core_foundation_sys"}" deps)
    (features_.libc."${deps."core_foundation"."0.2.3"."libc"}" deps)
  ];


# end
# core-foundation-sys-0.2.3

  crates.core_foundation_sys."0.2.3" = deps: { features?(features_.core_foundation_sys."0.2.3" deps {}) }: buildRustCrate {
    crateName = "core-foundation-sys";
    version = "0.2.3";
    description = "Bindings to Core Foundation for OS X";
    authors = [ "The Servo Project Developers" ];
    sha256 = "19s0d03294m9s5j8cvy345db3gkhs2y02j5268ap0c6ky5apl53s";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."core_foundation_sys"."0.2.3"."libc"}" deps)
    ]);
  };
  features_.core_foundation_sys."0.2.3" = deps: f: updateFeatures f (rec {
    core_foundation_sys."0.2.3".default = (f.core_foundation_sys."0.2.3".default or true);
    libc."${deps.core_foundation_sys."0.2.3".libc}".default = true;
  }) [
    (features_.libc."${deps."core_foundation_sys"."0.2.3"."libc"}" deps)
  ];


# end
# either-1.5.0

  crates.either."1.5.0" = deps: { features?(features_.either."1.5.0" deps {}) }: buildRustCrate {
    crateName = "either";
    version = "1.5.0";
    description = "The enum `Either` with variants `Left` and `Right` is a general purpose sum type with two cases.\n";
    authors = [ "bluss" ];
    sha256 = "1f7kl2ln01y02m8fpd2zrdjiwqmgfvl9nxxrfry3k19d1gd2bsvz";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."either"."1.5.0" or {});
  };
  features_.either."1.5.0" = deps: f: updateFeatures f (rec {
    either = fold recursiveUpdate {} [
      { "1.5.0"."use_std" =
        (f.either."1.5.0"."use_std" or false) ||
        (f.either."1.5.0".default or false) ||
        (either."1.5.0"."default" or false); }
      { "1.5.0".default = (f.either."1.5.0".default or true); }
    ];
  }) [];


# end
# enum_primitive-0.1.1

  crates.enum_primitive."0.1.1" = deps: { features?(features_.enum_primitive."0.1.1" deps {}) }: buildRustCrate {
    crateName = "enum_primitive";
    version = "0.1.1";
    description = "Macro to generate num::FromPrimitive instances for enum that works in Rust 1.0";
    authors = [ "Anders Kaseorg <andersk@mit.edu>" ];
    sha256 = "1a225rlsz7sz3nn14dar71kp2f9v08s3rwl6j55xp51mv01f695y";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."enum_primitive"."0.1.1"."num_traits"}" deps)
    ]);
  };
  features_.enum_primitive."0.1.1" = deps: f: updateFeatures f (rec {
    enum_primitive."0.1.1".default = (f.enum_primitive."0.1.1".default or true);
    num_traits."${deps.enum_primitive."0.1.1".num_traits}".default = (f.num_traits."${deps.enum_primitive."0.1.1".num_traits}".default or false);
  }) [
    (features_.num_traits."${deps."enum_primitive"."0.1.1"."num_traits"}" deps)
  ];


# end
# env_logger-0.3.5

  crates.env_logger."0.3.5" = deps: { features?(features_.env_logger."0.3.5" deps {}) }: buildRustCrate {
    crateName = "env_logger";
    version = "0.3.5";
    description = "An logging implementation for `log` which is configured via an environment\nvariable.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1mvxiaaqsyjliv1mm1qaagjqiccw11mdyi3n9h9rf8y6wj15zycw";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."env_logger"."0.3.5"."log"}" deps)
    ]
      ++ (if features.env_logger."0.3.5".regex or false then [ (crates.regex."${deps."env_logger"."0.3.5".regex}" deps) ] else []));
    features = mkFeatures (features."env_logger"."0.3.5" or {});
  };
  features_.env_logger."0.3.5" = deps: f: updateFeatures f (rec {
    env_logger = fold recursiveUpdate {} [
      { "0.3.5"."regex" =
        (f.env_logger."0.3.5"."regex" or false) ||
        (f.env_logger."0.3.5".default or false) ||
        (env_logger."0.3.5"."default" or false); }
      { "0.3.5".default = (f.env_logger."0.3.5".default or true); }
    ];
    log."${deps.env_logger."0.3.5".log}".default = true;
    regex."${deps.env_logger."0.3.5".regex}".default = true;
  }) [
    (features_.log."${deps."env_logger"."0.3.5"."log"}" deps)
    (features_.regex."${deps."env_logger"."0.3.5"."regex"}" deps)
  ];


# end
# env_logger-0.4.3

  crates.env_logger."0.4.3" = deps: { features?(features_.env_logger."0.4.3" deps {}) }: buildRustCrate {
    crateName = "env_logger";
    version = "0.4.3";
    description = "A logging implementation for `log` which is configured via an environment\nvariable.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0nrx04p4xa86d5kc7aq4fwvipbqji9cmgy449h47nc9f1chafhgg";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."env_logger"."0.4.3"."log"}" deps)
    ]
      ++ (if features.env_logger."0.4.3".regex or false then [ (crates.regex."${deps."env_logger"."0.4.3".regex}" deps) ] else []));
    features = mkFeatures (features."env_logger"."0.4.3" or {});
  };
  features_.env_logger."0.4.3" = deps: f: updateFeatures f (rec {
    env_logger = fold recursiveUpdate {} [
      { "0.4.3"."regex" =
        (f.env_logger."0.4.3"."regex" or false) ||
        (f.env_logger."0.4.3".default or false) ||
        (env_logger."0.4.3"."default" or false); }
      { "0.4.3".default = (f.env_logger."0.4.3".default or true); }
    ];
    log."${deps.env_logger."0.4.3".log}".default = true;
    regex."${deps.env_logger."0.4.3".regex}".default = true;
  }) [
    (features_.log."${deps."env_logger"."0.4.3"."log"}" deps)
    (features_.regex."${deps."env_logger"."0.4.3"."regex"}" deps)
  ];


# end
# error-chain-0.10.0

  crates.error_chain."0.10.0" = deps: { features?(features_.error_chain."0.10.0" deps {}) }: buildRustCrate {
    crateName = "error-chain";
    version = "0.10.0";
    description = "Yet another error boilerplate library.";
    authors = [ "Brian Anderson <banderson@mozilla.com>" "Paul Colomiets <paul@colomiets.name>" "Colin Kiegel <kiegel@gmx.de>" "Yamakaky <yamakaky@yamaworld.fr>" ];
    sha256 = "1xxbzd8cjlpzsb9fsih7mdnndhzrvykj0w77yg90qc85az1xwy5z";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.error_chain."0.10.0".backtrace or false then [ (crates.backtrace."${deps."error_chain"."0.10.0".backtrace}" deps) ] else []));
    features = mkFeatures (features."error_chain"."0.10.0" or {});
  };
  features_.error_chain."0.10.0" = deps: f: updateFeatures f (rec {
    backtrace."${deps.error_chain."0.10.0".backtrace}".default = true;
    error_chain = fold recursiveUpdate {} [
      { "0.10.0"."backtrace" =
        (f.error_chain."0.10.0"."backtrace" or false) ||
        (f.error_chain."0.10.0".default or false) ||
        (error_chain."0.10.0"."default" or false); }
      { "0.10.0"."example_generated" =
        (f.error_chain."0.10.0"."example_generated" or false) ||
        (f.error_chain."0.10.0".default or false) ||
        (error_chain."0.10.0"."default" or false); }
      { "0.10.0".default = (f.error_chain."0.10.0".default or true); }
    ];
  }) [
    (features_.backtrace."${deps."error_chain"."0.10.0"."backtrace"}" deps)
  ];


# end
# foreign-types-0.3.2

  crates.foreign_types."0.3.2" = deps: { features?(features_.foreign_types."0.3.2" deps {}) }: buildRustCrate {
    crateName = "foreign-types";
    version = "0.3.2";
    description = "A framework for Rust wrappers over C APIs";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "105n8sp2djb1s5lzrw04p7ss3dchr5qa3canmynx396nh3vwm2p8";
    dependencies = mapFeatures features ([
      (crates."foreign_types_shared"."${deps."foreign_types"."0.3.2"."foreign_types_shared"}" deps)
    ]);
  };
  features_.foreign_types."0.3.2" = deps: f: updateFeatures f (rec {
    foreign_types."0.3.2".default = (f.foreign_types."0.3.2".default or true);
    foreign_types_shared."${deps.foreign_types."0.3.2".foreign_types_shared}".default = true;
  }) [
    (features_.foreign_types_shared."${deps."foreign_types"."0.3.2"."foreign_types_shared"}" deps)
  ];


# end
# foreign-types-shared-0.1.1

  crates.foreign_types_shared."0.1.1" = deps: { features?(features_.foreign_types_shared."0.1.1" deps {}) }: buildRustCrate {
    crateName = "foreign-types-shared";
    version = "0.1.1";
    description = "An internal crate used by foreign-types";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "0b6cnvqbflws8dxywk4589vgbz80049lz4x1g9dfy4s1ppd3g4z5";
  };
  features_.foreign_types_shared."0.1.1" = deps: f: updateFeatures f (rec {
    foreign_types_shared."0.1.1".default = (f.foreign_types_shared."0.1.1".default or true);
  }) [];


# end
# frank_jwt-3.1.0

  crates.frank_jwt."3.1.0" = deps: { features?(features_.frank_jwt."3.1.0" deps {}) }: buildRustCrate {
    crateName = "frank_jwt";
    version = "3.1.0";
    description = "Implementation of JSON JWT";
    authors = [ "Alex Maslakov <me@gildedhonour.com>, <abc@nothingness.xyz>" ];
    sha256 = "1kqi6wcsk96chf1p4qfnbkmvpdb91wi0v53k5mp7wvdmaqwgj6zl";
    dependencies = mapFeatures features ([
      (crates."base64"."${deps."frank_jwt"."3.1.0"."base64"}" deps)
      (crates."openssl"."${deps."frank_jwt"."3.1.0"."openssl"}" deps)
      (crates."serde"."${deps."frank_jwt"."3.1.0"."serde"}" deps)
      (crates."serde_json"."${deps."frank_jwt"."3.1.0"."serde_json"}" deps)
    ]);
  };
  features_.frank_jwt."3.1.0" = deps: f: updateFeatures f (rec {
    base64."${deps.frank_jwt."3.1.0".base64}".default = true;
    frank_jwt."3.1.0".default = (f.frank_jwt."3.1.0".default or true);
    openssl."${deps.frank_jwt."3.1.0".openssl}".default = true;
    serde."${deps.frank_jwt."3.1.0".serde}".default = true;
    serde_json."${deps.frank_jwt."3.1.0".serde_json}".default = true;
  }) [
    (features_.base64."${deps."frank_jwt"."3.1.0"."base64"}" deps)
    (features_.openssl."${deps."frank_jwt"."3.1.0"."openssl"}" deps)
    (features_.serde."${deps."frank_jwt"."3.1.0"."serde"}" deps)
    (features_.serde_json."${deps."frank_jwt"."3.1.0"."serde_json"}" deps)
  ];


# end
# fs2-0.4.3

  crates.fs2."0.4.3" = deps: { features?(features_.fs2."0.4.3" deps {}) }: buildRustCrate {
    crateName = "fs2";
    version = "0.4.3";
    description = "Cross-platform file locks and file duplication.";
    authors = [ "Dan Burkert <dan@danburkert.com>" ];
    sha256 = "1crj36rhhpk3qby9yj7r77w7sld0mzab2yicmphbdkfymbmp3ldp";
    dependencies = (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."fs2"."0.4.3"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."fs2"."0.4.3"."winapi"}" deps)
    ]) else []);
  };
  features_.fs2."0.4.3" = deps: f: updateFeatures f (rec {
    fs2."0.4.3".default = (f.fs2."0.4.3".default or true);
    libc."${deps.fs2."0.4.3".libc}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.fs2."0.4.3".winapi}"."fileapi" = true; }
      { "${deps.fs2."0.4.3".winapi}"."handleapi" = true; }
      { "${deps.fs2."0.4.3".winapi}"."processthreadsapi" = true; }
      { "${deps.fs2."0.4.3".winapi}"."std" = true; }
      { "${deps.fs2."0.4.3".winapi}"."winbase" = true; }
      { "${deps.fs2."0.4.3".winapi}"."winerror" = true; }
      { "${deps.fs2."0.4.3".winapi}".default = true; }
    ];
  }) [
    (features_.libc."${deps."fs2"."0.4.3"."libc"}" deps)
    (features_.winapi."${deps."fs2"."0.4.3"."winapi"}" deps)
  ];


# end
# fuchsia-zircon-0.3.3

  crates.fuchsia_zircon."0.3.3" = deps: { features?(features_.fuchsia_zircon."0.3.3" deps {}) }: buildRustCrate {
    crateName = "fuchsia-zircon";
    version = "0.3.3";
    description = "Rust bindings for the Zircon kernel";
    authors = [ "Raph Levien <raph@google.com>" ];
    sha256 = "0jrf4shb1699r4la8z358vri8318w4mdi6qzfqy30p2ymjlca4gk";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."fuchsia_zircon"."0.3.3"."bitflags"}" deps)
      (crates."fuchsia_zircon_sys"."${deps."fuchsia_zircon"."0.3.3"."fuchsia_zircon_sys"}" deps)
    ]);
  };
  features_.fuchsia_zircon."0.3.3" = deps: f: updateFeatures f (rec {
    bitflags."${deps.fuchsia_zircon."0.3.3".bitflags}".default = true;
    fuchsia_zircon."0.3.3".default = (f.fuchsia_zircon."0.3.3".default or true);
    fuchsia_zircon_sys."${deps.fuchsia_zircon."0.3.3".fuchsia_zircon_sys}".default = true;
  }) [
    (features_.bitflags."${deps."fuchsia_zircon"."0.3.3"."bitflags"}" deps)
    (features_.fuchsia_zircon_sys."${deps."fuchsia_zircon"."0.3.3"."fuchsia_zircon_sys"}" deps)
  ];


# end
# fuchsia-zircon-sys-0.3.3

  crates.fuchsia_zircon_sys."0.3.3" = deps: { features?(features_.fuchsia_zircon_sys."0.3.3" deps {}) }: buildRustCrate {
    crateName = "fuchsia-zircon-sys";
    version = "0.3.3";
    description = "Low-level Rust bindings for the Zircon kernel";
    authors = [ "Raph Levien <raph@google.com>" ];
    sha256 = "08jp1zxrm9jbrr6l26bjal4dbm8bxfy57ickdgibsqxr1n9j3hf5";
  };
  features_.fuchsia_zircon_sys."0.3.3" = deps: f: updateFeatures f (rec {
    fuchsia_zircon_sys."0.3.3".default = (f.fuchsia_zircon_sys."0.3.3".default or true);
  }) [];


# end
# httparse-1.3.3

  crates.httparse."1.3.3" = deps: { features?(features_.httparse."1.3.3" deps {}) }: buildRustCrate {
    crateName = "httparse";
    version = "1.3.3";
    description = "A tiny, safe, speedy, zero-copy HTTP/1.x parser.";
    authors = [ "Sean McArthur <sean@seanmonstar.com>" ];
    sha256 = "1jymxy4bl0mzgp2dx0pzqzbr72sw5jmr5sjqiry4xr88z4z9qlyx";
    build = "build.rs";
    features = mkFeatures (features."httparse"."1.3.3" or {});
  };
  features_.httparse."1.3.3" = deps: f: updateFeatures f (rec {
    httparse = fold recursiveUpdate {} [
      { "1.3.3"."std" =
        (f.httparse."1.3.3"."std" or false) ||
        (f.httparse."1.3.3".default or false) ||
        (httparse."1.3.3"."default" or false); }
      { "1.3.3".default = (f.httparse."1.3.3".default or true); }
    ];
  }) [];


# end
# hyper-0.10.15

  crates.hyper."0.10.15" = deps: { features?(features_.hyper."0.10.15" deps {}) }: buildRustCrate {
    crateName = "hyper";
    version = "0.10.15";
    description = "A modern HTTP library.";
    authors = [ "Sean McArthur <sean.monstar@gmail.com>" "Jonathan Reem <jonathan.reem@gmail.com>" ];
    sha256 = "14bf31dwwfvza3kfc4mmk4q0v7iq5ys3hiz7islij1x9g4c53s9p";
    dependencies = mapFeatures features ([
      (crates."base64"."${deps."hyper"."0.10.15"."base64"}" deps)
      (crates."httparse"."${deps."hyper"."0.10.15"."httparse"}" deps)
      (crates."language_tags"."${deps."hyper"."0.10.15"."language_tags"}" deps)
      (crates."log"."${deps."hyper"."0.10.15"."log"}" deps)
      (crates."mime"."${deps."hyper"."0.10.15"."mime"}" deps)
      (crates."num_cpus"."${deps."hyper"."0.10.15"."num_cpus"}" deps)
      (crates."time"."${deps."hyper"."0.10.15"."time"}" deps)
      (crates."traitobject"."${deps."hyper"."0.10.15"."traitobject"}" deps)
      (crates."typeable"."${deps."hyper"."0.10.15"."typeable"}" deps)
      (crates."unicase"."${deps."hyper"."0.10.15"."unicase"}" deps)
      (crates."url"."${deps."hyper"."0.10.15"."url"}" deps)
    ]);
    features = mkFeatures (features."hyper"."0.10.15" or {});
  };
  features_.hyper."0.10.15" = deps: f: updateFeatures f (rec {
    base64."${deps.hyper."0.10.15".base64}".default = true;
    httparse."${deps.hyper."0.10.15".httparse}".default = true;
    hyper."0.10.15".default = (f.hyper."0.10.15".default or true);
    language_tags."${deps.hyper."0.10.15".language_tags}".default = true;
    log."${deps.hyper."0.10.15".log}".default = true;
    mime."${deps.hyper."0.10.15".mime}".default = true;
    num_cpus."${deps.hyper."0.10.15".num_cpus}".default = true;
    time."${deps.hyper."0.10.15".time}".default = true;
    traitobject."${deps.hyper."0.10.15".traitobject}".default = true;
    typeable."${deps.hyper."0.10.15".typeable}".default = true;
    unicase."${deps.hyper."0.10.15".unicase}".default = true;
    url."${deps.hyper."0.10.15".url}".default = true;
  }) [
    (features_.base64."${deps."hyper"."0.10.15"."base64"}" deps)
    (features_.httparse."${deps."hyper"."0.10.15"."httparse"}" deps)
    (features_.language_tags."${deps."hyper"."0.10.15"."language_tags"}" deps)
    (features_.log."${deps."hyper"."0.10.15"."log"}" deps)
    (features_.mime."${deps."hyper"."0.10.15"."mime"}" deps)
    (features_.num_cpus."${deps."hyper"."0.10.15"."num_cpus"}" deps)
    (features_.time."${deps."hyper"."0.10.15"."time"}" deps)
    (features_.traitobject."${deps."hyper"."0.10.15"."traitobject"}" deps)
    (features_.typeable."${deps."hyper"."0.10.15"."typeable"}" deps)
    (features_.unicase."${deps."hyper"."0.10.15"."unicase"}" deps)
    (features_.url."${deps."hyper"."0.10.15"."url"}" deps)
  ];


# end
# hyper-native-tls-0.2.4

  crates.hyper_native_tls."0.2.4" = deps: { features?(features_.hyper_native_tls."0.2.4" deps {}) }: buildRustCrate {
    crateName = "hyper-native-tls";
    version = "0.2.4";
    description = "native-tls support for Hyper";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "1niqi1z1a3xfb9qaawy3fzrgaf8qwr925fqjswlrdjczq176f1iy";
    dependencies = mapFeatures features ([
      (crates."antidote"."${deps."hyper_native_tls"."0.2.4"."antidote"}" deps)
      (crates."hyper"."${deps."hyper_native_tls"."0.2.4"."hyper"}" deps)
      (crates."native_tls"."${deps."hyper_native_tls"."0.2.4"."native_tls"}" deps)
    ]);
  };
  features_.hyper_native_tls."0.2.4" = deps: f: updateFeatures f (rec {
    antidote."${deps.hyper_native_tls."0.2.4".antidote}".default = true;
    hyper."${deps.hyper_native_tls."0.2.4".hyper}".default = true;
    hyper_native_tls."0.2.4".default = (f.hyper_native_tls."0.2.4".default or true);
    native_tls."${deps.hyper_native_tls."0.2.4".native_tls}".default = true;
  }) [
    (features_.antidote."${deps."hyper_native_tls"."0.2.4"."antidote"}" deps)
    (features_.hyper."${deps."hyper_native_tls"."0.2.4"."hyper"}" deps)
    (features_.native_tls."${deps."hyper_native_tls"."0.2.4"."native_tls"}" deps)
  ];


# end
# idna-0.1.5

  crates.idna."0.1.5" = deps: { features?(features_.idna."0.1.5" deps {}) }: buildRustCrate {
    crateName = "idna";
    version = "0.1.5";
    description = "IDNA (Internationalizing Domain Names in Applications) and Punycode.";
    authors = [ "The rust-url developers" ];
    sha256 = "1gwgl19rz5vzi67rrhamczhxy050f5ynx4ybabfapyalv7z1qmjy";
    dependencies = mapFeatures features ([
      (crates."matches"."${deps."idna"."0.1.5"."matches"}" deps)
      (crates."unicode_bidi"."${deps."idna"."0.1.5"."unicode_bidi"}" deps)
      (crates."unicode_normalization"."${deps."idna"."0.1.5"."unicode_normalization"}" deps)
    ]);
  };
  features_.idna."0.1.5" = deps: f: updateFeatures f (rec {
    idna."0.1.5".default = (f.idna."0.1.5".default or true);
    matches."${deps.idna."0.1.5".matches}".default = true;
    unicode_bidi."${deps.idna."0.1.5".unicode_bidi}".default = true;
    unicode_normalization."${deps.idna."0.1.5".unicode_normalization}".default = true;
  }) [
    (features_.matches."${deps."idna"."0.1.5"."matches"}" deps)
    (features_.unicode_bidi."${deps."idna"."0.1.5"."unicode_bidi"}" deps)
    (features_.unicode_normalization."${deps."idna"."0.1.5"."unicode_normalization"}" deps)
  ];


# end
# itoa-0.4.3

  crates.itoa."0.4.3" = deps: { features?(features_.itoa."0.4.3" deps {}) }: buildRustCrate {
    crateName = "itoa";
    version = "0.4.3";
    description = "Fast functions for printing integer primitives to an io::Write";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0zadimmdgvili3gdwxqg7ljv3r4wcdg1kkdfp9nl15vnm23vrhy1";
    features = mkFeatures (features."itoa"."0.4.3" or {});
  };
  features_.itoa."0.4.3" = deps: f: updateFeatures f (rec {
    itoa = fold recursiveUpdate {} [
      { "0.4.3"."std" =
        (f.itoa."0.4.3"."std" or false) ||
        (f.itoa."0.4.3".default or false) ||
        (itoa."0.4.3"."default" or false); }
      { "0.4.3".default = (f.itoa."0.4.3".default or true); }
    ];
  }) [];


# end
# kernel32-sys-0.2.2

  crates.kernel32_sys."0.2.2" = deps: { features?(features_.kernel32_sys."0.2.2" deps {}) }: buildRustCrate {
    crateName = "kernel32-sys";
    version = "0.2.2";
    description = "Contains function definitions for the Windows API library kernel32. See winapi for types and constants.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1lrw1hbinyvr6cp28g60z97w32w8vsk6pahk64pmrv2fmby8srfj";
    libName = "kernel32";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."winapi"."${deps."kernel32_sys"."0.2.2"."winapi"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."winapi_build"."${deps."kernel32_sys"."0.2.2"."winapi_build"}" deps)
    ]);
  };
  features_.kernel32_sys."0.2.2" = deps: f: updateFeatures f (rec {
    kernel32_sys."0.2.2".default = (f.kernel32_sys."0.2.2".default or true);
    winapi."${deps.kernel32_sys."0.2.2".winapi}".default = true;
    winapi_build."${deps.kernel32_sys."0.2.2".winapi_build}".default = true;
  }) [
    (features_.winapi."${deps."kernel32_sys"."0.2.2"."winapi"}" deps)
    (features_.winapi_build."${deps."kernel32_sys"."0.2.2"."winapi_build"}" deps)
  ];


# end
# language-tags-0.2.2

  crates.language_tags."0.2.2" = deps: { features?(features_.language_tags."0.2.2" deps {}) }: buildRustCrate {
    crateName = "language-tags";
    version = "0.2.2";
    description = "Language tags for Rust";
    authors = [ "Pyfisch <pyfisch@gmail.com>" ];
    sha256 = "1zkrdzsqzzc7509kd7nngdwrp461glm2g09kqpzaqksp82frjdvy";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."language_tags"."0.2.2" or {});
  };
  features_.language_tags."0.2.2" = deps: f: updateFeatures f (rec {
    language_tags = fold recursiveUpdate {} [
      { "0.2.2"."heapsize" =
        (f.language_tags."0.2.2"."heapsize" or false) ||
        (f.language_tags."0.2.2".heap_size or false) ||
        (language_tags."0.2.2"."heap_size" or false); }
      { "0.2.2"."heapsize_plugin" =
        (f.language_tags."0.2.2"."heapsize_plugin" or false) ||
        (f.language_tags."0.2.2".heap_size or false) ||
        (language_tags."0.2.2"."heap_size" or false); }
      { "0.2.2".default = (f.language_tags."0.2.2".default or true); }
    ];
  }) [];


# end
# lazy_static-0.2.11

  crates.lazy_static."0.2.11" = deps: { features?(features_.lazy_static."0.2.11" deps {}) }: buildRustCrate {
    crateName = "lazy_static";
    version = "0.2.11";
    description = "A macro for declaring lazily evaluated statics in Rust.";
    authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
    sha256 = "1x6871cvpy5b96yv4c7jvpq316fp5d4609s9py7qk6cd6x9k34vm";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."lazy_static"."0.2.11" or {});
  };
  features_.lazy_static."0.2.11" = deps: f: updateFeatures f (rec {
    lazy_static = fold recursiveUpdate {} [
      { "0.2.11"."compiletest_rs" =
        (f.lazy_static."0.2.11"."compiletest_rs" or false) ||
        (f.lazy_static."0.2.11".compiletest or false) ||
        (lazy_static."0.2.11"."compiletest" or false); }
      { "0.2.11"."nightly" =
        (f.lazy_static."0.2.11"."nightly" or false) ||
        (f.lazy_static."0.2.11".spin_no_std or false) ||
        (lazy_static."0.2.11"."spin_no_std" or false); }
      { "0.2.11"."spin" =
        (f.lazy_static."0.2.11"."spin" or false) ||
        (f.lazy_static."0.2.11".spin_no_std or false) ||
        (lazy_static."0.2.11"."spin_no_std" or false); }
      { "0.2.11".default = (f.lazy_static."0.2.11".default or true); }
    ];
  }) [];


# end
# lazy_static-1.2.0

  crates.lazy_static."1.2.0" = deps: { features?(features_.lazy_static."1.2.0" deps {}) }: buildRustCrate {
    crateName = "lazy_static";
    version = "1.2.0";
    description = "A macro for declaring lazily evaluated statics in Rust.";
    authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
    sha256 = "07p3b30k2akyr6xw08ggd5qiz5nw3vd3agggj360fcc1njz7d0ss";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."lazy_static"."1.2.0" or {});
  };
  features_.lazy_static."1.2.0" = deps: f: updateFeatures f (rec {
    lazy_static = fold recursiveUpdate {} [
      { "1.2.0"."spin" =
        (f.lazy_static."1.2.0"."spin" or false) ||
        (f.lazy_static."1.2.0".spin_no_std or false) ||
        (lazy_static."1.2.0"."spin_no_std" or false); }
      { "1.2.0".default = (f.lazy_static."1.2.0".default or true); }
    ];
  }) [];


# end
# libc-0.2.46

  crates.libc."0.2.46" = deps: { features?(features_.libc."0.2.46" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.46";
    description = "Raw FFI bindings to platform libraries like libc.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "03zvz98an2j1srhlzgbh7w2l0mj1sybsg0hc2gn0s31xjw39g74k";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."libc"."0.2.46" or {});
  };
  features_.libc."0.2.46" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.46"."align" =
        (f.libc."0.2.46"."align" or false) ||
        (f.libc."0.2.46".rustc-dep-of-std or false) ||
        (libc."0.2.46"."rustc-dep-of-std" or false); }
      { "0.2.46"."rustc-std-workspace-core" =
        (f.libc."0.2.46"."rustc-std-workspace-core" or false) ||
        (f.libc."0.2.46".rustc-dep-of-std or false) ||
        (libc."0.2.46"."rustc-dep-of-std" or false); }
      { "0.2.46"."use_std" =
        (f.libc."0.2.46"."use_std" or false) ||
        (f.libc."0.2.46".default or false) ||
        (libc."0.2.46"."default" or false); }
      { "0.2.46".default = (f.libc."0.2.46".default or true); }
    ];
  }) [];


# end
# linked-hash-map-0.4.2

  crates.linked_hash_map."0.4.2" = deps: { features?(features_.linked_hash_map."0.4.2" deps {}) }: buildRustCrate {
    crateName = "linked-hash-map";
    version = "0.4.2";
    description = "A HashMap wrapper that holds key-value pairs in insertion order";
    authors = [ "Stepan Koltsov <stepan.koltsov@gmail.com>" "Andrew Paseltiner <apaseltiner@gmail.com>" ];
    sha256 = "04da208h6jb69f46j37jnvsw2i1wqplglp4d61csqcrhh83avbgl";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."linked_hash_map"."0.4.2" or {});
  };
  features_.linked_hash_map."0.4.2" = deps: f: updateFeatures f (rec {
    linked_hash_map = fold recursiveUpdate {} [
      { "0.4.2"."heapsize" =
        (f.linked_hash_map."0.4.2"."heapsize" or false) ||
        (f.linked_hash_map."0.4.2".heapsize_impl or false) ||
        (linked_hash_map."0.4.2"."heapsize_impl" or false); }
      { "0.4.2"."serde" =
        (f.linked_hash_map."0.4.2"."serde" or false) ||
        (f.linked_hash_map."0.4.2".serde_impl or false) ||
        (linked_hash_map."0.4.2"."serde_impl" or false); }
      { "0.4.2"."serde_test" =
        (f.linked_hash_map."0.4.2"."serde_test" or false) ||
        (f.linked_hash_map."0.4.2".serde_impl or false) ||
        (linked_hash_map."0.4.2"."serde_impl" or false); }
      { "0.4.2".default = (f.linked_hash_map."0.4.2".default or true); }
    ];
  }) [];


# end
# log-0.3.8

  crates.log."0.3.8" = deps: { features?(features_.log."0.3.8" deps {}) }: buildRustCrate {
    crateName = "log";
    version = "0.3.8";
    description = "A lightweight logging facade for Rust\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1c43z4z85sxrsgir4s1hi84558ab5ic7jrn5qgmsiqcv90vvn006";
    features = mkFeatures (features."log"."0.3.8" or {});
  };
  features_.log."0.3.8" = deps: f: updateFeatures f (rec {
    log = fold recursiveUpdate {} [
      { "0.3.8"."use_std" =
        (f.log."0.3.8"."use_std" or false) ||
        (f.log."0.3.8".default or false) ||
        (log."0.3.8"."default" or false); }
      { "0.3.8".default = (f.log."0.3.8".default or true); }
    ];
  }) [];


# end
# lru-cache-0.1.1

  crates.lru_cache."0.1.1" = deps: { features?(features_.lru_cache."0.1.1" deps {}) }: buildRustCrate {
    crateName = "lru-cache";
    version = "0.1.1";
    description = "A cache that holds a limited number of key-value pairs";
    authors = [ "Stepan Koltsov <stepan.koltsov@gmail.com>" ];
    sha256 = "1hl6kii1g54sq649gnscv858mmw7a02xj081l4vcgvrswdi2z8fw";
    dependencies = mapFeatures features ([
      (crates."linked_hash_map"."${deps."lru_cache"."0.1.1"."linked_hash_map"}" deps)
    ]);
    features = mkFeatures (features."lru_cache"."0.1.1" or {});
  };
  features_.lru_cache."0.1.1" = deps: f: updateFeatures f (rec {
    linked_hash_map = fold recursiveUpdate {} [
      { "${deps.lru_cache."0.1.1".linked_hash_map}"."heapsize_impl" =
        (f.linked_hash_map."${deps.lru_cache."0.1.1".linked_hash_map}"."heapsize_impl" or false) ||
        (lru_cache."0.1.1"."heapsize_impl" or false) ||
        (f."lru_cache"."0.1.1"."heapsize_impl" or false); }
      { "${deps.lru_cache."0.1.1".linked_hash_map}".default = true; }
    ];
    lru_cache = fold recursiveUpdate {} [
      { "0.1.1"."heapsize" =
        (f.lru_cache."0.1.1"."heapsize" or false) ||
        (f.lru_cache."0.1.1".heapsize_impl or false) ||
        (lru_cache."0.1.1"."heapsize_impl" or false); }
      { "0.1.1".default = (f.lru_cache."0.1.1".default or true); }
    ];
  }) [
    (features_.linked_hash_map."${deps."lru_cache"."0.1.1"."linked_hash_map"}" deps)
  ];


# end
# matches-0.1.8

  crates.matches."0.1.8" = deps: { features?(features_.matches."0.1.8" deps {}) }: buildRustCrate {
    crateName = "matches";
    version = "0.1.8";
    description = "A macro to evaluate, as a boolean, whether an expression matches a pattern.";
    authors = [ "Simon Sapin <simon.sapin@exyr.org>" ];
    sha256 = "03hl636fg6xggy0a26200xs74amk3k9n0908rga2szn68agyz3cv";
    libPath = "lib.rs";
  };
  features_.matches."0.1.8" = deps: f: updateFeatures f (rec {
    matches."0.1.8".default = (f.matches."0.1.8".default or true);
  }) [];


# end
# md5-0.3.8

  crates.md5."0.3.8" = deps: { features?(features_.md5."0.3.8" deps {}) }: buildRustCrate {
    crateName = "md5";
    version = "0.3.8";
    description = "The package provides the MD5 hash function.";
    authors = [ "Ivan Ukhov <ivan.ukhov@gmail.com>" "Kamal Ahmad <shibe@openmailbox.org>" "Konstantin Stepanov <milezv@gmail.com>" "Lukas Kalbertodt <lukas.kalbertodt@gmail.com>" "Nathan Musoke <nathan.musoke@gmail.com>" "Tony Arcieri <bascule@gmail.com>" "Wim de With <register@dewith.io>" ];
    sha256 = "0ciydcf5y3zmygzschhg4f242p9rf1d75jfj0hay4xjj29l319yd";
  };
  features_.md5."0.3.8" = deps: f: updateFeatures f (rec {
    md5."0.3.8".default = (f.md5."0.3.8".default or true);
  }) [];


# end
# memchr-0.1.11

  crates.memchr."0.1.11" = deps: { features?(features_.memchr."0.1.11" deps {}) }: buildRustCrate {
    crateName = "memchr";
    version = "0.1.11";
    description = "Safe interface to memchr.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
    sha256 = "0x73jghamvxxq5fsw9wb0shk5m6qp3q6fsf0nibn0i6bbqkw91s8";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."memchr"."0.1.11"."libc"}" deps)
    ]);
  };
  features_.memchr."0.1.11" = deps: f: updateFeatures f (rec {
    libc."${deps.memchr."0.1.11".libc}".default = true;
    memchr."0.1.11".default = (f.memchr."0.1.11".default or true);
  }) [
    (features_.libc."${deps."memchr"."0.1.11"."libc"}" deps)
  ];


# end
# memchr-2.1.2

  crates.memchr."2.1.2" = deps: { features?(features_.memchr."2.1.2" deps {}) }: buildRustCrate {
    crateName = "memchr";
    version = "2.1.2";
    description = "Safe interface to memchr.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
    sha256 = "0vdwvcmn1j65qslsxlk7fjhm53nicd5cg5hvdmbg6kybyf1lnkv1";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."memchr"."2.1.2"."cfg_if"}" deps)
    ]
      ++ (if features.memchr."2.1.2".libc or false then [ (crates.libc."${deps."memchr"."2.1.2".libc}" deps) ] else []));

    buildDependencies = mapFeatures features ([
      (crates."version_check"."${deps."memchr"."2.1.2"."version_check"}" deps)
    ]);
    features = mkFeatures (features."memchr"."2.1.2" or {});
  };
  features_.memchr."2.1.2" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.memchr."2.1.2".cfg_if}".default = true;
    libc = fold recursiveUpdate {} [
      { "${deps.memchr."2.1.2".libc}"."use_std" =
        (f.libc."${deps.memchr."2.1.2".libc}"."use_std" or false) ||
        (memchr."2.1.2"."use_std" or false) ||
        (f."memchr"."2.1.2"."use_std" or false); }
      { "${deps.memchr."2.1.2".libc}".default = (f.libc."${deps.memchr."2.1.2".libc}".default or false); }
    ];
    memchr = fold recursiveUpdate {} [
      { "2.1.2"."libc" =
        (f.memchr."2.1.2"."libc" or false) ||
        (f.memchr."2.1.2".default or false) ||
        (memchr."2.1.2"."default" or false) ||
        (f.memchr."2.1.2".use_std or false) ||
        (memchr."2.1.2"."use_std" or false); }
      { "2.1.2"."use_std" =
        (f.memchr."2.1.2"."use_std" or false) ||
        (f.memchr."2.1.2".default or false) ||
        (memchr."2.1.2"."default" or false); }
      { "2.1.2".default = (f.memchr."2.1.2".default or true); }
    ];
    version_check."${deps.memchr."2.1.2".version_check}".default = true;
  }) [
    (features_.cfg_if."${deps."memchr"."2.1.2"."cfg_if"}" deps)
    (features_.libc."${deps."memchr"."2.1.2"."libc"}" deps)
    (features_.version_check."${deps."memchr"."2.1.2"."version_check"}" deps)
  ];


# end
# mime-0.2.6

  crates.mime."0.2.6" = deps: { features?(features_.mime."0.2.6" deps {}) }: buildRustCrate {
    crateName = "mime";
    version = "0.2.6";
    description = "Strongly Typed Mimes";
    authors = [ "Sean McArthur <sean.monstar@gmail.com>" ];
    sha256 = "1skwwa0j3kqd8rm9387zgabjhp07zj99q71nzlhba4lrz9r911b3";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."mime"."0.2.6"."log"}" deps)
    ]);
    features = mkFeatures (features."mime"."0.2.6" or {});
  };
  features_.mime."0.2.6" = deps: f: updateFeatures f (rec {
    log."${deps.mime."0.2.6".log}".default = true;
    mime = fold recursiveUpdate {} [
      { "0.2.6"."heapsize" =
        (f.mime."0.2.6"."heapsize" or false) ||
        (f.mime."0.2.6".heap_size or false) ||
        (mime."0.2.6"."heap_size" or false); }
      { "0.2.6".default = (f.mime."0.2.6".default or true); }
    ];
  }) [
    (features_.log."${deps."mime"."0.2.6"."log"}" deps)
  ];


# end
# native-tls-0.1.5

  crates.native_tls."0.1.5" = deps: { features?(features_.native_tls."0.1.5" deps {}) }: buildRustCrate {
    crateName = "native-tls";
    version = "0.1.5";
    description = "A wrapper over a platform's native TLS implementation";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "11f75qmbny5pnn6zp0vlvadrvc9ph9qsxiyn4n6q02xyd93pxxlf";
    dependencies = mapFeatures features ([
      (crates."lazy_static"."${deps."native_tls"."0.1.5"."lazy_static"}" deps)
    ])
      ++ (if kernel == "darwin" || kernel == "ios" then mapFeatures features ([
      (crates."libc"."${deps."native_tls"."0.1.5"."libc"}" deps)
      (crates."security_framework"."${deps."native_tls"."0.1.5"."security_framework"}" deps)
      (crates."security_framework_sys"."${deps."native_tls"."0.1.5"."security_framework_sys"}" deps)
      (crates."tempdir"."${deps."native_tls"."0.1.5"."tempdir"}" deps)
    ]) else [])
      ++ (if !(kernel == "windows" || kernel == "darwin" || kernel == "ios") then mapFeatures features ([
      (crates."openssl"."${deps."native_tls"."0.1.5"."openssl"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."schannel"."${deps."native_tls"."0.1.5"."schannel"}" deps)
    ]) else []);
  };
  features_.native_tls."0.1.5" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.native_tls."0.1.5".lazy_static}".default = true;
    libc."${deps.native_tls."0.1.5".libc}".default = true;
    native_tls."0.1.5".default = (f.native_tls."0.1.5".default or true);
    openssl."${deps.native_tls."0.1.5".openssl}".default = true;
    schannel."${deps.native_tls."0.1.5".schannel}".default = true;
    security_framework = fold recursiveUpdate {} [
      { "${deps.native_tls."0.1.5".security_framework}"."OSX_10_8" = true; }
      { "${deps.native_tls."0.1.5".security_framework}".default = true; }
    ];
    security_framework_sys."${deps.native_tls."0.1.5".security_framework_sys}".default = true;
    tempdir."${deps.native_tls."0.1.5".tempdir}".default = true;
  }) [
    (features_.lazy_static."${deps."native_tls"."0.1.5"."lazy_static"}" deps)
    (features_.libc."${deps."native_tls"."0.1.5"."libc"}" deps)
    (features_.security_framework."${deps."native_tls"."0.1.5"."security_framework"}" deps)
    (features_.security_framework_sys."${deps."native_tls"."0.1.5"."security_framework_sys"}" deps)
    (features_.tempdir."${deps."native_tls"."0.1.5"."tempdir"}" deps)
    (features_.openssl."${deps."native_tls"."0.1.5"."openssl"}" deps)
    (features_.schannel."${deps."native_tls"."0.1.5"."schannel"}" deps)
  ];


# end
# nom-4.1.1

  crates.nom."4.1.1" = deps: { features?(features_.nom."4.1.1" deps {}) }: buildRustCrate {
    crateName = "nom";
    version = "4.1.1";
    description = "A byte-oriented, zero-copy, parser combinators library";
    authors = [ "contact@geoffroycouprie.com" ];
    sha256 = "12xd401ac6q0nf1hdd8zfx2i6ihfraa3kr1acfy3g7qz94b99635";
    dependencies = mapFeatures features ([
      (crates."memchr"."${deps."nom"."4.1.1"."memchr"}" deps)
    ]);
    features = mkFeatures (features."nom"."4.1.1" or {});
  };
  features_.nom."4.1.1" = deps: f: updateFeatures f (rec {
    memchr = fold recursiveUpdate {} [
      { "${deps.nom."4.1.1".memchr}"."use_std" =
        (f.memchr."${deps.nom."4.1.1".memchr}"."use_std" or false) ||
        (nom."4.1.1"."std" or false) ||
        (f."nom"."4.1.1"."std" or false); }
      { "${deps.nom."4.1.1".memchr}".default = (f.memchr."${deps.nom."4.1.1".memchr}".default or false); }
    ];
    nom = fold recursiveUpdate {} [
      { "4.1.1"."alloc" =
        (f.nom."4.1.1"."alloc" or false) ||
        (f.nom."4.1.1".std or false) ||
        (nom."4.1.1"."std" or false) ||
        (f.nom."4.1.1".verbose-errors or false) ||
        (nom."4.1.1"."verbose-errors" or false); }
      { "4.1.1"."lazy_static" =
        (f.nom."4.1.1"."lazy_static" or false) ||
        (f.nom."4.1.1".regexp_macros or false) ||
        (nom."4.1.1"."regexp_macros" or false); }
      { "4.1.1"."regex" =
        (f.nom."4.1.1"."regex" or false) ||
        (f.nom."4.1.1".regexp or false) ||
        (nom."4.1.1"."regexp" or false); }
      { "4.1.1"."regexp" =
        (f.nom."4.1.1"."regexp" or false) ||
        (f.nom."4.1.1".regexp_macros or false) ||
        (nom."4.1.1"."regexp_macros" or false); }
      { "4.1.1"."std" =
        (f.nom."4.1.1"."std" or false) ||
        (f.nom."4.1.1".default or false) ||
        (nom."4.1.1"."default" or false); }
      { "4.1.1".default = (f.nom."4.1.1".default or true); }
    ];
  }) [
    (features_.memchr."${deps."nom"."4.1.1"."memchr"}" deps)
  ];


# end
# num-integer-0.1.39

  crates.num_integer."0.1.39" = deps: { features?(features_.num_integer."0.1.39" deps {}) }: buildRustCrate {
    crateName = "num-integer";
    version = "0.1.39";
    description = "Integer traits and functions";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1f42ls46cghs13qfzgbd7syib2zc6m7hlmv1qlar6c9mdxapvvbg";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."num_integer"."0.1.39"."num_traits"}" deps)
    ]);
    features = mkFeatures (features."num_integer"."0.1.39" or {});
  };
  features_.num_integer."0.1.39" = deps: f: updateFeatures f (rec {
    num_integer = fold recursiveUpdate {} [
      { "0.1.39"."std" =
        (f.num_integer."0.1.39"."std" or false) ||
        (f.num_integer."0.1.39".default or false) ||
        (num_integer."0.1.39"."default" or false); }
      { "0.1.39".default = (f.num_integer."0.1.39".default or true); }
    ];
    num_traits = fold recursiveUpdate {} [
      { "${deps.num_integer."0.1.39".num_traits}"."i128" =
        (f.num_traits."${deps.num_integer."0.1.39".num_traits}"."i128" or false) ||
        (num_integer."0.1.39"."i128" or false) ||
        (f."num_integer"."0.1.39"."i128" or false); }
      { "${deps.num_integer."0.1.39".num_traits}"."std" =
        (f.num_traits."${deps.num_integer."0.1.39".num_traits}"."std" or false) ||
        (num_integer."0.1.39"."std" or false) ||
        (f."num_integer"."0.1.39"."std" or false); }
      { "${deps.num_integer."0.1.39".num_traits}".default = (f.num_traits."${deps.num_integer."0.1.39".num_traits}".default or false); }
    ];
  }) [
    (features_.num_traits."${deps."num_integer"."0.1.39"."num_traits"}" deps)
  ];


# end
# num-traits-0.1.43

  crates.num_traits."0.1.43" = deps: { features?(features_.num_traits."0.1.43" deps {}) }: buildRustCrate {
    crateName = "num-traits";
    version = "0.1.43";
    description = "Numeric traits for generic mathematics";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1zdzx78vrcg3f39w94pqjs1mwxl1phyv7843hwgwkzggwcxhhf6s";
    dependencies = mapFeatures features ([
      (crates."num_traits"."${deps."num_traits"."0.1.43"."num_traits"}" deps)
    ]);
  };
  features_.num_traits."0.1.43" = deps: f: updateFeatures f (rec {
    num_traits = fold recursiveUpdate {} [
      { "${deps.num_traits."0.1.43".num_traits}".default = true; }
      { "0.1.43".default = (f.num_traits."0.1.43".default or true); }
    ];
  }) [
    (features_.num_traits."${deps."num_traits"."0.1.43"."num_traits"}" deps)
  ];


# end
# num-traits-0.2.6

  crates.num_traits."0.2.6" = deps: { features?(features_.num_traits."0.2.6" deps {}) }: buildRustCrate {
    crateName = "num-traits";
    version = "0.2.6";
    description = "Numeric traits for generic mathematics";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1d20sil9n0wgznd1nycm3yjfj1mzyl41ambb7by1apxlyiil1azk";
    build = "build.rs";
    features = mkFeatures (features."num_traits"."0.2.6" or {});
  };
  features_.num_traits."0.2.6" = deps: f: updateFeatures f (rec {
    num_traits = fold recursiveUpdate {} [
      { "0.2.6"."std" =
        (f.num_traits."0.2.6"."std" or false) ||
        (f.num_traits."0.2.6".default or false) ||
        (num_traits."0.2.6"."default" or false); }
      { "0.2.6".default = (f.num_traits."0.2.6".default or true); }
    ];
  }) [];


# end
# num_cpus-1.9.0

  crates.num_cpus."1.9.0" = deps: { features?(features_.num_cpus."1.9.0" deps {}) }: buildRustCrate {
    crateName = "num_cpus";
    version = "1.9.0";
    description = "Get the number of CPUs on a machine.";
    authors = [ "Sean McArthur <sean@seanmonstar.com>" ];
    sha256 = "0lv81a9sapkprfsi03rag1mygm9qxhdw2qscdvvx2yb62pc54pvi";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."num_cpus"."1.9.0"."libc"}" deps)
    ]);
  };
  features_.num_cpus."1.9.0" = deps: f: updateFeatures f (rec {
    libc."${deps.num_cpus."1.9.0".libc}".default = true;
    num_cpus."1.9.0".default = (f.num_cpus."1.9.0".default or true);
  }) [
    (features_.libc."${deps."num_cpus"."1.9.0"."libc"}" deps)
  ];


# end
# openssl-0.9.24

  crates.openssl."0.9.24" = deps: { features?(features_.openssl."0.9.24" deps {}) }: buildRustCrate {
    crateName = "openssl";
    version = "0.9.24";
    description = "OpenSSL bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "0wzm3c11g3ndaqyzq36mcdcm1q4a8pmsyi33ibybhjz28g2z0f79";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."openssl"."0.9.24"."bitflags"}" deps)
      (crates."foreign_types"."${deps."openssl"."0.9.24"."foreign_types"}" deps)
      (crates."lazy_static"."${deps."openssl"."0.9.24"."lazy_static"}" deps)
      (crates."libc"."${deps."openssl"."0.9.24"."libc"}" deps)
      (crates."openssl_sys"."${deps."openssl"."0.9.24"."openssl_sys"}" deps)
    ]);
    features = mkFeatures (features."openssl"."0.9.24" or {});
  };
  features_.openssl."0.9.24" = deps: f: updateFeatures f (rec {
    bitflags."${deps.openssl."0.9.24".bitflags}".default = true;
    foreign_types."${deps.openssl."0.9.24".foreign_types}".default = true;
    lazy_static."${deps.openssl."0.9.24".lazy_static}".default = true;
    libc."${deps.openssl."0.9.24".libc}".default = true;
    openssl."0.9.24".default = (f.openssl."0.9.24".default or true);
    openssl_sys."${deps.openssl."0.9.24".openssl_sys}".default = true;
  }) [
    (features_.bitflags."${deps."openssl"."0.9.24"."bitflags"}" deps)
    (features_.foreign_types."${deps."openssl"."0.9.24"."foreign_types"}" deps)
    (features_.lazy_static."${deps."openssl"."0.9.24"."lazy_static"}" deps)
    (features_.libc."${deps."openssl"."0.9.24"."libc"}" deps)
    (features_.openssl_sys."${deps."openssl"."0.9.24"."openssl_sys"}" deps)
  ];


# end
# openssl-0.10.16

  crates.openssl."0.10.16" = deps: { features?(features_.openssl."0.10.16" deps {}) }: buildRustCrate {
    crateName = "openssl";
    version = "0.10.16";
    description = "OpenSSL bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "17mi6p323rqkydfwykiba3b1a24j7jv7bmr7j5wai4c32i2khqsm";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."openssl"."0.10.16"."bitflags"}" deps)
      (crates."cfg_if"."${deps."openssl"."0.10.16"."cfg_if"}" deps)
      (crates."foreign_types"."${deps."openssl"."0.10.16"."foreign_types"}" deps)
      (crates."lazy_static"."${deps."openssl"."0.10.16"."lazy_static"}" deps)
      (crates."libc"."${deps."openssl"."0.10.16"."libc"}" deps)
      (crates."openssl_sys"."${deps."openssl"."0.10.16"."openssl_sys"}" deps)
    ]);
    features = mkFeatures (features."openssl"."0.10.16" or {});
  };
  features_.openssl."0.10.16" = deps: f: updateFeatures f (rec {
    bitflags."${deps.openssl."0.10.16".bitflags}".default = true;
    cfg_if."${deps.openssl."0.10.16".cfg_if}".default = true;
    foreign_types."${deps.openssl."0.10.16".foreign_types}".default = true;
    lazy_static."${deps.openssl."0.10.16".lazy_static}".default = true;
    libc."${deps.openssl."0.10.16".libc}".default = true;
    openssl."0.10.16".default = (f.openssl."0.10.16".default or true);
    openssl_sys = fold recursiveUpdate {} [
      { "${deps.openssl."0.10.16".openssl_sys}"."vendored" =
        (f.openssl_sys."${deps.openssl."0.10.16".openssl_sys}"."vendored" or false) ||
        (openssl."0.10.16"."vendored" or false) ||
        (f."openssl"."0.10.16"."vendored" or false); }
      { "${deps.openssl."0.10.16".openssl_sys}".default = true; }
    ];
  }) [
    (features_.bitflags."${deps."openssl"."0.10.16"."bitflags"}" deps)
    (features_.cfg_if."${deps."openssl"."0.10.16"."cfg_if"}" deps)
    (features_.foreign_types."${deps."openssl"."0.10.16"."foreign_types"}" deps)
    (features_.lazy_static."${deps."openssl"."0.10.16"."lazy_static"}" deps)
    (features_.libc."${deps."openssl"."0.10.16"."libc"}" deps)
    (features_.openssl_sys."${deps."openssl"."0.10.16"."openssl_sys"}" deps)
  ];


# end
# openssl-sys-0.9.40

  crates.openssl_sys."0.9.40" = deps: { features?(features_.openssl_sys."0.9.40" deps {}) }: buildRustCrate {
    crateName = "openssl-sys";
    version = "0.9.40";
    description = "FFI bindings to OpenSSL";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "11dqyk9g2wdwwj21zma71w5hd5d4sw3hm4pnpk8jjh0wjpkgjdvq";
    build = "build/main.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."openssl_sys"."0.9.40"."libc"}" deps)
    ])
      ++ (if abi == "msvc" then mapFeatures features ([
]) else []);

    buildDependencies = mapFeatures features ([
      (crates."cc"."${deps."openssl_sys"."0.9.40"."cc"}" deps)
      (crates."pkg_config"."${deps."openssl_sys"."0.9.40"."pkg_config"}" deps)
    ]);
    features = mkFeatures (features."openssl_sys"."0.9.40" or {});
  };
  features_.openssl_sys."0.9.40" = deps: f: updateFeatures f (rec {
    cc."${deps.openssl_sys."0.9.40".cc}".default = true;
    libc."${deps.openssl_sys."0.9.40".libc}".default = true;
    openssl_sys = fold recursiveUpdate {} [
      { "0.9.40"."openssl-src" =
        (f.openssl_sys."0.9.40"."openssl-src" or false) ||
        (f.openssl_sys."0.9.40".vendored or false) ||
        (openssl_sys."0.9.40"."vendored" or false); }
      { "0.9.40".default = (f.openssl_sys."0.9.40".default or true); }
    ];
    pkg_config."${deps.openssl_sys."0.9.40".pkg_config}".default = true;
  }) [
    (features_.libc."${deps."openssl_sys"."0.9.40"."libc"}" deps)
    (features_.cc."${deps."openssl_sys"."0.9.40"."cc"}" deps)
    (features_.pkg_config."${deps."openssl_sys"."0.9.40"."pkg_config"}" deps)
  ];


# end
# percent-encoding-1.0.1

  crates.percent_encoding."1.0.1" = deps: { features?(features_.percent_encoding."1.0.1" deps {}) }: buildRustCrate {
    crateName = "percent-encoding";
    version = "1.0.1";
    description = "Percent encoding and decoding";
    authors = [ "The rust-url developers" ];
    sha256 = "04ahrp7aw4ip7fmadb0bknybmkfav0kk0gw4ps3ydq5w6hr0ib5i";
    libPath = "lib.rs";
  };
  features_.percent_encoding."1.0.1" = deps: f: updateFeatures f (rec {
    percent_encoding."1.0.1".default = (f.percent_encoding."1.0.1".default or true);
  }) [];


# end
# pkg-config-0.3.14

  crates.pkg_config."0.3.14" = deps: { features?(features_.pkg_config."0.3.14" deps {}) }: buildRustCrate {
    crateName = "pkg-config";
    version = "0.3.14";
    description = "A library to run the pkg-config system tool at build time in order to be used in\nCargo build scripts.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0207fsarrm412j0dh87lfcas72n8mxar7q3mgflsbsrqnb140sv6";
  };
  features_.pkg_config."0.3.14" = deps: f: updateFeatures f (rec {
    pkg_config."0.3.14".default = (f.pkg_config."0.3.14".default or true);
  }) [];


# end
# proc-macro2-0.4.24

  crates.proc_macro2."0.4.24" = deps: { features?(features_.proc_macro2."0.4.24" deps {}) }: buildRustCrate {
    crateName = "proc-macro2";
    version = "0.4.24";
    description = "A stable implementation of the upcoming new `proc_macro` API. Comes with an\noption, off by default, to also reimplement itself in terms of the upstream\nunstable API.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0ra2z9j3h0bbfq40p8mfwf28shnbxqryb45pfzg47xaszf85ylv2";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."unicode_xid"."${deps."proc_macro2"."0.4.24"."unicode_xid"}" deps)
    ]);
    features = mkFeatures (features."proc_macro2"."0.4.24" or {});
  };
  features_.proc_macro2."0.4.24" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "0.4.24"."proc-macro" =
        (f.proc_macro2."0.4.24"."proc-macro" or false) ||
        (f.proc_macro2."0.4.24".default or false) ||
        (proc_macro2."0.4.24"."default" or false) ||
        (f.proc_macro2."0.4.24".nightly or false) ||
        (proc_macro2."0.4.24"."nightly" or false); }
      { "0.4.24".default = (f.proc_macro2."0.4.24".default or true); }
    ];
    unicode_xid."${deps.proc_macro2."0.4.24".unicode_xid}".default = true;
  }) [
    (features_.unicode_xid."${deps."proc_macro2"."0.4.24"."unicode_xid"}" deps)
  ];


# end
# quote-0.6.10

  crates.quote."0.6.10" = deps: { features?(features_.quote."0.6.10" deps {}) }: buildRustCrate {
    crateName = "quote";
    version = "0.6.10";
    description = "Quasi-quoting macro quote!(...)";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0q5dlhk9hz795872fsf02vlbazx691393j7q426q590vdqcgj0qx";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."quote"."0.6.10"."proc_macro2"}" deps)
    ]);
    features = mkFeatures (features."quote"."0.6.10" or {});
  };
  features_.quote."0.6.10" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.quote."0.6.10".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.quote."0.6.10".proc_macro2}"."proc-macro" or false) ||
        (quote."0.6.10"."proc-macro" or false) ||
        (f."quote"."0.6.10"."proc-macro" or false); }
      { "${deps.quote."0.6.10".proc_macro2}".default = (f.proc_macro2."${deps.quote."0.6.10".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "0.6.10"."proc-macro" =
        (f.quote."0.6.10"."proc-macro" or false) ||
        (f.quote."0.6.10".default or false) ||
        (quote."0.6.10"."default" or false); }
      { "0.6.10".default = (f.quote."0.6.10".default or true); }
    ];
  }) [
    (features_.proc_macro2."${deps."quote"."0.6.10"."proc_macro2"}" deps)
  ];


# end
# rand-0.3.22

  crates.rand."0.3.22" = deps: { features?(features_.rand."0.3.22" deps {}) }: buildRustCrate {
    crateName = "rand";
    version = "0.3.22";
    description = "Random number generators and other randomness functionality.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0wrj12acx7l4hr7ag3nz8b50yhp8ancyq988bzmnnsxln67rsys0";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."rand"."0.3.22"."libc"}" deps)
      (crates."rand"."${deps."rand"."0.3.22"."rand"}" deps)
    ])
      ++ (if kernel == "fuchsia" then mapFeatures features ([
      (crates."fuchsia_zircon"."${deps."rand"."0.3.22"."fuchsia_zircon"}" deps)
    ]) else []);
    features = mkFeatures (features."rand"."0.3.22" or {});
  };
  features_.rand."0.3.22" = deps: f: updateFeatures f (rec {
    fuchsia_zircon."${deps.rand."0.3.22".fuchsia_zircon}".default = true;
    libc."${deps.rand."0.3.22".libc}".default = true;
    rand = fold recursiveUpdate {} [
      { "${deps.rand."0.3.22".rand}".default = true; }
      { "0.3.22"."i128_support" =
        (f.rand."0.3.22"."i128_support" or false) ||
        (f.rand."0.3.22".nightly or false) ||
        (rand."0.3.22"."nightly" or false); }
      { "0.3.22".default = (f.rand."0.3.22".default or true); }
    ];
  }) [
    (features_.libc."${deps."rand"."0.3.22"."libc"}" deps)
    (features_.rand."${deps."rand"."0.3.22"."rand"}" deps)
    (features_.fuchsia_zircon."${deps."rand"."0.3.22"."fuchsia_zircon"}" deps)
  ];


# end
# rand-0.4.3

  crates.rand."0.4.3" = deps: { features?(features_.rand."0.4.3" deps {}) }: buildRustCrate {
    crateName = "rand";
    version = "0.4.3";
    description = "Random number generators and other randomness functionality.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1644wri45l147822xy7dgdm4k7myxzs66cb795ga0x7dan11ci4f";
    dependencies = (if kernel == "fuchsia" then mapFeatures features ([
      (crates."fuchsia_zircon"."${deps."rand"."0.4.3"."fuchsia_zircon"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
    ]
      ++ (if features.rand."0.4.3".libc or false then [ (crates.libc."${deps."rand"."0.4.3".libc}" deps) ] else [])) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."rand"."0.4.3"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."rand"."0.4.3" or {});
  };
  features_.rand."0.4.3" = deps: f: updateFeatures f (rec {
    fuchsia_zircon."${deps.rand."0.4.3".fuchsia_zircon}".default = true;
    libc."${deps.rand."0.4.3".libc}".default = true;
    rand = fold recursiveUpdate {} [
      { "0.4.3"."i128_support" =
        (f.rand."0.4.3"."i128_support" or false) ||
        (f.rand."0.4.3".nightly or false) ||
        (rand."0.4.3"."nightly" or false); }
      { "0.4.3"."libc" =
        (f.rand."0.4.3"."libc" or false) ||
        (f.rand."0.4.3".std or false) ||
        (rand."0.4.3"."std" or false); }
      { "0.4.3"."std" =
        (f.rand."0.4.3"."std" or false) ||
        (f.rand."0.4.3".default or false) ||
        (rand."0.4.3"."default" or false); }
      { "0.4.3".default = (f.rand."0.4.3".default or true); }
    ];
    winapi = fold recursiveUpdate {} [
      { "${deps.rand."0.4.3".winapi}"."minwindef" = true; }
      { "${deps.rand."0.4.3".winapi}"."ntsecapi" = true; }
      { "${deps.rand."0.4.3".winapi}"."profileapi" = true; }
      { "${deps.rand."0.4.3".winapi}"."winnt" = true; }
      { "${deps.rand."0.4.3".winapi}".default = true; }
    ];
  }) [
    (features_.fuchsia_zircon."${deps."rand"."0.4.3"."fuchsia_zircon"}" deps)
    (features_.libc."${deps."rand"."0.4.3"."libc"}" deps)
    (features_.winapi."${deps."rand"."0.4.3"."winapi"}" deps)
  ];


# end
# redox_syscall-0.1.50

  crates.redox_syscall."0.1.50" = deps: { features?(features_.redox_syscall."0.1.50" deps {}) }: buildRustCrate {
    crateName = "redox_syscall";
    version = "0.1.50";
    description = "A Rust library to access raw Redox system calls";
    authors = [ "Jeremy Soller <jackpot51@gmail.com>" ];
    sha256 = "0f7lpamlizfv9cbyyqwwzp9rbk66gppqdx5kw5vq24cfzy9dbpiw";
    libName = "syscall";
  };
  features_.redox_syscall."0.1.50" = deps: f: updateFeatures f (rec {
    redox_syscall."0.1.50".default = (f.redox_syscall."0.1.50".default or true);
  }) [];


# end
# regex-0.1.80

  crates.regex."0.1.80" = deps: { features?(features_.regex."0.1.80" deps {}) }: buildRustCrate {
    crateName = "regex";
    version = "0.1.80";
    description = "An implementation of regular expressions for Rust. This implementation uses\nfinite automata and guarantees linear time matching on all inputs.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0y4s8ghhx6sgzb35irwivm3w0l2hhqhmdcd2px9hirqnkagal9l6";
    dependencies = mapFeatures features ([
      (crates."aho_corasick"."${deps."regex"."0.1.80"."aho_corasick"}" deps)
      (crates."memchr"."${deps."regex"."0.1.80"."memchr"}" deps)
      (crates."regex_syntax"."${deps."regex"."0.1.80"."regex_syntax"}" deps)
      (crates."thread_local"."${deps."regex"."0.1.80"."thread_local"}" deps)
      (crates."utf8_ranges"."${deps."regex"."0.1.80"."utf8_ranges"}" deps)
    ]);
    features = mkFeatures (features."regex"."0.1.80" or {});
  };
  features_.regex."0.1.80" = deps: f: updateFeatures f (rec {
    aho_corasick."${deps.regex."0.1.80".aho_corasick}".default = true;
    memchr."${deps.regex."0.1.80".memchr}".default = true;
    regex = fold recursiveUpdate {} [
      { "0.1.80"."simd" =
        (f.regex."0.1.80"."simd" or false) ||
        (f.regex."0.1.80".simd-accel or false) ||
        (regex."0.1.80"."simd-accel" or false); }
      { "0.1.80".default = (f.regex."0.1.80".default or true); }
    ];
    regex_syntax."${deps.regex."0.1.80".regex_syntax}".default = true;
    thread_local."${deps.regex."0.1.80".thread_local}".default = true;
    utf8_ranges."${deps.regex."0.1.80".utf8_ranges}".default = true;
  }) [
    (features_.aho_corasick."${deps."regex"."0.1.80"."aho_corasick"}" deps)
    (features_.memchr."${deps."regex"."0.1.80"."memchr"}" deps)
    (features_.regex_syntax."${deps."regex"."0.1.80"."regex_syntax"}" deps)
    (features_.thread_local."${deps."regex"."0.1.80"."thread_local"}" deps)
    (features_.utf8_ranges."${deps."regex"."0.1.80"."utf8_ranges"}" deps)
  ];


# end
# regex-0.2.11

  crates.regex."0.2.11" = deps: { features?(features_.regex."0.2.11" deps {}) }: buildRustCrate {
    crateName = "regex";
    version = "0.2.11";
    description = "An implementation of regular expressions for Rust. This implementation uses\nfinite automata and guarantees linear time matching on all inputs.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0r50cymxdqp0fv1dxd22mjr6y32q450nwacd279p9s7lh0cafijj";
    dependencies = mapFeatures features ([
      (crates."aho_corasick"."${deps."regex"."0.2.11"."aho_corasick"}" deps)
      (crates."memchr"."${deps."regex"."0.2.11"."memchr"}" deps)
      (crates."regex_syntax"."${deps."regex"."0.2.11"."regex_syntax"}" deps)
      (crates."thread_local"."${deps."regex"."0.2.11"."thread_local"}" deps)
      (crates."utf8_ranges"."${deps."regex"."0.2.11"."utf8_ranges"}" deps)
    ]);
    features = mkFeatures (features."regex"."0.2.11" or {});
  };
  features_.regex."0.2.11" = deps: f: updateFeatures f (rec {
    aho_corasick."${deps.regex."0.2.11".aho_corasick}".default = true;
    memchr."${deps.regex."0.2.11".memchr}".default = true;
    regex = fold recursiveUpdate {} [
      { "0.2.11"."pattern" =
        (f.regex."0.2.11"."pattern" or false) ||
        (f.regex."0.2.11".unstable or false) ||
        (regex."0.2.11"."unstable" or false); }
      { "0.2.11".default = (f.regex."0.2.11".default or true); }
    ];
    regex_syntax."${deps.regex."0.2.11".regex_syntax}".default = true;
    thread_local."${deps.regex."0.2.11".thread_local}".default = true;
    utf8_ranges."${deps.regex."0.2.11".utf8_ranges}".default = true;
  }) [
    (features_.aho_corasick."${deps."regex"."0.2.11"."aho_corasick"}" deps)
    (features_.memchr."${deps."regex"."0.2.11"."memchr"}" deps)
    (features_.regex_syntax."${deps."regex"."0.2.11"."regex_syntax"}" deps)
    (features_.thread_local."${deps."regex"."0.2.11"."thread_local"}" deps)
    (features_.utf8_ranges."${deps."regex"."0.2.11"."utf8_ranges"}" deps)
  ];


# end
# regex-syntax-0.3.9

  crates.regex_syntax."0.3.9" = deps: { features?(features_.regex_syntax."0.3.9" deps {}) }: buildRustCrate {
    crateName = "regex-syntax";
    version = "0.3.9";
    description = "A regular expression parser.";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1mzhphkbwppwd1zam2jkgjk550cqgf6506i87bw2yzrvcsraiw7m";
  };
  features_.regex_syntax."0.3.9" = deps: f: updateFeatures f (rec {
    regex_syntax."0.3.9".default = (f.regex_syntax."0.3.9".default or true);
  }) [];


# end
# regex-syntax-0.5.6

  crates.regex_syntax."0.5.6" = deps: { features?(features_.regex_syntax."0.5.6" deps {}) }: buildRustCrate {
    crateName = "regex-syntax";
    version = "0.5.6";
    description = "A regular expression parser.";
    authors = [ "The Rust Project Developers" ];
    sha256 = "10vf3r34bgjnbrnqd5aszn35bjvm8insw498l1vjy8zx5yms3427";
    dependencies = mapFeatures features ([
      (crates."ucd_util"."${deps."regex_syntax"."0.5.6"."ucd_util"}" deps)
    ]);
  };
  features_.regex_syntax."0.5.6" = deps: f: updateFeatures f (rec {
    regex_syntax."0.5.6".default = (f.regex_syntax."0.5.6".default or true);
    ucd_util."${deps.regex_syntax."0.5.6".ucd_util}".default = true;
  }) [
    (features_.ucd_util."${deps."regex_syntax"."0.5.6"."ucd_util"}" deps)
  ];


# end
# remove_dir_all-0.5.1

  crates.remove_dir_all."0.5.1" = deps: { features?(features_.remove_dir_all."0.5.1" deps {}) }: buildRustCrate {
    crateName = "remove_dir_all";
    version = "0.5.1";
    description = "A safe, reliable implementation of remove_dir_all for Windows";
    authors = [ "Aaronepower <theaaronepower@gmail.com>" ];
    sha256 = "1chx3yvfbj46xjz4bzsvps208l46hfbcy0sm98gpiya454n4rrl7";
    dependencies = (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."remove_dir_all"."0.5.1"."winapi"}" deps)
    ]) else []);
  };
  features_.remove_dir_all."0.5.1" = deps: f: updateFeatures f (rec {
    remove_dir_all."0.5.1".default = (f.remove_dir_all."0.5.1".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.remove_dir_all."0.5.1".winapi}"."errhandlingapi" = true; }
      { "${deps.remove_dir_all."0.5.1".winapi}"."fileapi" = true; }
      { "${deps.remove_dir_all."0.5.1".winapi}"."std" = true; }
      { "${deps.remove_dir_all."0.5.1".winapi}"."winbase" = true; }
      { "${deps.remove_dir_all."0.5.1".winapi}"."winerror" = true; }
      { "${deps.remove_dir_all."0.5.1".winapi}".default = true; }
    ];
  }) [
    (features_.winapi."${deps."remove_dir_all"."0.5.1"."winapi"}" deps)
  ];


# end
# rustc-demangle-0.1.13

  crates.rustc_demangle."0.1.13" = deps: { features?(features_.rustc_demangle."0.1.13" deps {}) }: buildRustCrate {
    crateName = "rustc-demangle";
    version = "0.1.13";
    description = "Rust compiler symbol demangling.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0sr6cr02araqnlqwc5ghvnafjmkw11vzjswqaz757lvyrcl8xcy6";
  };
  features_.rustc_demangle."0.1.13" = deps: f: updateFeatures f (rec {
    rustc_demangle."0.1.13".default = (f.rustc_demangle."0.1.13".default or true);
  }) [];


# end
# ryu-0.2.7

  crates.ryu."0.2.7" = deps: { features?(features_.ryu."0.2.7" deps {}) }: buildRustCrate {
    crateName = "ryu";
    version = "0.2.7";
    description = "Fast floating point to string conversion";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0m8szf1m87wfqkwh1f9zp9bn2mb0m9nav028xxnd0hlig90b44bd";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."ryu"."0.2.7" or {});
  };
  features_.ryu."0.2.7" = deps: f: updateFeatures f (rec {
    ryu."0.2.7".default = (f.ryu."0.2.7".default or true);
  }) [];


# end
# safemem-0.3.0

  crates.safemem."0.3.0" = deps: { features?(features_.safemem."0.3.0" deps {}) }: buildRustCrate {
    crateName = "safemem";
    version = "0.3.0";
    description = "Safe wrappers for memory-accessing functions, like `std::ptr::copy()`.";
    authors = [ "Austin Bonander <austin.bonander@gmail.com>" ];
    sha256 = "0pr39b468d05f6m7m4alsngmj5p7an8df21apsxbi57k0lmwrr18";
    features = mkFeatures (features."safemem"."0.3.0" or {});
  };
  features_.safemem."0.3.0" = deps: f: updateFeatures f (rec {
    safemem = fold recursiveUpdate {} [
      { "0.3.0"."std" =
        (f.safemem."0.3.0"."std" or false) ||
        (f.safemem."0.3.0".default or false) ||
        (safemem."0.3.0"."default" or false); }
      { "0.3.0".default = (f.safemem."0.3.0".default or true); }
    ];
  }) [];


# end
# schannel-0.1.14

  crates.schannel."0.1.14" = deps: { features?(features_.schannel."0.1.14" deps {}) }: buildRustCrate {
    crateName = "schannel";
    version = "0.1.14";
    description = "Schannel bindings for rust, allowing SSL/TLS (e.g. https) without openssl";
    authors = [ "Steven Fackler <sfackler@gmail.com>" "Steffen Butzer <steffen.butzer@outlook.com>" ];
    sha256 = "1g0a88jknns1kwn3x1k3ci5y5zvg58pwdg1xrxkrw3cwd2hynm9k";
    dependencies = mapFeatures features ([
      (crates."lazy_static"."${deps."schannel"."0.1.14"."lazy_static"}" deps)
      (crates."winapi"."${deps."schannel"."0.1.14"."winapi"}" deps)
    ]);
  };
  features_.schannel."0.1.14" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.schannel."0.1.14".lazy_static}".default = true;
    schannel."0.1.14".default = (f.schannel."0.1.14".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.schannel."0.1.14".winapi}"."lmcons" = true; }
      { "${deps.schannel."0.1.14".winapi}"."minschannel" = true; }
      { "${deps.schannel."0.1.14".winapi}"."schannel" = true; }
      { "${deps.schannel."0.1.14".winapi}"."securitybaseapi" = true; }
      { "${deps.schannel."0.1.14".winapi}"."sspi" = true; }
      { "${deps.schannel."0.1.14".winapi}"."sysinfoapi" = true; }
      { "${deps.schannel."0.1.14".winapi}"."timezoneapi" = true; }
      { "${deps.schannel."0.1.14".winapi}"."winbase" = true; }
      { "${deps.schannel."0.1.14".winapi}"."wincrypt" = true; }
      { "${deps.schannel."0.1.14".winapi}"."winerror" = true; }
      { "${deps.schannel."0.1.14".winapi}".default = true; }
    ];
  }) [
    (features_.lazy_static."${deps."schannel"."0.1.14"."lazy_static"}" deps)
    (features_.winapi."${deps."schannel"."0.1.14"."winapi"}" deps)
  ];


# end
# security-framework-0.1.16

  crates.security_framework."0.1.16" = deps: { features?(features_.security_framework."0.1.16" deps {}) }: buildRustCrate {
    crateName = "security-framework";
    version = "0.1.16";
    description = "Security Framework bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "1kxczsaj8gz4922jl5af2gkxh71rasb6khaf3dp7ldlnw9qf2sbm";
    dependencies = mapFeatures features ([
      (crates."core_foundation"."${deps."security_framework"."0.1.16"."core_foundation"}" deps)
      (crates."core_foundation_sys"."${deps."security_framework"."0.1.16"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."security_framework"."0.1.16"."libc"}" deps)
      (crates."security_framework_sys"."${deps."security_framework"."0.1.16"."security_framework_sys"}" deps)
    ]);
    features = mkFeatures (features."security_framework"."0.1.16" or {});
  };
  features_.security_framework."0.1.16" = deps: f: updateFeatures f (rec {
    core_foundation."${deps.security_framework."0.1.16".core_foundation}".default = true;
    core_foundation_sys."${deps.security_framework."0.1.16".core_foundation_sys}".default = true;
    libc."${deps.security_framework."0.1.16".libc}".default = true;
    security_framework = fold recursiveUpdate {} [
      { "0.1.16"."OSX_10_10" =
        (f.security_framework."0.1.16"."OSX_10_10" or false) ||
        (f.security_framework."0.1.16".OSX_10_11 or false) ||
        (security_framework."0.1.16"."OSX_10_11" or false); }
      { "0.1.16"."OSX_10_11" =
        (f.security_framework."0.1.16"."OSX_10_11" or false) ||
        (f.security_framework."0.1.16".OSX_10_12 or false) ||
        (security_framework."0.1.16"."OSX_10_12" or false); }
      { "0.1.16"."OSX_10_8" =
        (f.security_framework."0.1.16"."OSX_10_8" or false) ||
        (f.security_framework."0.1.16".OSX_10_9 or false) ||
        (security_framework."0.1.16"."OSX_10_9" or false); }
      { "0.1.16"."OSX_10_9" =
        (f.security_framework."0.1.16"."OSX_10_9" or false) ||
        (f.security_framework."0.1.16".OSX_10_10 or false) ||
        (security_framework."0.1.16"."OSX_10_10" or false); }
      { "0.1.16".default = (f.security_framework."0.1.16".default or true); }
    ];
    security_framework_sys = fold recursiveUpdate {} [
      { "${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_10" =
        (f.security_framework_sys."${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_10" or false) ||
        (security_framework."0.1.16"."OSX_10_10" or false) ||
        (f."security_framework"."0.1.16"."OSX_10_10" or false); }
      { "${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_11" =
        (f.security_framework_sys."${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_11" or false) ||
        (security_framework."0.1.16"."OSX_10_11" or false) ||
        (f."security_framework"."0.1.16"."OSX_10_11" or false) ||
        (security_framework."0.1.16"."OSX_10_12" or false) ||
        (f."security_framework"."0.1.16"."OSX_10_12" or false); }
      { "${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_8" =
        (f.security_framework_sys."${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_8" or false) ||
        (security_framework."0.1.16"."OSX_10_8" or false) ||
        (f."security_framework"."0.1.16"."OSX_10_8" or false); }
      { "${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_9" =
        (f.security_framework_sys."${deps.security_framework."0.1.16".security_framework_sys}"."OSX_10_9" or false) ||
        (security_framework."0.1.16"."OSX_10_9" or false) ||
        (f."security_framework"."0.1.16"."OSX_10_9" or false); }
      { "${deps.security_framework."0.1.16".security_framework_sys}".default = true; }
    ];
  }) [
    (features_.core_foundation."${deps."security_framework"."0.1.16"."core_foundation"}" deps)
    (features_.core_foundation_sys."${deps."security_framework"."0.1.16"."core_foundation_sys"}" deps)
    (features_.libc."${deps."security_framework"."0.1.16"."libc"}" deps)
    (features_.security_framework_sys."${deps."security_framework"."0.1.16"."security_framework_sys"}" deps)
  ];


# end
# security-framework-sys-0.1.16

  crates.security_framework_sys."0.1.16" = deps: { features?(features_.security_framework_sys."0.1.16" deps {}) }: buildRustCrate {
    crateName = "security-framework-sys";
    version = "0.1.16";
    description = "Security Framework bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "0ai2pivdr5fyc7czbkpcrwap0imyy0r8ndarrl3n5kiv0jha1js3";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."core_foundation_sys"."${deps."security_framework_sys"."0.1.16"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."security_framework_sys"."0.1.16"."libc"}" deps)
    ]);
    features = mkFeatures (features."security_framework_sys"."0.1.16" or {});
  };
  features_.security_framework_sys."0.1.16" = deps: f: updateFeatures f (rec {
    core_foundation_sys."${deps.security_framework_sys."0.1.16".core_foundation_sys}".default = true;
    libc."${deps.security_framework_sys."0.1.16".libc}".default = true;
    security_framework_sys = fold recursiveUpdate {} [
      { "0.1.16"."OSX_10_10" =
        (f.security_framework_sys."0.1.16"."OSX_10_10" or false) ||
        (f.security_framework_sys."0.1.16".OSX_10_11 or false) ||
        (security_framework_sys."0.1.16"."OSX_10_11" or false); }
      { "0.1.16"."OSX_10_11" =
        (f.security_framework_sys."0.1.16"."OSX_10_11" or false) ||
        (f.security_framework_sys."0.1.16".OSX_10_12 or false) ||
        (security_framework_sys."0.1.16"."OSX_10_12" or false); }
      { "0.1.16"."OSX_10_8" =
        (f.security_framework_sys."0.1.16"."OSX_10_8" or false) ||
        (f.security_framework_sys."0.1.16".OSX_10_9 or false) ||
        (security_framework_sys."0.1.16"."OSX_10_9" or false); }
      { "0.1.16"."OSX_10_9" =
        (f.security_framework_sys."0.1.16"."OSX_10_9" or false) ||
        (f.security_framework_sys."0.1.16".OSX_10_10 or false) ||
        (security_framework_sys."0.1.16"."OSX_10_10" or false); }
      { "0.1.16".default = (f.security_framework_sys."0.1.16".default or true); }
    ];
  }) [
    (features_.core_foundation_sys."${deps."security_framework_sys"."0.1.16"."core_foundation_sys"}" deps)
    (features_.libc."${deps."security_framework_sys"."0.1.16"."libc"}" deps)
  ];


# end
# separator-0.4.1

  crates.separator."0.4.1" = deps: { features?(features_.separator."0.4.1" deps {}) }: buildRustCrate {
    crateName = "separator";
    version = "0.4.1";
    description = "Formats numbers into strings with thousands separators for readability.";
    authors = [ "Saghm Rossi <saghmrossi@gmail.com>" ];
    edition = "2018";
    sha256 = "1l7yhf6dy09k9cy0kkwb9wy98rn8mnz72q27wbd6bhiflllwghr7";
  };
  features_.separator."0.4.1" = deps: f: updateFeatures f (rec {
    separator."0.4.1".default = (f.separator."0.4.1".default or true);
  }) [];


# end
# serde-1.0.84

  crates.serde."1.0.84" = deps: { features?(features_.serde."1.0.84" deps {}) }: buildRustCrate {
    crateName = "serde";
    version = "1.0.84";
    description = "A generic serialization/deserialization framework";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1x40cvvkbkz592jflwbfbxhim3wxdqp9dy0qxjw13ra7q57b29gy";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."serde"."1.0.84" or {});
  };
  features_.serde."1.0.84" = deps: f: updateFeatures f (rec {
    serde = fold recursiveUpdate {} [
      { "1.0.84"."serde_derive" =
        (f.serde."1.0.84"."serde_derive" or false) ||
        (f.serde."1.0.84".derive or false) ||
        (serde."1.0.84"."derive" or false); }
      { "1.0.84"."std" =
        (f.serde."1.0.84"."std" or false) ||
        (f.serde."1.0.84".default or false) ||
        (serde."1.0.84"."default" or false); }
      { "1.0.84"."unstable" =
        (f.serde."1.0.84"."unstable" or false) ||
        (f.serde."1.0.84".alloc or false) ||
        (serde."1.0.84"."alloc" or false); }
      { "1.0.84".default = (f.serde."1.0.84".default or true); }
    ];
  }) [];


# end
# serde_derive-1.0.84

  crates.serde_derive."1.0.84" = deps: { features?(features_.serde_derive."1.0.84" deps {}) }: buildRustCrate {
    crateName = "serde_derive";
    version = "1.0.84";
    description = "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0iz0f0z86k1kc57xaa4zjmx42y3hmmhjbv3g979ncfza228b2ki6";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."serde_derive"."1.0.84"."proc_macro2"}" deps)
      (crates."quote"."${deps."serde_derive"."1.0.84"."quote"}" deps)
      (crates."syn"."${deps."serde_derive"."1.0.84"."syn"}" deps)
    ]);
    features = mkFeatures (features."serde_derive"."1.0.84" or {});
  };
  features_.serde_derive."1.0.84" = deps: f: updateFeatures f (rec {
    proc_macro2."${deps.serde_derive."1.0.84".proc_macro2}".default = true;
    quote."${deps.serde_derive."1.0.84".quote}".default = true;
    serde_derive."1.0.84".default = (f.serde_derive."1.0.84".default or true);
    syn = fold recursiveUpdate {} [
      { "${deps.serde_derive."1.0.84".syn}"."visit" = true; }
      { "${deps.serde_derive."1.0.84".syn}".default = true; }
    ];
  }) [
    (features_.proc_macro2."${deps."serde_derive"."1.0.84"."proc_macro2"}" deps)
    (features_.quote."${deps."serde_derive"."1.0.84"."quote"}" deps)
    (features_.syn."${deps."serde_derive"."1.0.84"."syn"}" deps)
  ];


# end
# serde_json-1.0.34

  crates.serde_json."1.0.34" = deps: { features?(features_.serde_json."1.0.34" deps {}) }: buildRustCrate {
    crateName = "serde_json";
    version = "1.0.34";
    description = "A JSON serialization file format";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1hj96vsyfc1kzni1j7dkx818pjxadj72c9d2nq8i6vhzg37h3j8f";
    dependencies = mapFeatures features ([
      (crates."itoa"."${deps."serde_json"."1.0.34"."itoa"}" deps)
      (crates."ryu"."${deps."serde_json"."1.0.34"."ryu"}" deps)
      (crates."serde"."${deps."serde_json"."1.0.34"."serde"}" deps)
    ]);
    features = mkFeatures (features."serde_json"."1.0.34" or {});
  };
  features_.serde_json."1.0.34" = deps: f: updateFeatures f (rec {
    itoa."${deps.serde_json."1.0.34".itoa}".default = true;
    ryu."${deps.serde_json."1.0.34".ryu}".default = true;
    serde."${deps.serde_json."1.0.34".serde}".default = true;
    serde_json = fold recursiveUpdate {} [
      { "1.0.34"."indexmap" =
        (f.serde_json."1.0.34"."indexmap" or false) ||
        (f.serde_json."1.0.34".preserve_order or false) ||
        (serde_json."1.0.34"."preserve_order" or false); }
      { "1.0.34".default = (f.serde_json."1.0.34".default or true); }
    ];
  }) [
    (features_.itoa."${deps."serde_json"."1.0.34"."itoa"}" deps)
    (features_.ryu."${deps."serde_json"."1.0.34"."ryu"}" deps)
    (features_.serde."${deps."serde_json"."1.0.34"."serde"}" deps)
  ];


# end
# syn-0.15.23

  crates.syn."0.15.23" = deps: { features?(features_.syn."0.15.23" deps {}) }: buildRustCrate {
    crateName = "syn";
    version = "0.15.23";
    description = "Parser for Rust source code";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0ybqj4vv16s16lshn464rx24v95yx4s41jq5ir004n62zksz77a1";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."syn"."0.15.23"."proc_macro2"}" deps)
      (crates."unicode_xid"."${deps."syn"."0.15.23"."unicode_xid"}" deps)
    ]
      ++ (if features.syn."0.15.23".quote or false then [ (crates.quote."${deps."syn"."0.15.23".quote}" deps) ] else []));
    features = mkFeatures (features."syn"."0.15.23" or {});
  };
  features_.syn."0.15.23" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.syn."0.15.23".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.syn."0.15.23".proc_macro2}"."proc-macro" or false) ||
        (syn."0.15.23"."proc-macro" or false) ||
        (f."syn"."0.15.23"."proc-macro" or false); }
      { "${deps.syn."0.15.23".proc_macro2}".default = (f.proc_macro2."${deps.syn."0.15.23".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "${deps.syn."0.15.23".quote}"."proc-macro" =
        (f.quote."${deps.syn."0.15.23".quote}"."proc-macro" or false) ||
        (syn."0.15.23"."proc-macro" or false) ||
        (f."syn"."0.15.23"."proc-macro" or false); }
      { "${deps.syn."0.15.23".quote}".default = (f.quote."${deps.syn."0.15.23".quote}".default or false); }
    ];
    syn = fold recursiveUpdate {} [
      { "0.15.23"."clone-impls" =
        (f.syn."0.15.23"."clone-impls" or false) ||
        (f.syn."0.15.23".default or false) ||
        (syn."0.15.23"."default" or false); }
      { "0.15.23"."derive" =
        (f.syn."0.15.23"."derive" or false) ||
        (f.syn."0.15.23".default or false) ||
        (syn."0.15.23"."default" or false); }
      { "0.15.23"."parsing" =
        (f.syn."0.15.23"."parsing" or false) ||
        (f.syn."0.15.23".default or false) ||
        (syn."0.15.23"."default" or false); }
      { "0.15.23"."printing" =
        (f.syn."0.15.23"."printing" or false) ||
        (f.syn."0.15.23".default or false) ||
        (syn."0.15.23"."default" or false); }
      { "0.15.23"."proc-macro" =
        (f.syn."0.15.23"."proc-macro" or false) ||
        (f.syn."0.15.23".default or false) ||
        (syn."0.15.23"."default" or false); }
      { "0.15.23"."quote" =
        (f.syn."0.15.23"."quote" or false) ||
        (f.syn."0.15.23".printing or false) ||
        (syn."0.15.23"."printing" or false); }
      { "0.15.23".default = (f.syn."0.15.23".default or true); }
    ];
    unicode_xid."${deps.syn."0.15.23".unicode_xid}".default = true;
  }) [
    (features_.proc_macro2."${deps."syn"."0.15.23"."proc_macro2"}" deps)
    (features_.quote."${deps."syn"."0.15.23"."quote"}" deps)
    (features_.unicode_xid."${deps."syn"."0.15.23"."unicode_xid"}" deps)
  ];


# end
# sys-info-0.5.6

  crates.sys_info."0.5.6" = deps: { features?(features_.sys_info."0.5.6" deps {}) }: buildRustCrate {
    crateName = "sys-info";
    version = "0.5.6";
    description = "Get system information in Rust.\n\nFor now it supports Linux, Mac OS X and Windows.\n";
    authors = [ "Siyu Wang <FillZpp.pub@gmail.com>" ];
    sha256 = "118ma1x3gnlm5jxxgi0bp8bskka5npnwn4f8m93zncbrbmzic2ff";
    libPath = "lib.rs";
    libName = "sys_info";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."sys_info"."0.5.6"."libc"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."cc"."${deps."sys_info"."0.5.6"."cc"}" deps)
    ]);
  };
  features_.sys_info."0.5.6" = deps: f: updateFeatures f (rec {
    cc."${deps.sys_info."0.5.6".cc}".default = true;
    libc."${deps.sys_info."0.5.6".libc}".default = true;
    sys_info."0.5.6".default = (f.sys_info."0.5.6".default or true);
  }) [
    (features_.libc."${deps."sys_info"."0.5.6"."libc"}" deps)
    (features_.cc."${deps."sys_info"."0.5.6"."cc"}" deps)
  ];


# end
# tempdir-0.3.7

  crates.tempdir."0.3.7" = deps: { features?(features_.tempdir."0.3.7" deps {}) }: buildRustCrate {
    crateName = "tempdir";
    version = "0.3.7";
    description = "A library for managing a temporary directory and deleting all contents when it's\ndropped.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0y53sxybyljrr7lh0x0ysrsa7p7cljmwv9v80acy3rc6n97g67vy";
    dependencies = mapFeatures features ([
      (crates."rand"."${deps."tempdir"."0.3.7"."rand"}" deps)
      (crates."remove_dir_all"."${deps."tempdir"."0.3.7"."remove_dir_all"}" deps)
    ]);
  };
  features_.tempdir."0.3.7" = deps: f: updateFeatures f (rec {
    rand."${deps.tempdir."0.3.7".rand}".default = true;
    remove_dir_all."${deps.tempdir."0.3.7".remove_dir_all}".default = true;
    tempdir."0.3.7".default = (f.tempdir."0.3.7".default or true);
  }) [
    (features_.rand."${deps."tempdir"."0.3.7"."rand"}" deps)
    (features_.remove_dir_all."${deps."tempdir"."0.3.7"."remove_dir_all"}" deps)
  ];


# end
# tempfile-2.2.0

  crates.tempfile."2.2.0" = deps: { features?(features_.tempfile."2.2.0" deps {}) }: buildRustCrate {
    crateName = "tempfile";
    version = "2.2.0";
    description = "Securely create temporary files.";
    authors = [ "Steven Allen <steven@stebalien.com>" ];
    sha256 = "1z3l901ipvi0s0mdppw4lwfa77ydb22rfnf6y9sh0pifj7ah5drf";
    dependencies = mapFeatures features ([
      (crates."rand"."${deps."tempfile"."2.2.0"."rand"}" deps)
    ])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."tempfile"."2.2.0"."redox_syscall"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."tempfile"."2.2.0"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."kernel32_sys"."${deps."tempfile"."2.2.0"."kernel32_sys"}" deps)
      (crates."winapi"."${deps."tempfile"."2.2.0"."winapi"}" deps)
    ]) else []);
  };
  features_.tempfile."2.2.0" = deps: f: updateFeatures f (rec {
    kernel32_sys."${deps.tempfile."2.2.0".kernel32_sys}".default = true;
    libc."${deps.tempfile."2.2.0".libc}".default = true;
    rand."${deps.tempfile."2.2.0".rand}".default = true;
    redox_syscall."${deps.tempfile."2.2.0".redox_syscall}".default = true;
    tempfile."2.2.0".default = (f.tempfile."2.2.0".default or true);
    winapi."${deps.tempfile."2.2.0".winapi}".default = true;
  }) [
    (features_.rand."${deps."tempfile"."2.2.0"."rand"}" deps)
    (features_.redox_syscall."${deps."tempfile"."2.2.0"."redox_syscall"}" deps)
    (features_.libc."${deps."tempfile"."2.2.0"."libc"}" deps)
    (features_.kernel32_sys."${deps."tempfile"."2.2.0"."kernel32_sys"}" deps)
    (features_.winapi."${deps."tempfile"."2.2.0"."winapi"}" deps)
  ];


# end
# thread-id-2.0.0

  crates.thread_id."2.0.0" = deps: { features?(features_.thread_id."2.0.0" deps {}) }: buildRustCrate {
    crateName = "thread-id";
    version = "2.0.0";
    description = "Get a unique thread ID";
    authors = [ "Ruud van Asseldonk <dev@veniogames.com>" ];
    sha256 = "06i3c8ckn97i5rp16civ2vpqbknlkx66dkrl070iw60nawi0kjc3";
    dependencies = mapFeatures features ([
      (crates."kernel32_sys"."${deps."thread_id"."2.0.0"."kernel32_sys"}" deps)
      (crates."libc"."${deps."thread_id"."2.0.0"."libc"}" deps)
    ]);
  };
  features_.thread_id."2.0.0" = deps: f: updateFeatures f (rec {
    kernel32_sys."${deps.thread_id."2.0.0".kernel32_sys}".default = true;
    libc."${deps.thread_id."2.0.0".libc}".default = true;
    thread_id."2.0.0".default = (f.thread_id."2.0.0".default or true);
  }) [
    (features_.kernel32_sys."${deps."thread_id"."2.0.0"."kernel32_sys"}" deps)
    (features_.libc."${deps."thread_id"."2.0.0"."libc"}" deps)
  ];


# end
# thread_local-0.2.7

  crates.thread_local."0.2.7" = deps: { features?(features_.thread_local."0.2.7" deps {}) }: buildRustCrate {
    crateName = "thread_local";
    version = "0.2.7";
    description = "Per-object thread-local storage";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    sha256 = "19p0zrs24rdwjvpi10jig5ms3sxj00pv8shkr9cpddri8cdghqp7";
    dependencies = mapFeatures features ([
      (crates."thread_id"."${deps."thread_local"."0.2.7"."thread_id"}" deps)
    ]);
  };
  features_.thread_local."0.2.7" = deps: f: updateFeatures f (rec {
    thread_id."${deps.thread_local."0.2.7".thread_id}".default = true;
    thread_local."0.2.7".default = (f.thread_local."0.2.7".default or true);
  }) [
    (features_.thread_id."${deps."thread_local"."0.2.7"."thread_id"}" deps)
  ];


# end
# thread_local-0.3.6

  crates.thread_local."0.3.6" = deps: { features?(features_.thread_local."0.3.6" deps {}) }: buildRustCrate {
    crateName = "thread_local";
    version = "0.3.6";
    description = "Per-object thread-local storage";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    sha256 = "02rksdwjmz2pw9bmgbb4c0bgkbq5z6nvg510sq1s6y2j1gam0c7i";
    dependencies = mapFeatures features ([
      (crates."lazy_static"."${deps."thread_local"."0.3.6"."lazy_static"}" deps)
    ]);
  };
  features_.thread_local."0.3.6" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.thread_local."0.3.6".lazy_static}".default = true;
    thread_local."0.3.6".default = (f.thread_local."0.3.6".default or true);
  }) [
    (features_.lazy_static."${deps."thread_local"."0.3.6"."lazy_static"}" deps)
  ];


# end
# time-0.1.41

  crates.time."0.1.41" = deps: { features?(features_.time."0.1.41" deps {}) }: buildRustCrate {
    crateName = "time";
    version = "0.1.41";
    description = "Utilities for working with time-related functions in Rust.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0k56037d04zwy1zdznjcyq0q86r84903ixb41xk5sbwmia1k5gqg";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."time"."0.1.41"."libc"}" deps)
    ])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."time"."0.1.41"."redox_syscall"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."time"."0.1.41"."winapi"}" deps)
    ]) else []);
  };
  features_.time."0.1.41" = deps: f: updateFeatures f (rec {
    libc."${deps.time."0.1.41".libc}".default = true;
    redox_syscall."${deps.time."0.1.41".redox_syscall}".default = true;
    time."0.1.41".default = (f.time."0.1.41".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.time."0.1.41".winapi}"."minwinbase" = true; }
      { "${deps.time."0.1.41".winapi}"."minwindef" = true; }
      { "${deps.time."0.1.41".winapi}"."ntdef" = true; }
      { "${deps.time."0.1.41".winapi}"."profileapi" = true; }
      { "${deps.time."0.1.41".winapi}"."std" = true; }
      { "${deps.time."0.1.41".winapi}"."sysinfoapi" = true; }
      { "${deps.time."0.1.41".winapi}"."timezoneapi" = true; }
      { "${deps.time."0.1.41".winapi}".default = true; }
    ];
  }) [
    (features_.libc."${deps."time"."0.1.41"."libc"}" deps)
    (features_.redox_syscall."${deps."time"."0.1.41"."redox_syscall"}" deps)
    (features_.winapi."${deps."time"."0.1.41"."winapi"}" deps)
  ];


# end
# traitobject-0.1.0

  crates.traitobject."0.1.0" = deps: { features?(features_.traitobject."0.1.0" deps {}) }: buildRustCrate {
    crateName = "traitobject";
    version = "0.1.0";
    description = "Unsafe helpers for working with raw trait objects.";
    authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
    sha256 = "10hi8pl361l539g4kg74mcrhn7grmwlar4jl528ddn2z2jvb7lw3";
  };
  features_.traitobject."0.1.0" = deps: f: updateFeatures f (rec {
    traitobject."0.1.0".default = (f.traitobject."0.1.0".default or true);
  }) [];


# end
# typeable-0.1.2

  crates.typeable."0.1.2" = deps: { features?(features_.typeable."0.1.2" deps {}) }: buildRustCrate {
    crateName = "typeable";
    version = "0.1.2";
    description = "Exposes Typeable, for getting TypeIds at runtime.";
    authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
    sha256 = "0lvff10hwyy852m6r11msyv1rpgpnapn284i8dk0p0q5saqvbvnx";
  };
  features_.typeable."0.1.2" = deps: f: updateFeatures f (rec {
    typeable."0.1.2".default = (f.typeable."0.1.2".default or true);
  }) [];


# end
# ucd-util-0.1.3

  crates.ucd_util."0.1.3" = deps: { features?(features_.ucd_util."0.1.3" deps {}) }: buildRustCrate {
    crateName = "ucd-util";
    version = "0.1.3";
    description = "A small utility library for working with the Unicode character database.\n";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "1n1qi3jywq5syq90z9qd8qzbn58pcjgv1sx4sdmipm4jf9zanz15";
  };
  features_.ucd_util."0.1.3" = deps: f: updateFeatures f (rec {
    ucd_util."0.1.3".default = (f.ucd_util."0.1.3".default or true);
  }) [];


# end
# unicase-1.4.2

  crates.unicase."1.4.2" = deps: { features?(features_.unicase."1.4.2" deps {}) }: buildRustCrate {
    crateName = "unicase";
    version = "1.4.2";
    description = "A case-insensitive wrapper around strings.";
    authors = [ "Sean McArthur <sean.monstar@gmail.com>" ];
    sha256 = "0rbnhw2mnhcwrij3vczp0sl8zdfmvf2dlh8hly81kj7132kfj0mf";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);

    buildDependencies = mapFeatures features ([
      (crates."version_check"."${deps."unicase"."1.4.2"."version_check"}" deps)
    ]);
    features = mkFeatures (features."unicase"."1.4.2" or {});
  };
  features_.unicase."1.4.2" = deps: f: updateFeatures f (rec {
    unicase = fold recursiveUpdate {} [
      { "1.4.2"."heapsize" =
        (f.unicase."1.4.2"."heapsize" or false) ||
        (f.unicase."1.4.2".heap_size or false) ||
        (unicase."1.4.2"."heap_size" or false); }
      { "1.4.2"."heapsize_plugin" =
        (f.unicase."1.4.2"."heapsize_plugin" or false) ||
        (f.unicase."1.4.2".heap_size or false) ||
        (unicase."1.4.2"."heap_size" or false); }
      { "1.4.2".default = (f.unicase."1.4.2".default or true); }
    ];
    version_check."${deps.unicase."1.4.2".version_check}".default = true;
  }) [
    (features_.version_check."${deps."unicase"."1.4.2"."version_check"}" deps)
  ];


# end
# unicode-bidi-0.3.4

  crates.unicode_bidi."0.3.4" = deps: { features?(features_.unicode_bidi."0.3.4" deps {}) }: buildRustCrate {
    crateName = "unicode-bidi";
    version = "0.3.4";
    description = "Implementation of the Unicode Bidirectional Algorithm";
    authors = [ "The Servo Project Developers" ];
    sha256 = "0lcd6jasrf8p9p0q20qyf10c6xhvw40m2c4rr105hbk6zy26nj1q";
    libName = "unicode_bidi";
    dependencies = mapFeatures features ([
      (crates."matches"."${deps."unicode_bidi"."0.3.4"."matches"}" deps)
    ]);
    features = mkFeatures (features."unicode_bidi"."0.3.4" or {});
  };
  features_.unicode_bidi."0.3.4" = deps: f: updateFeatures f (rec {
    matches."${deps.unicode_bidi."0.3.4".matches}".default = true;
    unicode_bidi = fold recursiveUpdate {} [
      { "0.3.4"."flame" =
        (f.unicode_bidi."0.3.4"."flame" or false) ||
        (f.unicode_bidi."0.3.4".flame_it or false) ||
        (unicode_bidi."0.3.4"."flame_it" or false); }
      { "0.3.4"."flamer" =
        (f.unicode_bidi."0.3.4"."flamer" or false) ||
        (f.unicode_bidi."0.3.4".flame_it or false) ||
        (unicode_bidi."0.3.4"."flame_it" or false); }
      { "0.3.4"."serde" =
        (f.unicode_bidi."0.3.4"."serde" or false) ||
        (f.unicode_bidi."0.3.4".with_serde or false) ||
        (unicode_bidi."0.3.4"."with_serde" or false); }
      { "0.3.4".default = (f.unicode_bidi."0.3.4".default or true); }
    ];
  }) [
    (features_.matches."${deps."unicode_bidi"."0.3.4"."matches"}" deps)
  ];


# end
# unicode-normalization-0.1.7

  crates.unicode_normalization."0.1.7" = deps: { features?(features_.unicode_normalization."0.1.7" deps {}) }: buildRustCrate {
    crateName = "unicode-normalization";
    version = "0.1.7";
    description = "This crate provides functions for normalization of\nUnicode strings, including Canonical and Compatible\nDecomposition and Recomposition, as described in\nUnicode Standard Annex #15.\n";
    authors = [ "kwantam <kwantam@gmail.com>" ];
    sha256 = "1da2hv800pd0wilmn4idwpgv5p510hjxizjcfv6xzb40xcsjd8gs";
  };
  features_.unicode_normalization."0.1.7" = deps: f: updateFeatures f (rec {
    unicode_normalization."0.1.7".default = (f.unicode_normalization."0.1.7".default or true);
  }) [];


# end
# unicode-xid-0.1.0

  crates.unicode_xid."0.1.0" = deps: { features?(features_.unicode_xid."0.1.0" deps {}) }: buildRustCrate {
    crateName = "unicode-xid";
    version = "0.1.0";
    description = "Determine whether characters have the XID_Start\nor XID_Continue properties according to\nUnicode Standard Annex #31.\n";
    authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
    sha256 = "05wdmwlfzxhq3nhsxn6wx4q8dhxzzfb9szsz6wiw092m1rjj01zj";
    features = mkFeatures (features."unicode_xid"."0.1.0" or {});
  };
  features_.unicode_xid."0.1.0" = deps: f: updateFeatures f (rec {
    unicode_xid."0.1.0".default = (f.unicode_xid."0.1.0".default or true);
  }) [];


# end
# url-1.7.2

  crates.url."1.7.2" = deps: { features?(features_.url."1.7.2" deps {}) }: buildRustCrate {
    crateName = "url";
    version = "1.7.2";
    description = "URL library for Rust, based on the WHATWG URL Standard";
    authors = [ "The rust-url developers" ];
    sha256 = "0qzrjzd9r1niv7037x4cgnv98fs1vj0k18lpxx890ipc47x5gc09";
    dependencies = mapFeatures features ([
      (crates."idna"."${deps."url"."1.7.2"."idna"}" deps)
      (crates."matches"."${deps."url"."1.7.2"."matches"}" deps)
      (crates."percent_encoding"."${deps."url"."1.7.2"."percent_encoding"}" deps)
    ]);
    features = mkFeatures (features."url"."1.7.2" or {});
  };
  features_.url."1.7.2" = deps: f: updateFeatures f (rec {
    idna."${deps.url."1.7.2".idna}".default = true;
    matches."${deps.url."1.7.2".matches}".default = true;
    percent_encoding."${deps.url."1.7.2".percent_encoding}".default = true;
    url = fold recursiveUpdate {} [
      { "1.7.2"."encoding" =
        (f.url."1.7.2"."encoding" or false) ||
        (f.url."1.7.2".query_encoding or false) ||
        (url."1.7.2"."query_encoding" or false); }
      { "1.7.2"."heapsize" =
        (f.url."1.7.2"."heapsize" or false) ||
        (f.url."1.7.2".heap_size or false) ||
        (url."1.7.2"."heap_size" or false); }
      { "1.7.2".default = (f.url."1.7.2".default or true); }
    ];
  }) [
    (features_.idna."${deps."url"."1.7.2"."idna"}" deps)
    (features_.matches."${deps."url"."1.7.2"."matches"}" deps)
    (features_.percent_encoding."${deps."url"."1.7.2"."percent_encoding"}" deps)
  ];


# end
# utf8-ranges-0.1.3

  crates.utf8_ranges."0.1.3" = deps: { features?(features_.utf8_ranges."0.1.3" deps {}) }: buildRustCrate {
    crateName = "utf8-ranges";
    version = "0.1.3";
    description = "Convert ranges of Unicode codepoints to UTF-8 byte ranges.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "1cj548a91a93j8375p78qikaiam548xh84cb0ck8y119adbmsvbp";
  };
  features_.utf8_ranges."0.1.3" = deps: f: updateFeatures f (rec {
    utf8_ranges."0.1.3".default = (f.utf8_ranges."0.1.3".default or true);
  }) [];


# end
# utf8-ranges-1.0.2

  crates.utf8_ranges."1.0.2" = deps: { features?(features_.utf8_ranges."1.0.2" deps {}) }: buildRustCrate {
    crateName = "utf8-ranges";
    version = "1.0.2";
    description = "Convert ranges of Unicode codepoints to UTF-8 byte ranges.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "1my02laqsgnd8ib4dvjgd4rilprqjad6pb9jj9vi67csi5qs2281";
  };
  features_.utf8_ranges."1.0.2" = deps: f: updateFeatures f (rec {
    utf8_ranges."1.0.2".default = (f.utf8_ranges."1.0.2".default or true);
  }) [];


# end
# uuid-0.4.0

  crates.uuid."0.4.0" = deps: { features?(features_.uuid."0.4.0" deps {}) }: buildRustCrate {
    crateName = "uuid";
    version = "0.4.0";
    description = "A library to generate and parse UUIDs.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1fzgp3ayz7g5zx4gvxkxxm9jkrllj5qlvlyp6r2an0wyqm2y9qlh";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.uuid."0.4.0".rand or false then [ (crates.rand."${deps."uuid"."0.4.0".rand}" deps) ] else []));
    features = mkFeatures (features."uuid"."0.4.0" or {});
  };
  features_.uuid."0.4.0" = deps: f: updateFeatures f (rec {
    rand."${deps.uuid."0.4.0".rand}".default = true;
    uuid = fold recursiveUpdate {} [
      { "0.4.0"."rand" =
        (f.uuid."0.4.0"."rand" or false) ||
        (f.uuid."0.4.0".v4 or false) ||
        (uuid."0.4.0"."v4" or false); }
      { "0.4.0"."sha1" =
        (f.uuid."0.4.0"."sha1" or false) ||
        (f.uuid."0.4.0".v5 or false) ||
        (uuid."0.4.0"."v5" or false); }
      { "0.4.0".default = (f.uuid."0.4.0".default or true); }
    ];
  }) [
    (features_.rand."${deps."uuid"."0.4.0"."rand"}" deps)
  ];


# end
# vcpkg-0.2.6

  crates.vcpkg."0.2.6" = deps: { features?(features_.vcpkg."0.2.6" deps {}) }: buildRustCrate {
    crateName = "vcpkg";
    version = "0.2.6";
    description = "A library to find native dependencies in a vcpkg tree at build\ntime in order to be used in Cargo build scripts.\n";
    authors = [ "Jim McGrath <jimmc2@gmail.com>" ];
    sha256 = "1ig6jqpzzl1z9vk4qywgpfr4hfbd8ny8frqsgm3r449wkc4n1i5x";
  };
  features_.vcpkg."0.2.6" = deps: f: updateFeatures f (rec {
    vcpkg."0.2.6".default = (f.vcpkg."0.2.6".default or true);
  }) [];


# end
# version_check-0.1.5

  crates.version_check."0.1.5" = deps: { features?(features_.version_check."0.1.5" deps {}) }: buildRustCrate {
    crateName = "version_check";
    version = "0.1.5";
    description = "Tiny crate to check the version of the installed/running rustc.";
    authors = [ "Sergio Benitez <sb@sergio.bz>" ];
    sha256 = "1yrx9xblmwbafw2firxyqbj8f771kkzfd24n3q7xgwiqyhi0y8qd";
  };
  features_.version_check."0.1.5" = deps: f: updateFeatures f (rec {
    version_check."0.1.5".default = (f.version_check."0.1.5".default or true);
  }) [];


# end
# winapi-0.2.8

  crates.winapi."0.2.8" = deps: { features?(features_.winapi."0.2.8" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.2.8";
    description = "Types and constants for WinAPI bindings. See README for list of crates providing function bindings.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0a45b58ywf12vb7gvj6h3j264nydynmzyqz8d8rqxsj6icqv82as";
  };
  features_.winapi."0.2.8" = deps: f: updateFeatures f (rec {
    winapi."0.2.8".default = (f.winapi."0.2.8".default or true);
  }) [];


# end
# winapi-0.3.6

  crates.winapi."0.3.6" = deps: { features?(features_.winapi."0.3.6" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.3.6";
    description = "Raw FFI bindings for all of Windows API.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1d9jfp4cjd82sr1q4dgdlrkvm33zhhav9d7ihr0nivqbncr059m4";
    build = "build.rs";
    dependencies = (if kernel == "i686-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_i686_pc_windows_gnu"."${deps."winapi"."0.3.6"."winapi_i686_pc_windows_gnu"}" deps)
    ]) else [])
      ++ (if kernel == "x86_64-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_x86_64_pc_windows_gnu"."${deps."winapi"."0.3.6"."winapi_x86_64_pc_windows_gnu"}" deps)
    ]) else []);
    features = mkFeatures (features."winapi"."0.3.6" or {});
  };
  features_.winapi."0.3.6" = deps: f: updateFeatures f (rec {
    winapi."0.3.6".default = (f.winapi."0.3.6".default or true);
    winapi_i686_pc_windows_gnu."${deps.winapi."0.3.6".winapi_i686_pc_windows_gnu}".default = true;
    winapi_x86_64_pc_windows_gnu."${deps.winapi."0.3.6".winapi_x86_64_pc_windows_gnu}".default = true;
  }) [
    (features_.winapi_i686_pc_windows_gnu."${deps."winapi"."0.3.6"."winapi_i686_pc_windows_gnu"}" deps)
    (features_.winapi_x86_64_pc_windows_gnu."${deps."winapi"."0.3.6"."winapi_x86_64_pc_windows_gnu"}" deps)
  ];


# end
# winapi-build-0.1.1

  crates.winapi_build."0.1.1" = deps: { features?(features_.winapi_build."0.1.1" deps {}) }: buildRustCrate {
    crateName = "winapi-build";
    version = "0.1.1";
    description = "Common code for build.rs in WinAPI -sys crates.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1lxlpi87rkhxcwp2ykf1ldw3p108hwm24nywf3jfrvmff4rjhqga";
    libName = "build";
  };
  features_.winapi_build."0.1.1" = deps: f: updateFeatures f (rec {
    winapi_build."0.1.1".default = (f.winapi_build."0.1.1".default or true);
  }) [];


# end
# winapi-i686-pc-windows-gnu-0.4.0

  crates.winapi_i686_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_i686_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    description = "Import libraries for the i686-pc-windows-gnu target. Please don't use this crate directly, depend on winapi instead.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "05ihkij18r4gamjpxj4gra24514can762imjzlmak5wlzidplzrp";
    build = "build.rs";
  };
  features_.winapi_i686_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_i686_pc_windows_gnu."0.4.0".default = (f.winapi_i686_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
# winapi-x86_64-pc-windows-gnu-0.4.0

  crates.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_x86_64_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    description = "Import libraries for the x86_64-pc-windows-gnu target. Please don't use this crate directly, depend on winapi instead.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0n1ylmlsb8yg1v583i4xy0qmqg42275flvbc51hdqjjfjcl9vlbj";
    build = "build.rs";
  };
  features_.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_x86_64_pc_windows_gnu."0.4.0".default = (f.winapi_x86_64_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
}
