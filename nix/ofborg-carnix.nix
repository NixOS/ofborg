{ buildPlatform, buildRustCrate, fetchgit, ... }:
let kernel = buildPlatform.parsed.kernel.name;
    abi = buildPlatform.parsed.abi.name;
    advapi32_sys_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "advapi32-sys";
      version = "0.2.0";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "1l6789hkz2whd9gklwz1m379kcvyizaj8nnzj3rn4a5h79yg59v7";
      libName = "advapi32";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    aho_corasick_0_5_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "aho-corasick";
      version = "0.5.3";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "1igab46mvgknga3sxkqc917yfff0wsjxjzabdigmh240p5qxqlnn";
      libName = "aho_corasick";
      crateBin = [ {  name = "aho-corasick-dot"; } ];
      inherit dependencies buildDependencies features;
    };
    aho_corasick_0_6_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "aho-corasick";
      version = "0.6.3";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "1cpqzf6acj8lm06z3f1cg41wn6c2n9l3v49nh0dvimv4055qib6k";
      libName = "aho_corasick";
      crateBin = [ {  name = "aho-corasick-dot"; } ];
      inherit dependencies buildDependencies features;
    };
    amq_proto_0_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "amq-proto";
      version = "0.1.0";
      authors = [ "Andrii Dmytrenko <refresh.xss@gmail.com>" ];
      sha256 = "0333fsph61q9nxbx6h8hdxjmpabjm9vmsfc6q5agy801x35r4ml9";
      inherit dependencies buildDependencies features;
    };
    amqp_0_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "amqp";
      version = "0.1.0";
      authors = [ "Andrii Dmytrenko <andrey@reevoo.com>" ];
      src = fetchgit {
         url = "https://github.com/grahamc/rust-amqp.git";
         rev = "1216885c84f7c94a205a8e41519684e7df0e0f35";
         sha256 = "0xmdhi8xiphrahs0mfjfamsxqglbzcxgm5h2xqhmlrbn5n1d479p";
      };
      inherit dependencies buildDependencies features;
    };
    antidote_1_0_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "antidote";
      version = "1.0.0";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "1x2wgaw603jcjwsfvc8s2rpaqjv0aqj8mvws2ahhkvfnwkdf7icw";
      inherit dependencies buildDependencies features;
    };
    backtrace_0_3_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "backtrace";
      version = "0.3.3";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" "The Rust Project Developers" ];
      sha256 = "0invfdxkj85v8zyrjs3amfxjdk2a36x8irq7wq7kny6q49hh8y0z";
      inherit dependencies buildDependencies features;
    };
    backtrace_sys_0_1_16_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "backtrace-sys";
      version = "0.1.16";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
      sha256 = "1cn2c8q3dn06crmnk0p62czkngam4l8nf57wy33nz1y5g25pszwy";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    base64_0_6_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "base64";
      version = "0.6.0";
      authors = [ "Alice Maz <alice@alicemaz.com>" "Marshall Pierce <marshall@mpierce.org>" ];
      sha256 = "0ql1rmczbnww3iszc0pfc6mqa47ravpsdf525vp6s8r32nyzspl5";
      inherit dependencies buildDependencies features;
    };
    bit_vec_0_4_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "bit-vec";
      version = "0.4.4";
      authors = [ "Alexis Beingessner <a.beingessner@gmail.com>" ];
      sha256 = "06czykmn001z6c3a4nsrpc3lrj63ga0kzp7kgva9r9wylhkkqpq9";
      inherit dependencies buildDependencies features;
    };
    bitflags_0_7_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "bitflags";
      version = "0.7.0";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1hr72xg5slm0z4pxs2hiy4wcyx3jva70h58b7mid8l0a4c8f7gn5";
      inherit dependencies buildDependencies features;
    };
    bitflags_0_9_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "bitflags";
      version = "0.9.1";
      authors = [ "The Rust Project Developers" ];
      sha256 = "18h073l5jd88rx4qdr95fjddr9rk79pb1aqnshzdnw16cfmb9rws";
      inherit dependencies buildDependencies features;
    };
    byteorder_0_5_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "byteorder";
      version = "0.5.3";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "0zsr6b0m0yl5c0yy92nq7srfpczd1dx1xqcx3rlm5fbl8si9clqx";
      inherit dependencies buildDependencies features;
    };
    byteorder_1_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "byteorder";
      version = "1.1.0";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "1i2n0161jm00zvzh4bncgv9zrwa6ydbxdn5j4bx0wwn7rvi9zycp";
      inherit dependencies buildDependencies features;
    };
    cc_1_0_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "cc";
      version = "1.0.3";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
      sha256 = "193pwqgh79w6k0k29svyds5nnlrwx44myqyrw605d5jj4yk2zmpr";
      inherit dependencies buildDependencies features;
    };
    cfg_if_0_1_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "cfg-if";
      version = "0.1.2";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
      sha256 = "0x06hvrrqy96m97593823vvxcgvjaxckghwyy2jcyc8qc7c6cyhi";
      inherit dependencies buildDependencies features;
    };
    core_foundation_0_2_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "core-foundation";
      version = "0.2.3";
      authors = [ "The Servo Project Developers" ];
      sha256 = "1g0vpya5h2wa0nlz4a74jar6y8z09f0p76zbzfqrm3dbfsrld1pm";
      inherit dependencies buildDependencies features;
    };
    core_foundation_sys_0_2_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "core-foundation-sys";
      version = "0.2.3";
      authors = [ "The Servo Project Developers" ];
      sha256 = "19s0d03294m9s5j8cvy345db3gkhs2y02j5268ap0c6ky5apl53s";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    crypt32_sys_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "crypt32-sys";
      version = "0.2.0";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "1vy1q3ayc7f4wiwyxw31hd12cvs7791x3by6ka9wbxhm5gzfs3d0";
      libName = "crypt32";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    dbghelp_sys_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "dbghelp-sys";
      version = "0.2.0";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "0ylpi3bbiy233m57hnisn1df1v0lbl7nsxn34b0anzsgg440hqpq";
      libName = "dbghelp";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    dtoa_0_4_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "dtoa";
      version = "0.4.2";
      authors = [ "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1bxsh6fags7nr36vlz07ik2a1rzyipc8x1y30kjk832hf2pzadmw";
      inherit dependencies buildDependencies features;
    };
    enum_primitive_0_1_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "enum_primitive";
      version = "0.1.1";
      authors = [ "Anders Kaseorg <andersk@mit.edu>" ];
      sha256 = "1a225rlsz7sz3nn14dar71kp2f9v08s3rwl6j55xp51mv01f695y";
      inherit dependencies buildDependencies features;
    };
    env_logger_0_3_5_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "env_logger";
      version = "0.3.5";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1mvxiaaqsyjliv1mm1qaagjqiccw11mdyi3n9h9rf8y6wj15zycw";
      inherit dependencies buildDependencies features;
    };
    env_logger_0_4_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "env_logger";
      version = "0.4.3";
      authors = [ "The Rust Project Developers" ];
      sha256 = "0nrx04p4xa86d5kc7aq4fwvipbqji9cmgy449h47nc9f1chafhgg";
      inherit dependencies buildDependencies features;
    };
    error_chain_0_10_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "error-chain";
      version = "0.10.0";
      authors = [ "Brian Anderson <banderson@mozilla.com>" "Paul Colomiets <paul@colomiets.name>" "Colin Kiegel <kiegel@gmx.de>" "Yamakaky <yamakaky@yamaworld.fr>" ];
      sha256 = "1xxbzd8cjlpzsb9fsih7mdnndhzrvykj0w77yg90qc85az1xwy5z";
      inherit dependencies buildDependencies features;
    };
    foreign_types_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "foreign-types";
      version = "0.2.0";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "1sznwg2py4xi7hyrx0gg1sirlwgh87wsanvjx3zb475g6c4139jh";
      inherit dependencies buildDependencies features;
    };
    fs2_0_4_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "fs2";
      version = "0.4.2";
      authors = [ "Dan Burkert <dan@danburkert.com>" ];
      sha256 = "034s52pmqvrkafmmlnklysqx6gl08rl63ycngbav9hs0mrq22qvf";
      inherit dependencies buildDependencies features;
    };
    fuchsia_zircon_0_2_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "fuchsia-zircon";
      version = "0.2.1";
      authors = [ "Raph Levien <raph@google.com>" ];
      sha256 = "0yd4rd7ql1vdr349p6vgq2dnwmpylky1kjp8g1zgvp250jxrhddb";
      inherit dependencies buildDependencies features;
    };
    fuchsia_zircon_sys_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "fuchsia-zircon-sys";
      version = "0.2.0";
      authors = [ "Raph Levien <raph@google.com>" ];
      sha256 = "1yrqsrjwlhl3di6prxf5xmyd82gyjaysldbka5wwk83z11mpqh4w";
      inherit dependencies buildDependencies features;
    };
    httparse_1_2_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "httparse";
      version = "1.2.3";
      authors = [ "Sean McArthur <sean.monstar@gmail.com>" ];
      sha256 = "13x17y9bip0bija06y4vwpgh8jdmdi2gsvjq02kyfy0fbp5cqa93";
      inherit dependencies buildDependencies features;
    };
    hubcaps_0_3_16_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "hubcaps";
      version = "0.3.16";
      authors = [ "softprops <d.tangren@gmail.com>" ];
      src = fetchgit {
         url = "https://github.com/grahamc/hubcaps.git";
         rev = "3ddb700e6b51d7ffc2edd3b001987b4fa124d0e2";
         sha256 = "1ivh9jjcjnbm5fsbr0w4wa1wmka6hsq0zjh148f9hs3q93hspr71";
      };
      inherit dependencies buildDependencies features;
    };
    hyper_0_10_13_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "hyper";
      version = "0.10.13";
      authors = [ "Sean McArthur <sean.monstar@gmail.com>" "Jonathan Reem <jonathan.reem@gmail.com>" ];
      sha256 = "1ps970916ciphcx3zrqklfay1488ky6yk7kr8kvnr363v6w9wfp5";
      inherit dependencies buildDependencies features;
    };
    hyper_native_tls_0_2_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "hyper-native-tls";
      version = "0.2.4";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "1niqi1z1a3xfb9qaawy3fzrgaf8qwr925fqjswlrdjczq176f1iy";
      inherit dependencies buildDependencies features;
    };
    idna_0_1_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "idna";
      version = "0.1.4";
      authors = [ "The rust-url developers" ];
      sha256 = "15j44qgjx1skwg9i7f4cm36ni4n99b1ayx23yxx7axxcw8vjf336";
      inherit dependencies buildDependencies features;
    };
    itoa_0_3_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "itoa";
      version = "0.3.4";
      authors = [ "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1nfkzz6vrgj0d9l3yzjkkkqzdgs68y294fjdbl7jq118qi8xc9d9";
      inherit dependencies buildDependencies features;
    };
    kernel32_sys_0_2_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "kernel32-sys";
      version = "0.2.2";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "1lrw1hbinyvr6cp28g60z97w32w8vsk6pahk64pmrv2fmby8srfj";
      libName = "kernel32";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    language_tags_0_2_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "language-tags";
      version = "0.2.2";
      authors = [ "Pyfisch <pyfisch@gmail.com>" ];
      sha256 = "1zkrdzsqzzc7509kd7nngdwrp461glm2g09kqpzaqksp82frjdvy";
      inherit dependencies buildDependencies features;
    };
    lazy_static_0_2_9_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "lazy_static";
      version = "0.2.9";
      authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
      sha256 = "08ldzr5292y3hvi6l6v8l4i6v95lm1aysmnfln65h10sqrfh6iw7";
      inherit dependencies buildDependencies features;
    };
    libc_0_2_33_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "libc";
      version = "0.2.33";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1l7synziccnvarsq2kk22vps720ih6chmn016bhr2bq54hblbnl1";
      inherit dependencies buildDependencies features;
    };
    log_0_3_8_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "log";
      version = "0.3.8";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1c43z4z85sxrsgir4s1hi84558ab5ic7jrn5qgmsiqcv90vvn006";
      inherit dependencies buildDependencies features;
    };
    matches_0_1_6_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "matches";
      version = "0.1.6";
      authors = [ "Simon Sapin <simon.sapin@exyr.org>" ];
      sha256 = "1zlrqlbvzxdil8z8ial2ihvxjwvlvg3g8dr0lcdpsjclkclasjan";
      libPath = "lib.rs";
      inherit dependencies buildDependencies features;
    };
    md5_0_3_6_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "md5";
      version = "0.3.6";
      authors = [ "Ivan Ukhov <ivan.ukhov@gmail.com>" "Konstantin Stepanov <milezv@gmail.com>" "Lukas Kalbertodt <lukas.kalbertodt@gmail.com>" "Nathan Musoke <nathan.musoke@gmail.com>" "Tony Arcieri <bascule@gmail.com>" ];
      sha256 = "0q6lxmjqxc6vcsyyaggank89bw8g64spw29hl5yvn8l0f4an03nd";
      inherit dependencies buildDependencies features;
    };
    memchr_0_1_11_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "memchr";
      version = "0.1.11";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
      sha256 = "0x73jghamvxxq5fsw9wb0shk5m6qp3q6fsf0nibn0i6bbqkw91s8";
      inherit dependencies buildDependencies features;
    };
    memchr_1_0_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "memchr";
      version = "1.0.2";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
      sha256 = "0dfb8ifl9nrc9kzgd5z91q6qg87sh285q1ih7xgrsglmqfav9lg7";
      inherit dependencies buildDependencies features;
    };
    mime_0_2_6_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "mime";
      version = "0.2.6";
      authors = [ "Sean McArthur <sean.monstar@gmail.com>" ];
      sha256 = "1skwwa0j3kqd8rm9387zgabjhp07zj99q71nzlhba4lrz9r911b3";
      inherit dependencies buildDependencies features;
    };
    native_tls_0_1_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "native-tls";
      version = "0.1.4";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "0q5y5i96mfpjbhx8y7w9rdq65mksw67m60bw4xqlybc8y6jkr99v";
      inherit dependencies buildDependencies features;
    };
    num_traits_0_1_40_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "num-traits";
      version = "0.1.40";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1fr8ghp4i97q3agki54i0hpmqxv3s65i2mqd1pinc7w7arc3fplw";
      inherit dependencies buildDependencies features;
    };
    num_cpus_1_7_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "num_cpus";
      version = "1.7.0";
      authors = [ "Sean McArthur <sean@seanmonstar.com>" ];
      sha256 = "0231xmd65ma3pqfiw8pkv9dvm9x708z4xlrwp3i0sgiwv408dz3f";
      inherit dependencies buildDependencies features;
    };
    ofborg_0_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "ofborg";
      version = "0.1.0";
      authors = [ "Graham Christensen <graham@grahamc.com>" ];
      src = ./../ofborg;
      inherit dependencies buildDependencies features;
    };
    openssl_0_9_20_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "openssl";
      version = "0.9.20";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "0dbj6k6z828c3sqbxidw5zfval29k8dlsr8qn8fizhc1alli18gx";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    openssl_sys_0_9_20_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "openssl-sys";
      version = "0.9.20";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "05q6qagvy7lim9vkq2v00vpm34j1dq4xy9pchs7fb6yy803vx24m";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    percent_encoding_1_0_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "percent-encoding";
      version = "1.0.1";
      authors = [ "The rust-url developers" ];
      sha256 = "04ahrp7aw4ip7fmadb0bknybmkfav0kk0gw4ps3ydq5w6hr0ib5i";
      libPath = "lib.rs";
      inherit dependencies buildDependencies features;
    };
    pkg_config_0_3_9_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "pkg-config";
      version = "0.3.9";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
      sha256 = "06k8fxgrsrxj8mjpjcq1n7mn2p1shpxif4zg9y5h09c7vy20s146";
      inherit dependencies buildDependencies features;
    };
    quote_0_3_15_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "quote";
      version = "0.3.15";
      authors = [ "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "09il61jv4kd1360spaj46qwyl21fv1qz18fsv2jra8wdnlgl5jsg";
      inherit dependencies buildDependencies features;
    };
    rand_0_3_18_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "rand";
      version = "0.3.18";
      authors = [ "The Rust Project Developers" ];
      sha256 = "15d7c3myn968dzjs0a2pgv58hzdavxnq6swgj032lw2v966ir4xv";
      inherit dependencies buildDependencies features;
    };
    redox_syscall_0_1_31_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "redox_syscall";
      version = "0.1.31";
      authors = [ "Jeremy Soller <jackpot51@gmail.com>" ];
      sha256 = "0kipd9qslzin4fgj4jrxv6yz5l3l71gnbd7fq1jhk2j7f2sq33j4";
      libName = "syscall";
      inherit dependencies buildDependencies features;
    };
    regex_0_1_80_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "regex";
      version = "0.1.80";
      authors = [ "The Rust Project Developers" ];
      sha256 = "0y4s8ghhx6sgzb35irwivm3w0l2hhqhmdcd2px9hirqnkagal9l6";
      inherit dependencies buildDependencies features;
    };
    regex_0_2_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "regex";
      version = "0.2.2";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1f1zrrynfylg0vcfyfp60bybq4rp5g1yk2k7lc7fyz7mmc7k2qr7";
      inherit dependencies buildDependencies features;
    };
    regex_syntax_0_3_9_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "regex-syntax";
      version = "0.3.9";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1mzhphkbwppwd1zam2jkgjk550cqgf6506i87bw2yzrvcsraiw7m";
      inherit dependencies buildDependencies features;
    };
    regex_syntax_0_4_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "regex-syntax";
      version = "0.4.1";
      authors = [ "The Rust Project Developers" ];
      sha256 = "01yrsm68lj86ad1whgg1z95c2pfsvv58fz8qjcgw7mlszc0c08ls";
      inherit dependencies buildDependencies features;
    };
    rustc_demangle_0_1_5_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "rustc-demangle";
      version = "0.1.5";
      authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
      sha256 = "096kkcx9j747700fhxj1s4rlwkj21pqjmvj64psdj6bakb2q13nc";
      inherit dependencies buildDependencies features;
    };
    safemem_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "safemem";
      version = "0.2.0";
      authors = [ "Austin Bonander <austin.bonander@gmail.com>" ];
      sha256 = "058m251q202n479ip1h6s91yw3plg66vsk5mpaflssn6rs5hijdm";
      inherit dependencies buildDependencies features;
    };
    schannel_0_1_8_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "schannel";
      version = "0.1.8";
      authors = [ "Steven Fackler <sfackler@gmail.com>" "Steffen Butzer <steffen.butzer@outlook.com>" ];
      sha256 = "01vgljs175gl2rdjdnys5da2lv98xfl3ir1csvpw4hgv3xirhx3q";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    secur32_sys_0_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "secur32-sys";
      version = "0.2.0";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "0sp46ix9mx1156bidpfiq30xxsgmpva5jffls3259kxjqlxifcnx";
      libName = "secur32";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    security_framework_0_1_16_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "security-framework";
      version = "0.1.16";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "1kxczsaj8gz4922jl5af2gkxh71rasb6khaf3dp7ldlnw9qf2sbm";
      inherit dependencies buildDependencies features;
    };
    security_framework_sys_0_1_16_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "security-framework-sys";
      version = "0.1.16";
      authors = [ "Steven Fackler <sfackler@gmail.com>" ];
      sha256 = "0ai2pivdr5fyc7czbkpcrwap0imyy0r8ndarrl3n5kiv0jha1js3";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    serde_1_0_19_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "serde";
      version = "1.0.19";
      authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "0dfhkkbrpr0vr1b2hhbddizb8bq4phi5ck0jhy3yx31bc2byb1l1";
      inherit dependencies buildDependencies features;
    };
    serde_derive_1_0_19_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "serde_derive";
      version = "1.0.19";
      authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1fbr1zi25fgwy49mvpjq8g611mnv3vcd4n0mgca2lfdsp5n2nw5v";
      procMacro = true;
      inherit dependencies buildDependencies features;
    };
    serde_derive_internals_0_17_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "serde_derive_internals";
      version = "0.17.0";
      authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1g1j3v6pj9wbcz3v3w4smjpwrcdwjicmf6yd5cbai04as9iwhw74";
      inherit dependencies buildDependencies features;
    };
    serde_json_1_0_6_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "serde_json";
      version = "1.0.6";
      authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1kacyc59splwbg8gr7qs32pp9smgy1khq0ggnv07yxhs7h355vjz";
      inherit dependencies buildDependencies features;
    };
    syn_0_11_11_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "syn";
      version = "0.11.11";
      authors = [ "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "0yw8ng7x1dn5a6ykg0ib49y7r9nhzgpiq2989rqdp7rdz3n85502";
      inherit dependencies buildDependencies features;
    };
    synom_0_11_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "synom";
      version = "0.11.3";
      authors = [ "David Tolnay <dtolnay@gmail.com>" ];
      sha256 = "1l6d1s9qjfp6ng2s2z8219igvlv7gyk8gby97sdykqc1r93d8rhc";
      inherit dependencies buildDependencies features;
    };
    tempdir_0_3_5_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "tempdir";
      version = "0.3.5";
      authors = [ "The Rust Project Developers" ];
      sha256 = "0rirc5prqppzgd15fm8ayan349lgk2k5iqdkrbwrwrv5pm4znsnz";
      inherit dependencies buildDependencies features;
    };
    tempfile_2_2_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "tempfile";
      version = "2.2.0";
      authors = [ "Steven Allen <steven@stebalien.com>" ];
      sha256 = "1z3l901ipvi0s0mdppw4lwfa77ydb22rfnf6y9sh0pifj7ah5drf";
      inherit dependencies buildDependencies features;
    };
    thread_id_2_0_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "thread-id";
      version = "2.0.0";
      authors = [ "Ruud van Asseldonk <dev@veniogames.com>" ];
      sha256 = "06i3c8ckn97i5rp16civ2vpqbknlkx66dkrl070iw60nawi0kjc3";
      inherit dependencies buildDependencies features;
    };
    thread_local_0_2_7_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "thread_local";
      version = "0.2.7";
      authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
      sha256 = "19p0zrs24rdwjvpi10jig5ms3sxj00pv8shkr9cpddri8cdghqp7";
      inherit dependencies buildDependencies features;
    };
    thread_local_0_3_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "thread_local";
      version = "0.3.4";
      authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
      sha256 = "1y6cwyhhx2nkz4b3dziwhqdvgq830z8wjp32b40pjd8r0hxqv2jr";
      inherit dependencies buildDependencies features;
    };
    time_0_1_38_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "time";
      version = "0.1.38";
      authors = [ "The Rust Project Developers" ];
      sha256 = "1ws283vvz7c6jfiwn53rmc6kybapr4pjaahfxxrz232b0qzw7gcp";
      inherit dependencies buildDependencies features;
    };
    traitobject_0_1_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "traitobject";
      version = "0.1.0";
      authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
      sha256 = "10hi8pl361l539g4kg74mcrhn7grmwlar4jl528ddn2z2jvb7lw3";
      inherit dependencies buildDependencies features;
    };
    typeable_0_1_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "typeable";
      version = "0.1.2";
      authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
      sha256 = "0lvff10hwyy852m6r11msyv1rpgpnapn284i8dk0p0q5saqvbvnx";
      inherit dependencies buildDependencies features;
    };
    unicase_1_4_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "unicase";
      version = "1.4.2";
      authors = [ "Sean McArthur <sean.monstar@gmail.com>" ];
      sha256 = "0rbnhw2mnhcwrij3vczp0sl8zdfmvf2dlh8hly81kj7132kfj0mf";
      build = "build.rs";
      inherit dependencies buildDependencies features;
    };
    unicode_bidi_0_3_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "unicode-bidi";
      version = "0.3.4";
      authors = [ "The Servo Project Developers" ];
      sha256 = "0lcd6jasrf8p9p0q20qyf10c6xhvw40m2c4rr105hbk6zy26nj1q";
      libName = "unicode_bidi";
      inherit dependencies buildDependencies features;
    };
    unicode_normalization_0_1_5_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "unicode-normalization";
      version = "0.1.5";
      authors = [ "kwantam <kwantam@gmail.com>" ];
      sha256 = "0hg29g86fca7b65mwk4sm5s838js6bqrl0gabadbazvbsgjam0j5";
      inherit dependencies buildDependencies features;
    };
    unicode_xid_0_0_4_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "unicode-xid";
      version = "0.0.4";
      authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
      sha256 = "1dc8wkkcd3s6534s5aw4lbjn8m67flkkbnajp5bl8408wdg8rh9v";
      inherit dependencies buildDependencies features;
    };
    unreachable_1_0_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "unreachable";
      version = "1.0.0";
      authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
      sha256 = "1am8czbk5wwr25gbp2zr007744fxjshhdqjz9liz7wl4pnv3whcf";
      inherit dependencies buildDependencies features;
    };
    url_1_6_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "url";
      version = "1.6.0";
      authors = [ "The rust-url developers" ];
      sha256 = "1bvzl4dvjj84h46ai3x23wyafa2wwhchj08vr2brf25dxwc7mg18";
      inherit dependencies buildDependencies features;
    };
    utf8_ranges_0_1_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "utf8-ranges";
      version = "0.1.3";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "1cj548a91a93j8375p78qikaiam548xh84cb0ck8y119adbmsvbp";
      inherit dependencies buildDependencies features;
    };
    utf8_ranges_1_0_0_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "utf8-ranges";
      version = "1.0.0";
      authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
      sha256 = "0rzmqprwjv9yp1n0qqgahgm24872x6c0xddfym5pfndy7a36vkn0";
      inherit dependencies buildDependencies features;
    };
    vcpkg_0_2_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "vcpkg";
      version = "0.2.2";
      authors = [ "Jim McGrath <jimmc2@gmail.com>" ];
      sha256 = "1fl5j0ksnwrnsrf1b1a9lqbjgnajdipq0030vsbhx81mb7d9478a";
      inherit dependencies buildDependencies features;
    };
    version_check_0_1_3_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "version_check";
      version = "0.1.3";
      authors = [ "Sergio Benitez <sb@sergio.bz>" ];
      sha256 = "0z635wdclv9bvafj11fpgndn7y79ibpsnc364pm61i1m4wwg8msg";
      inherit dependencies buildDependencies features;
    };
    void_1_0_2_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "void";
      version = "1.0.2";
      authors = [ "Jonathan Reem <jonathan.reem@gmail.com>" ];
      sha256 = "0h1dm0dx8dhf56a83k68mijyxigqhizpskwxfdrs1drwv2cdclv3";
      inherit dependencies buildDependencies features;
    };
    winapi_0_2_8_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "winapi";
      version = "0.2.8";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "0a45b58ywf12vb7gvj6h3j264nydynmzyqz8d8rqxsj6icqv82as";
      inherit dependencies buildDependencies features;
    };
    winapi_build_0_1_1_ = { dependencies?[], buildDependencies?[], features?[] }: buildRustCrate {
      crateName = "winapi-build";
      version = "0.1.1";
      authors = [ "Peter Atashian <retep998@gmail.com>" ];
      sha256 = "1lxlpi87rkhxcwp2ykf1ldw3p108hwm24nywf3jfrvmff4rjhqga";
      libName = "build";
      inherit dependencies buildDependencies features;
    };

in
rec {
  advapi32_sys_0_2_0 = advapi32_sys_0_2_0_ {
    dependencies = [ winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  aho_corasick_0_5_3 = aho_corasick_0_5_3_ {
    dependencies = [ memchr_0_1_11 ];
  };
  aho_corasick_0_6_3 = aho_corasick_0_6_3_ {
    dependencies = [ memchr_1_0_2 ];
  };
  amq_proto_0_1_0 = amq_proto_0_1_0_ {
    dependencies = [ bit_vec_0_4_4 byteorder_0_5_3 enum_primitive_0_1_1 env_logger_0_3_5 error_chain_0_10_0 log_0_3_8 ];
  };
  amqp_0_1_0 = amqp_0_1_0_ {
    dependencies = [ amq_proto_0_1_0 env_logger_0_3_5 log_0_3_8 openssl_0_9_20 url_1_6_0 ];
    features = [ "openssl" "tls" ];
  };
  antidote_1_0_0 = antidote_1_0_0_ {};
  backtrace_0_3_3 = backtrace_0_3_3_ {
    dependencies = [ cfg_if_0_1_2 rustc_demangle_0_1_5 ]
      ++ (if (kernel == "linux" || kernel == "darwin") && !(kernel == "emscripten") && !(kernel == "darwin") && !(kernel == "ios") then [ backtrace_sys_0_1_16 ] else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then [ libc_0_2_33 ] else [])
      ++ (if kernel == "windows" then [ dbghelp_sys_0_2_0 kernel32_sys_0_2_2 winapi_0_2_8 ] else []);
    features = [ "backtrace-sys" "coresymbolication" "dbghelp" "dbghelp-sys" "dladdr" "kernel32-sys" "libbacktrace" "libunwind" "winapi" ];
  };
  backtrace_sys_0_1_16 = backtrace_sys_0_1_16_ {
    dependencies = [ libc_0_2_33 ];
    buildDependencies = [ cc_1_0_3 ];
  };
  base64_0_6_0 = base64_0_6_0_ {
    dependencies = [ byteorder_1_1_0 safemem_0_2_0 ];
  };
  bit_vec_0_4_4 = bit_vec_0_4_4_ {};
  bitflags_0_7_0 = bitflags_0_7_0_ {};
  bitflags_0_9_1 = bitflags_0_9_1_ {
    features = [ "example_generated" ];
  };
  byteorder_0_5_3 = byteorder_0_5_3_ {
    features = [ "std" ];
  };
  byteorder_1_1_0 = byteorder_1_1_0_ {
    features = [ "std" ];
  };
  cc_1_0_3 = cc_1_0_3_ {
    dependencies = [];
  };
  cfg_if_0_1_2 = cfg_if_0_1_2_ {};
  core_foundation_0_2_3 = core_foundation_0_2_3_ {
    dependencies = [ core_foundation_sys_0_2_3 libc_0_2_33 ];
  };
  core_foundation_sys_0_2_3 = core_foundation_sys_0_2_3_ {
    dependencies = [ libc_0_2_33 ];
  };
  crypt32_sys_0_2_0 = crypt32_sys_0_2_0_ {
    dependencies = [ winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  dbghelp_sys_0_2_0 = dbghelp_sys_0_2_0_ {
    dependencies = [ winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  dtoa_0_4_2 = dtoa_0_4_2_ {};
  enum_primitive_0_1_1 = enum_primitive_0_1_1_ {
    dependencies = [ num_traits_0_1_40 ];
  };
  env_logger_0_3_5 = env_logger_0_3_5_ {
    dependencies = [ log_0_3_8 regex_0_1_80 ];
    features = [ "regex" ];
  };
  env_logger_0_4_3 = env_logger_0_4_3_ {
    dependencies = [ log_0_3_8 regex_0_2_2 ];
    features = [ "regex" ];
  };
  error_chain_0_10_0 = error_chain_0_10_0_ {
    dependencies = [ backtrace_0_3_3 ];
    features = [ "backtrace" "example_generated" ];
  };
  foreign_types_0_2_0 = foreign_types_0_2_0_ {};
  fs2_0_4_2 = fs2_0_4_2_ {
    dependencies = (if (kernel == "linux" || kernel == "darwin") then [ libc_0_2_33 ] else [])
      ++ (if kernel == "windows" then [ kernel32_sys_0_2_2 winapi_0_2_8 ] else []);
  };
  fuchsia_zircon_0_2_1 = fuchsia_zircon_0_2_1_ {
    dependencies = [ fuchsia_zircon_sys_0_2_0 ];
  };
  fuchsia_zircon_sys_0_2_0 = fuchsia_zircon_sys_0_2_0_ {
    dependencies = [ bitflags_0_7_0 ];
  };
  httparse_1_2_3 = httparse_1_2_3_ {
    features = [ "std" ];
  };
  hubcaps_0_3_16 = hubcaps_0_3_16_ {
    dependencies = [ error_chain_0_10_0 hyper_0_10_13 log_0_3_8 serde_1_0_19 serde_derive_1_0_19 serde_json_1_0_6 url_1_6_0 ];
  };
  hyper_0_10_13 = hyper_0_10_13_ {
    dependencies = [ base64_0_6_0 httparse_1_2_3 language_tags_0_2_2 log_0_3_8 mime_0_2_6 num_cpus_1_7_0 time_0_1_38 traitobject_0_1_0 typeable_0_1_2 unicase_1_4_2 url_1_6_0 ];
  };
  hyper_native_tls_0_2_4 = hyper_native_tls_0_2_4_ {
    dependencies = [ antidote_1_0_0 hyper_0_10_13 native_tls_0_1_4 ];
  };
  idna_0_1_4 = idna_0_1_4_ {
    dependencies = [ matches_0_1_6 unicode_bidi_0_3_4 unicode_normalization_0_1_5 ];
  };
  itoa_0_3_4 = itoa_0_3_4_ {};
  kernel32_sys_0_2_2 = kernel32_sys_0_2_2_ {
    dependencies = [ winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  language_tags_0_2_2 = language_tags_0_2_2_ {};
  lazy_static_0_2_9 = lazy_static_0_2_9_ {};
  libc_0_2_33 = libc_0_2_33_ {
    features = [ "use_std" ];
  };
  log_0_3_8 = log_0_3_8_ {
    features = [ "use_std" ];
  };
  matches_0_1_6 = matches_0_1_6_ {};
  md5_0_3_6 = md5_0_3_6_ {};
  memchr_0_1_11 = memchr_0_1_11_ {
    dependencies = [ libc_0_2_33 ];
  };
  memchr_1_0_2 = memchr_1_0_2_ {
    dependencies = [ libc_0_2_33 ];
    features = [ "libc" "use_std" ];
  };
  mime_0_2_6 = mime_0_2_6_ {
    dependencies = [ log_0_3_8 ];
  };
  native_tls_0_1_4 = native_tls_0_1_4_ {
    dependencies = (if !(kernel == "windows" || kernel == "darwin") then [ openssl_0_9_20 ] else [])
      ++ (if kernel == "darwin" then [ security_framework_0_1_16 security_framework_sys_0_1_16 tempdir_0_3_5 ] else [])
      ++ (if kernel == "windows" then [ schannel_0_1_8 ] else []);
  };
  num_traits_0_1_40 = num_traits_0_1_40_ {};
  num_cpus_1_7_0 = num_cpus_1_7_0_ {
    dependencies = [ libc_0_2_33 ];
  };
  ofborg_0_1_0 = ofborg_0_1_0_ {
    dependencies = [ amqp_0_1_0 env_logger_0_4_3 fs2_0_4_2 hubcaps_0_3_16 hyper_0_10_13 hyper_native_tls_0_2_4 log_0_3_8 md5_0_3_6 serde_1_0_19 serde_derive_1_0_19 serde_json_1_0_6 tempfile_2_2_0 ];
  };
  openssl_0_9_20 = openssl_0_9_20_ {
    dependencies = [ bitflags_0_9_1 foreign_types_0_2_0 lazy_static_0_2_9 libc_0_2_33 openssl_sys_0_9_20 ];
  };
  openssl_sys_0_9_20 = openssl_sys_0_9_20_ {
    dependencies = [ libc_0_2_33 ]
      ++ (if abi == "msvc" then [] else []);
    buildDependencies = [ cc_1_0_3 pkg_config_0_3_9 ];
  };
  percent_encoding_1_0_1 = percent_encoding_1_0_1_ {};
  pkg_config_0_3_9 = pkg_config_0_3_9_ {};
  quote_0_3_15 = quote_0_3_15_ {};
  rand_0_3_18 = rand_0_3_18_ {
    dependencies = [ libc_0_2_33 ]
      ++ (if kernel == "fuchsia" then [ fuchsia_zircon_0_2_1 ] else []);
  };
  redox_syscall_0_1_31 = redox_syscall_0_1_31_ {};
  regex_0_1_80 = regex_0_1_80_ {
    dependencies = [ aho_corasick_0_5_3 memchr_0_1_11 regex_syntax_0_3_9 thread_local_0_2_7 utf8_ranges_0_1_3 ];
  };
  regex_0_2_2 = regex_0_2_2_ {
    dependencies = [ aho_corasick_0_6_3 memchr_1_0_2 regex_syntax_0_4_1 thread_local_0_3_4 utf8_ranges_1_0_0 ];
  };
  regex_syntax_0_3_9 = regex_syntax_0_3_9_ {};
  regex_syntax_0_4_1 = regex_syntax_0_4_1_ {};
  rustc_demangle_0_1_5 = rustc_demangle_0_1_5_ {};
  safemem_0_2_0 = safemem_0_2_0_ {};
  schannel_0_1_8 = schannel_0_1_8_ {
    dependencies = [ advapi32_sys_0_2_0 crypt32_sys_0_2_0 kernel32_sys_0_2_2 lazy_static_0_2_9 secur32_sys_0_2_0 winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  secur32_sys_0_2_0 = secur32_sys_0_2_0_ {
    dependencies = [ winapi_0_2_8 ];
    buildDependencies = [ winapi_build_0_1_1 ];
  };
  security_framework_0_1_16 = security_framework_0_1_16_ {
    dependencies = [ core_foundation_0_2_3 core_foundation_sys_0_2_3 libc_0_2_33 security_framework_sys_0_1_16 ];
  };
  security_framework_sys_0_1_16 = security_framework_sys_0_1_16_ {
    dependencies = [ core_foundation_sys_0_2_3 libc_0_2_33 ];
  };
  serde_1_0_19 = serde_1_0_19_ {
    features = [ "std" ];
  };
  serde_derive_1_0_19 = serde_derive_1_0_19_ {
    dependencies = [ quote_0_3_15 serde_derive_internals_0_17_0 syn_0_11_11 ];
  };
  serde_derive_internals_0_17_0 = serde_derive_internals_0_17_0_ {
    dependencies = [ syn_0_11_11 synom_0_11_3 ];
  };
  serde_json_1_0_6 = serde_json_1_0_6_ {
    dependencies = [ dtoa_0_4_2 itoa_0_3_4 num_traits_0_1_40 serde_1_0_19 ];
  };
  syn_0_11_11 = syn_0_11_11_ {
    dependencies = [ quote_0_3_15 synom_0_11_3 unicode_xid_0_0_4 ];
    features = [ "parsing" "printing" "quote" "synom" "unicode-xid" "visit" ];
  };
  synom_0_11_3 = synom_0_11_3_ {
    dependencies = [ unicode_xid_0_0_4 ];
  };
  tempdir_0_3_5 = tempdir_0_3_5_ {
    dependencies = [ rand_0_3_18 ];
  };
  tempfile_2_2_0 = tempfile_2_2_0_ {
    dependencies = [ rand_0_3_18 ]
      ++ (if kernel == "redox" then [ redox_syscall_0_1_31 ] else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then [ libc_0_2_33 ] else [])
      ++ (if kernel == "windows" then [ kernel32_sys_0_2_2 winapi_0_2_8 ] else []);
  };
  thread_id_2_0_0 = thread_id_2_0_0_ {
    dependencies = [ kernel32_sys_0_2_2 libc_0_2_33 ];
  };
  thread_local_0_2_7 = thread_local_0_2_7_ {
    dependencies = [ thread_id_2_0_0 ];
  };
  thread_local_0_3_4 = thread_local_0_3_4_ {
    dependencies = [ lazy_static_0_2_9 unreachable_1_0_0 ];
  };
  time_0_1_38 = time_0_1_38_ {
    dependencies = [ libc_0_2_33 ]
      ++ (if kernel == "redox" then [ redox_syscall_0_1_31 ] else [])
      ++ (if kernel == "windows" then [ kernel32_sys_0_2_2 winapi_0_2_8 ] else []);
  };
  traitobject_0_1_0 = traitobject_0_1_0_ {};
  typeable_0_1_2 = typeable_0_1_2_ {};
  unicase_1_4_2 = unicase_1_4_2_ {
    buildDependencies = [ version_check_0_1_3 ];};
  unicode_bidi_0_3_4 = unicode_bidi_0_3_4_ {
    dependencies = [ matches_0_1_6 ];
  };
  unicode_normalization_0_1_5 = unicode_normalization_0_1_5_ {};
  unicode_xid_0_0_4 = unicode_xid_0_0_4_ {};
  unreachable_1_0_0 = unreachable_1_0_0_ {
    dependencies = [ void_1_0_2 ];
  };
  url_1_6_0 = url_1_6_0_ {
    dependencies = [ idna_0_1_4 matches_0_1_6 percent_encoding_1_0_1 ];
  };
  utf8_ranges_0_1_3 = utf8_ranges_0_1_3_ {};
  utf8_ranges_1_0_0 = utf8_ranges_1_0_0_ {};
  vcpkg_0_2_2 = vcpkg_0_2_2_ {};
  version_check_0_1_3 = version_check_0_1_3_ {};
  void_1_0_2 = void_1_0_2_ {
    features = [ "std" ];
  };
  winapi_0_2_8 = winapi_0_2_8_ {};
  winapi_build_0_1_1 = winapi_build_0_1_1_ {};
}
