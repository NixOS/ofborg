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
# amq-protocol-6.0.0-rc2

  crates.amq_protocol."6.0.0-rc2" = deps: { features?(features_.amq_protocol."6.0.0-rc2" deps {}) }: buildRustCrate {
    crateName = "amq-protocol";
    version = "6.0.0-rc2";
    description = "AMQP specifications";
    authors = [ "Marc-Antoine Perennou <%arc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "15cp7155yqmpczrc3ckcd6kkbvrjqawkhgm77wrjnlrcf0hfwf8r";
    libName = "amq_protocol";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."amq_protocol_tcp"."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_tcp"}" deps)
      (crates."amq_protocol_types"."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_types"}" deps)
      (crates."amq_protocol_uri"."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_uri"}" deps)
      (crates."cookie_factory"."${deps."amq_protocol"."6.0.0-rc2"."cookie_factory"}" deps)
      (crates."nom"."${deps."amq_protocol"."6.0.0-rc2"."nom"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."amq_protocol_codegen"."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_codegen"}" deps)
    ]);
    features = mkFeatures (features."amq_protocol"."6.0.0-rc2" or {});
  };
  features_.amq_protocol."6.0.0-rc2" = deps: f: updateFeatures f (rec {
    amq_protocol = fold recursiveUpdate {} [
      { "6.0.0-rc2"."native-tls" =
        (f.amq_protocol."6.0.0-rc2"."native-tls" or false) ||
        (f.amq_protocol."6.0.0-rc2".default or false) ||
        (amq_protocol."6.0.0-rc2"."default" or false); }
      { "6.0.0-rc2".default = (f.amq_protocol."6.0.0-rc2".default or true); }
    ];
    amq_protocol_codegen."${deps.amq_protocol."6.0.0-rc2".amq_protocol_codegen}".default = true;
    amq_protocol_tcp = fold recursiveUpdate {} [
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."native-tls" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."native-tls" or false) ||
        (amq_protocol."6.0.0-rc2"."native-tls" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."native-tls" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."openssl" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."openssl" or false) ||
        (amq_protocol."6.0.0-rc2"."openssl" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."openssl" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls" or false) ||
        (amq_protocol."6.0.0-rc2"."rustls" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."rustls" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls-native-certs" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls-native-certs" or false) ||
        (amq_protocol."6.0.0-rc2"."rustls-native-certs" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."rustls-native-certs" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls-webpki-roots-certs" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."rustls-webpki-roots-certs" or false) ||
        (amq_protocol."6.0.0-rc2"."rustls-webpki-roots-certs" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."rustls-webpki-roots-certs" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."vendored-openssl" =
        (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}"."vendored-openssl" or false) ||
        (amq_protocol."6.0.0-rc2"."vendored-openssl" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."vendored-openssl" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}".default = (f.amq_protocol_tcp."${deps.amq_protocol."6.0.0-rc2".amq_protocol_tcp}".default or false); }
    ];
    amq_protocol_types = fold recursiveUpdate {} [
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_types}"."verbose-errors" =
        (f.amq_protocol_types."${deps.amq_protocol."6.0.0-rc2".amq_protocol_types}"."verbose-errors" or false) ||
        (amq_protocol."6.0.0-rc2"."verbose-errors" or false) ||
        (f."amq_protocol"."6.0.0-rc2"."verbose-errors" or false); }
      { "${deps.amq_protocol."6.0.0-rc2".amq_protocol_types}".default = true; }
    ];
    amq_protocol_uri."${deps.amq_protocol."6.0.0-rc2".amq_protocol_uri}".default = true;
    cookie_factory = fold recursiveUpdate {} [
      { "${deps.amq_protocol."6.0.0-rc2".cookie_factory}"."std" = true; }
      { "${deps.amq_protocol."6.0.0-rc2".cookie_factory}".default = true; }
    ];
    nom = fold recursiveUpdate {} [
      { "${deps.amq_protocol."6.0.0-rc2".nom}"."std" = true; }
      { "${deps.amq_protocol."6.0.0-rc2".nom}".default = true; }
    ];
  }) [
    (features_.amq_protocol_tcp."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_tcp"}" deps)
    (features_.amq_protocol_types."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_types"}" deps)
    (features_.amq_protocol_uri."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_uri"}" deps)
    (features_.cookie_factory."${deps."amq_protocol"."6.0.0-rc2"."cookie_factory"}" deps)
    (features_.nom."${deps."amq_protocol"."6.0.0-rc2"."nom"}" deps)
    (features_.amq_protocol_codegen."${deps."amq_protocol"."6.0.0-rc2"."amq_protocol_codegen"}" deps)
  ];


# end
# amq-protocol-codegen-6.0.0-rc2

  crates.amq_protocol_codegen."6.0.0-rc2" = deps: { features?(features_.amq_protocol_codegen."6.0.0-rc2" deps {}) }: buildRustCrate {
    crateName = "amq-protocol-codegen";
    version = "6.0.0-rc2";
    description = "AMQP specifications - codegen";
    authors = [ "Marc-Antoine Perennou <%arc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "0wly6zmjl503038845vf38sh63z6761lykcflpdqw4r28xhmi38h";
    libName = "amq_protocol_codegen";
    dependencies = mapFeatures features ([
      (crates."amq_protocol_types"."${deps."amq_protocol_codegen"."6.0.0-rc2"."amq_protocol_types"}" deps)
      (crates."handlebars"."${deps."amq_protocol_codegen"."6.0.0-rc2"."handlebars"}" deps)
      (crates."serde"."${deps."amq_protocol_codegen"."6.0.0-rc2"."serde"}" deps)
      (crates."serde_json"."${deps."amq_protocol_codegen"."6.0.0-rc2"."serde_json"}" deps)
    ]);
  };
  features_.amq_protocol_codegen."6.0.0-rc2" = deps: f: updateFeatures f (rec {
    amq_protocol_codegen."6.0.0-rc2".default = (f.amq_protocol_codegen."6.0.0-rc2".default or true);
    amq_protocol_types."${deps.amq_protocol_codegen."6.0.0-rc2".amq_protocol_types}".default = true;
    handlebars."${deps.amq_protocol_codegen."6.0.0-rc2".handlebars}".default = true;
    serde = fold recursiveUpdate {} [
      { "${deps.amq_protocol_codegen."6.0.0-rc2".serde}"."derive" = true; }
      { "${deps.amq_protocol_codegen."6.0.0-rc2".serde}".default = true; }
    ];
    serde_json."${deps.amq_protocol_codegen."6.0.0-rc2".serde_json}".default = true;
  }) [
    (features_.amq_protocol_types."${deps."amq_protocol_codegen"."6.0.0-rc2"."amq_protocol_types"}" deps)
    (features_.handlebars."${deps."amq_protocol_codegen"."6.0.0-rc2"."handlebars"}" deps)
    (features_.serde."${deps."amq_protocol_codegen"."6.0.0-rc2"."serde"}" deps)
    (features_.serde_json."${deps."amq_protocol_codegen"."6.0.0-rc2"."serde_json"}" deps)
  ];


# end
# amq-protocol-tcp-6.0.0-rc2

  crates.amq_protocol_tcp."6.0.0-rc2" = deps: { features?(features_.amq_protocol_tcp."6.0.0-rc2" deps {}) }: buildRustCrate {
    crateName = "amq-protocol-tcp";
    version = "6.0.0-rc2";
    description = "AMQP URI TCP connection handling";
    authors = [ "Marc-Antoine Perennou <%arc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "09lmizfxaq3azg2qf7j65hp74z4z360cs327bzcmipd90fa35q28";
    libName = "amq_protocol_tcp";
    dependencies = mapFeatures features ([
      (crates."amq_protocol_uri"."${deps."amq_protocol_tcp"."6.0.0-rc2"."amq_protocol_uri"}" deps)
      (crates."log"."${deps."amq_protocol_tcp"."6.0.0-rc2"."log"}" deps)
      (crates."tcp_stream"."${deps."amq_protocol_tcp"."6.0.0-rc2"."tcp_stream"}" deps)
    ]);
    features = mkFeatures (features."amq_protocol_tcp"."6.0.0-rc2" or {});
  };
  features_.amq_protocol_tcp."6.0.0-rc2" = deps: f: updateFeatures f (rec {
    amq_protocol_tcp = fold recursiveUpdate {} [
      { "6.0.0-rc2"."native-tls" =
        (f.amq_protocol_tcp."6.0.0-rc2"."native-tls" or false) ||
        (f.amq_protocol_tcp."6.0.0-rc2".default or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."default" or false); }
      { "6.0.0-rc2"."rustls-connector" =
        (f.amq_protocol_tcp."6.0.0-rc2"."rustls-connector" or false) ||
        (f.amq_protocol_tcp."6.0.0-rc2".rustls-native-certs or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls-native-certs" or false) ||
        (f.amq_protocol_tcp."6.0.0-rc2".rustls-webpki-roots-certs or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls-webpki-roots-certs" or false); }
      { "6.0.0-rc2"."rustls-native-certs" =
        (f.amq_protocol_tcp."6.0.0-rc2"."rustls-native-certs" or false) ||
        (f.amq_protocol_tcp."6.0.0-rc2".rustls or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls" or false); }
      { "6.0.0-rc2".default = (f.amq_protocol_tcp."6.0.0-rc2".default or true); }
    ];
    amq_protocol_uri."${deps.amq_protocol_tcp."6.0.0-rc2".amq_protocol_uri}".default = true;
    log."${deps.amq_protocol_tcp."6.0.0-rc2".log}".default = true;
    tcp_stream = fold recursiveUpdate {} [
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."native-tls" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."native-tls" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."native-tls" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."native-tls" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."openssl" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."openssl" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."openssl" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."openssl" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-connector" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-connector" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls-connector" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."rustls-connector" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-native-certs" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-native-certs" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls-native-certs" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."rustls-native-certs" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-webpki-roots-certs" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."rustls-webpki-roots-certs" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."rustls-webpki-roots-certs" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."rustls-webpki-roots-certs" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."vendored-openssl" =
        (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}"."vendored-openssl" or false) ||
        (amq_protocol_tcp."6.0.0-rc2"."vendored-openssl" or false) ||
        (f."amq_protocol_tcp"."6.0.0-rc2"."vendored-openssl" or false); }
      { "${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}".default = (f.tcp_stream."${deps.amq_protocol_tcp."6.0.0-rc2".tcp_stream}".default or false); }
    ];
  }) [
    (features_.amq_protocol_uri."${deps."amq_protocol_tcp"."6.0.0-rc2"."amq_protocol_uri"}" deps)
    (features_.log."${deps."amq_protocol_tcp"."6.0.0-rc2"."log"}" deps)
    (features_.tcp_stream."${deps."amq_protocol_tcp"."6.0.0-rc2"."tcp_stream"}" deps)
  ];


# end
# amq-protocol-types-6.0.0-rc2

  crates.amq_protocol_types."6.0.0-rc2" = deps: { features?(features_.amq_protocol_types."6.0.0-rc2" deps {}) }: buildRustCrate {
    crateName = "amq-protocol-types";
    version = "6.0.0-rc2";
    description = "AMQP specifications - types";
    authors = [ "Marc-Antoine Perennou <%arc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "0fvypamdx218mw4ji46cpwpb4ikl0kf9si2yi1mcfk0rrhhb41fh";
    libName = "amq_protocol_types";
    dependencies = mapFeatures features ([
      (crates."cookie_factory"."${deps."amq_protocol_types"."6.0.0-rc2"."cookie_factory"}" deps)
      (crates."nom"."${deps."amq_protocol_types"."6.0.0-rc2"."nom"}" deps)
      (crates."serde"."${deps."amq_protocol_types"."6.0.0-rc2"."serde"}" deps)
      (crates."serde_json"."${deps."amq_protocol_types"."6.0.0-rc2"."serde_json"}" deps)
    ]);
    features = mkFeatures (features."amq_protocol_types"."6.0.0-rc2" or {});
  };
  features_.amq_protocol_types."6.0.0-rc2" = deps: f: updateFeatures f (rec {
    amq_protocol_types."6.0.0-rc2".default = (f.amq_protocol_types."6.0.0-rc2".default or true);
    cookie_factory = fold recursiveUpdate {} [
      { "${deps.amq_protocol_types."6.0.0-rc2".cookie_factory}"."std" = true; }
      { "${deps.amq_protocol_types."6.0.0-rc2".cookie_factory}".default = true; }
    ];
    nom = fold recursiveUpdate {} [
      { "${deps.amq_protocol_types."6.0.0-rc2".nom}"."std" = true; }
      { "${deps.amq_protocol_types."6.0.0-rc2".nom}".default = true; }
    ];
    serde = fold recursiveUpdate {} [
      { "${deps.amq_protocol_types."6.0.0-rc2".serde}"."derive" = true; }
      { "${deps.amq_protocol_types."6.0.0-rc2".serde}".default = true; }
    ];
    serde_json."${deps.amq_protocol_types."6.0.0-rc2".serde_json}".default = true;
  }) [
    (features_.cookie_factory."${deps."amq_protocol_types"."6.0.0-rc2"."cookie_factory"}" deps)
    (features_.nom."${deps."amq_protocol_types"."6.0.0-rc2"."nom"}" deps)
    (features_.serde."${deps."amq_protocol_types"."6.0.0-rc2"."serde"}" deps)
    (features_.serde_json."${deps."amq_protocol_types"."6.0.0-rc2"."serde_json"}" deps)
  ];


# end
# amq-protocol-uri-6.0.0-rc2

  crates.amq_protocol_uri."6.0.0-rc2" = deps: { features?(features_.amq_protocol_uri."6.0.0-rc2" deps {}) }: buildRustCrate {
    crateName = "amq-protocol-uri";
    version = "6.0.0-rc2";
    description = "AMQP URI manipulation";
    authors = [ "Marc-Antoine Perennou <%arc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "04mabvw38y5bzwcms4zxzpfpkc5c9v9is1wyi02z7bgs8yp9kfvn";
    libName = "amq_protocol_uri";
    dependencies = mapFeatures features ([
      (crates."percent_encoding"."${deps."amq_protocol_uri"."6.0.0-rc2"."percent_encoding"}" deps)
      (crates."url"."${deps."amq_protocol_uri"."6.0.0-rc2"."url"}" deps)
    ]);
  };
  features_.amq_protocol_uri."6.0.0-rc2" = deps: f: updateFeatures f (rec {
    amq_protocol_uri."6.0.0-rc2".default = (f.amq_protocol_uri."6.0.0-rc2".default or true);
    percent_encoding."${deps.amq_protocol_uri."6.0.0-rc2".percent_encoding}".default = true;
    url."${deps.amq_protocol_uri."6.0.0-rc2".url}".default = true;
  }) [
    (features_.percent_encoding."${deps."amq_protocol_uri"."6.0.0-rc2"."percent_encoding"}" deps)
    (features_.url."${deps."amq_protocol_uri"."6.0.0-rc2"."url"}" deps)
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
# arrayvec-0.5.1

  crates.arrayvec."0.5.1" = deps: { features?(features_.arrayvec."0.5.1" deps {}) }: buildRustCrate {
    crateName = "arrayvec";
    version = "0.5.1";
    description = "A vector with fixed capacity, backed by an array (it can be stored on the stack too). Implements fixed capacity ArrayVec and ArrayString.";
    authors = [ "bluss" ];
    edition = "2018";
    sha256 = "01fc06ab7zh75z26m2l4a0fc7zy4zpr962qazdcp9hl4fgdwbj6v";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."arrayvec"."0.5.1" or {});
  };
  features_.arrayvec."0.5.1" = deps: f: updateFeatures f (rec {
    arrayvec = fold recursiveUpdate {} [
      { "0.5.1"."std" =
        (f.arrayvec."0.5.1"."std" or false) ||
        (f.arrayvec."0.5.1".default or false) ||
        (arrayvec."0.5.1"."default" or false); }
      { "0.5.1".default = (f.arrayvec."0.5.1".default or true); }
    ];
  }) [];


# end
# async-std-1.5.0

  crates.async_std."1.5.0" = deps: { features?(features_.async_std."1.5.0" deps {}) }: buildRustCrate {
    crateName = "async-std";
    version = "1.5.0";
    description = "Async version of the Rust standard library";
    authors = [ "Stjepan Glavina <stjepang@gmail.com>" "Yoshua Wuyts <yoshuawuyts@gmail.com>" "Contributors to async-std" ];
    edition = "2018";
    sha256 = "1nij2ch2idcrz5j70icd9979gw2ri7j0xcasla79q2ckkxphly8d";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.async_std."1.5.0".async-task or false then [ (crates.async_task."${deps."async_std"."1.5.0".async_task}" deps) ] else [])
      ++ (if features.async_std."1.5.0".crossbeam-channel or false then [ (crates.crossbeam_channel."${deps."async_std"."1.5.0".crossbeam_channel}" deps) ] else [])
      ++ (if features.async_std."1.5.0".crossbeam-deque or false then [ (crates.crossbeam_deque."${deps."async_std"."1.5.0".crossbeam_deque}" deps) ] else [])
      ++ (if features.async_std."1.5.0".crossbeam-utils or false then [ (crates.crossbeam_utils."${deps."async_std"."1.5.0".crossbeam_utils}" deps) ] else [])
      ++ (if features.async_std."1.5.0".futures-core or false then [ (crates.futures_core."${deps."async_std"."1.5.0".futures_core}" deps) ] else [])
      ++ (if features.async_std."1.5.0".futures-io or false then [ (crates.futures_io."${deps."async_std"."1.5.0".futures_io}" deps) ] else [])
      ++ (if features.async_std."1.5.0".futures-timer or false then [ (crates.futures_timer."${deps."async_std"."1.5.0".futures_timer}" deps) ] else [])
      ++ (if features.async_std."1.5.0".kv-log-macro or false then [ (crates.kv_log_macro."${deps."async_std"."1.5.0".kv_log_macro}" deps) ] else [])
      ++ (if features.async_std."1.5.0".log or false then [ (crates.log."${deps."async_std"."1.5.0".log}" deps) ] else [])
      ++ (if features.async_std."1.5.0".memchr or false then [ (crates.memchr."${deps."async_std"."1.5.0".memchr}" deps) ] else [])
      ++ (if features.async_std."1.5.0".mio or false then [ (crates.mio."${deps."async_std"."1.5.0".mio}" deps) ] else [])
      ++ (if features.async_std."1.5.0".mio-uds or false then [ (crates.mio_uds."${deps."async_std"."1.5.0".mio_uds}" deps) ] else [])
      ++ (if features.async_std."1.5.0".num_cpus or false then [ (crates.num_cpus."${deps."async_std"."1.5.0".num_cpus}" deps) ] else [])
      ++ (if features.async_std."1.5.0".once_cell or false then [ (crates.once_cell."${deps."async_std"."1.5.0".once_cell}" deps) ] else [])
      ++ (if features.async_std."1.5.0".pin-project-lite or false then [ (crates.pin_project_lite."${deps."async_std"."1.5.0".pin_project_lite}" deps) ] else [])
      ++ (if features.async_std."1.5.0".pin-utils or false then [ (crates.pin_utils."${deps."async_std"."1.5.0".pin_utils}" deps) ] else [])
      ++ (if features.async_std."1.5.0".slab or false then [ (crates.slab."${deps."async_std"."1.5.0".slab}" deps) ] else []));
    features = mkFeatures (features."async_std"."1.5.0" or {});
  };
  features_.async_std."1.5.0" = deps: f: updateFeatures f (rec {
    async_std = fold recursiveUpdate {} [
      { "1.5.0"."async-attributes" =
        (f.async_std."1.5.0"."async-attributes" or false) ||
        (f.async_std."1.5.0".attributes or false) ||
        (async_std."1.5.0"."attributes" or false); }
      { "1.5.0"."async-task" =
        (f.async_std."1.5.0"."async-task" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."attributes" =
        (f.async_std."1.5.0"."attributes" or false) ||
        (f.async_std."1.5.0".docs or false) ||
        (async_std."1.5.0"."docs" or false); }
      { "1.5.0"."broadcaster" =
        (f.async_std."1.5.0"."broadcaster" or false) ||
        (f.async_std."1.5.0".unstable or false) ||
        (async_std."1.5.0"."unstable" or false); }
      { "1.5.0"."crossbeam-channel" =
        (f.async_std."1.5.0"."crossbeam-channel" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."crossbeam-deque" =
        (f.async_std."1.5.0"."crossbeam-deque" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."crossbeam-utils" =
        (f.async_std."1.5.0"."crossbeam-utils" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."default" =
        (f.async_std."1.5.0"."default" or false) ||
        (f.async_std."1.5.0".docs or false) ||
        (async_std."1.5.0"."docs" or false); }
      { "1.5.0"."futures-core" =
        (f.async_std."1.5.0"."futures-core" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."futures-io" =
        (f.async_std."1.5.0"."futures-io" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."futures-timer" =
        (f.async_std."1.5.0"."futures-timer" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false) ||
        (f.async_std."1.5.0".unstable or false) ||
        (async_std."1.5.0"."unstable" or false); }
      { "1.5.0"."kv-log-macro" =
        (f.async_std."1.5.0"."kv-log-macro" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."log" =
        (f.async_std."1.5.0"."log" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."memchr" =
        (f.async_std."1.5.0"."memchr" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."mio" =
        (f.async_std."1.5.0"."mio" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."mio-uds" =
        (f.async_std."1.5.0"."mio-uds" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."num_cpus" =
        (f.async_std."1.5.0"."num_cpus" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false); }
      { "1.5.0"."once_cell" =
        (f.async_std."1.5.0"."once_cell" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."pin-project-lite" =
        (f.async_std."1.5.0"."pin-project-lite" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."pin-utils" =
        (f.async_std."1.5.0"."pin-utils" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."slab" =
        (f.async_std."1.5.0"."slab" or false) ||
        (f.async_std."1.5.0".std or false) ||
        (async_std."1.5.0"."std" or false); }
      { "1.5.0"."std" =
        (f.async_std."1.5.0"."std" or false) ||
        (f.async_std."1.5.0".default or false) ||
        (async_std."1.5.0"."default" or false) ||
        (f.async_std."1.5.0".unstable or false) ||
        (async_std."1.5.0"."unstable" or false); }
      { "1.5.0"."unstable" =
        (f.async_std."1.5.0"."unstable" or false) ||
        (f.async_std."1.5.0".docs or false) ||
        (async_std."1.5.0"."docs" or false); }
      { "1.5.0".default = (f.async_std."1.5.0".default or true); }
    ];
    async_task."${deps.async_std."1.5.0".async_task}".default = true;
    crossbeam_channel."${deps.async_std."1.5.0".crossbeam_channel}".default = true;
    crossbeam_deque."${deps.async_std."1.5.0".crossbeam_deque}".default = true;
    crossbeam_utils."${deps.async_std."1.5.0".crossbeam_utils}".default = true;
    futures_core."${deps.async_std."1.5.0".futures_core}".default = true;
    futures_io."${deps.async_std."1.5.0".futures_io}".default = true;
    futures_timer."${deps.async_std."1.5.0".futures_timer}".default = true;
    kv_log_macro."${deps.async_std."1.5.0".kv_log_macro}".default = true;
    log = fold recursiveUpdate {} [
      { "${deps.async_std."1.5.0".log}"."kv_unstable" = true; }
      { "${deps.async_std."1.5.0".log}".default = true; }
    ];
    memchr."${deps.async_std."1.5.0".memchr}".default = true;
    mio."${deps.async_std."1.5.0".mio}".default = true;
    mio_uds."${deps.async_std."1.5.0".mio_uds}".default = true;
    num_cpus."${deps.async_std."1.5.0".num_cpus}".default = true;
    once_cell."${deps.async_std."1.5.0".once_cell}".default = true;
    pin_project_lite."${deps.async_std."1.5.0".pin_project_lite}".default = true;
    pin_utils."${deps.async_std."1.5.0".pin_utils}".default = true;
    slab."${deps.async_std."1.5.0".slab}".default = true;
  }) [
    (features_.async_task."${deps."async_std"."1.5.0"."async_task"}" deps)
    (features_.crossbeam_channel."${deps."async_std"."1.5.0"."crossbeam_channel"}" deps)
    (features_.crossbeam_deque."${deps."async_std"."1.5.0"."crossbeam_deque"}" deps)
    (features_.crossbeam_utils."${deps."async_std"."1.5.0"."crossbeam_utils"}" deps)
    (features_.futures_core."${deps."async_std"."1.5.0"."futures_core"}" deps)
    (features_.futures_io."${deps."async_std"."1.5.0"."futures_io"}" deps)
    (features_.futures_timer."${deps."async_std"."1.5.0"."futures_timer"}" deps)
    (features_.kv_log_macro."${deps."async_std"."1.5.0"."kv_log_macro"}" deps)
    (features_.log."${deps."async_std"."1.5.0"."log"}" deps)
    (features_.memchr."${deps."async_std"."1.5.0"."memchr"}" deps)
    (features_.mio."${deps."async_std"."1.5.0"."mio"}" deps)
    (features_.mio_uds."${deps."async_std"."1.5.0"."mio_uds"}" deps)
    (features_.num_cpus."${deps."async_std"."1.5.0"."num_cpus"}" deps)
    (features_.once_cell."${deps."async_std"."1.5.0"."once_cell"}" deps)
    (features_.pin_project_lite."${deps."async_std"."1.5.0"."pin_project_lite"}" deps)
    (features_.pin_utils."${deps."async_std"."1.5.0"."pin_utils"}" deps)
    (features_.slab."${deps."async_std"."1.5.0"."slab"}" deps)
  ];


# end
# async-task-1.3.1

  crates.async_task."1.3.1" = deps: { features?(features_.async_task."1.3.1" deps {}) }: buildRustCrate {
    crateName = "async-task";
    version = "1.3.1";
    description = "Task abstraction for building executors";
    authors = [ "Stjepan Glavina <stjepang@gmail.com>" ];
    edition = "2018";
    sha256 = "1zrs4yzhrzjgjf0988i8mxg6gnwwfrap081pxzyk1wfab9sn59dj";
    dependencies = (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."async_task"."1.3.1"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."async_task"."1.3.1"."winapi"}" deps)
    ]) else []);
  };
  features_.async_task."1.3.1" = deps: f: updateFeatures f (rec {
    async_task."1.3.1".default = (f.async_task."1.3.1".default or true);
    libc."${deps.async_task."1.3.1".libc}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.async_task."1.3.1".winapi}"."processthreadsapi" = true; }
      { "${deps.async_task."1.3.1".winapi}".default = true; }
    ];
  }) [
    (features_.libc."${deps."async_task"."1.3.1"."libc"}" deps)
    (features_.winapi."${deps."async_task"."1.3.1"."winapi"}" deps)
  ];


# end
# async-task-3.0.0

  crates.async_task."3.0.0" = deps: { features?(features_.async_task."3.0.0" deps {}) }: buildRustCrate {
    crateName = "async-task";
    version = "3.0.0";
    description = "Task abstraction for building executors";
    authors = [ "Stjepan Glavina <stjepang@gmail.com>" ];
    edition = "2018";
    sha256 = "1aqalsjp0k71sp0jgdhvcy5vpslkjzm0zr52q6fzrjy4jz0f0yag";
    features = mkFeatures (features."async_task"."3.0.0" or {});
  };
  features_.async_task."3.0.0" = deps: f: updateFeatures f (rec {
    async_task = fold recursiveUpdate {} [
      { "3.0.0"."std" =
        (f.async_task."3.0.0"."std" or false) ||
        (f.async_task."3.0.0".default or false) ||
        (async_task."3.0.0"."default" or false); }
      { "3.0.0".default = (f.async_task."3.0.0".default or true); }
    ];
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
# autocfg-1.0.0

  crates.autocfg."1.0.0" = deps: { features?(features_.autocfg."1.0.0" deps {}) }: buildRustCrate {
    crateName = "autocfg";
    version = "1.0.0";
    description = "Automatic cfg for Rust compiler features";
    authors = [ "Josh Stone <cuviper@gmail.com>" ];
    sha256 = "1hhgqh551gmws22z9rxbnsvlppwxvlj0nszj7n1x56pqa3j3swy7";
  };
  features_.autocfg."1.0.0" = deps: f: updateFeatures f (rec {
    autocfg."1.0.0".default = (f.autocfg."1.0.0".default or true);
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
# bitflags-1.2.1

  crates.bitflags."1.2.1" = deps: { features?(features_.bitflags."1.2.1" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "1.2.1";
    description = "A macro to generate structures which behave like bitflags.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0b77awhpn7yaqjjibm69ginfn996azx5vkzfjj39g3wbsqs7mkxg";
    build = "build.rs";
    features = mkFeatures (features."bitflags"."1.2.1" or {});
  };
  features_.bitflags."1.2.1" = deps: f: updateFeatures f (rec {
    bitflags."1.2.1".default = (f.bitflags."1.2.1".default or true);
  }) [];


# end
# block-buffer-0.7.3

  crates.block_buffer."0.7.3" = deps: { features?(features_.block_buffer."0.7.3" deps {}) }: buildRustCrate {
    crateName = "block-buffer";
    version = "0.7.3";
    description = "Fixed size buffer for block processing of data";
    authors = [ "RustCrypto Developers" ];
    sha256 = "0kryp6l1ia1f5vxmmzggx0pnc5zqxm6m9m9wvh5y0b3wdcj5xm1v";
    dependencies = mapFeatures features ([
      (crates."block_padding"."${deps."block_buffer"."0.7.3"."block_padding"}" deps)
      (crates."byte_tools"."${deps."block_buffer"."0.7.3"."byte_tools"}" deps)
      (crates."byteorder"."${deps."block_buffer"."0.7.3"."byteorder"}" deps)
      (crates."generic_array"."${deps."block_buffer"."0.7.3"."generic_array"}" deps)
    ]);
  };
  features_.block_buffer."0.7.3" = deps: f: updateFeatures f (rec {
    block_buffer."0.7.3".default = (f.block_buffer."0.7.3".default or true);
    block_padding."${deps.block_buffer."0.7.3".block_padding}".default = true;
    byte_tools."${deps.block_buffer."0.7.3".byte_tools}".default = true;
    byteorder."${deps.block_buffer."0.7.3".byteorder}".default = (f.byteorder."${deps.block_buffer."0.7.3".byteorder}".default or false);
    generic_array."${deps.block_buffer."0.7.3".generic_array}".default = true;
  }) [
    (features_.block_padding."${deps."block_buffer"."0.7.3"."block_padding"}" deps)
    (features_.byte_tools."${deps."block_buffer"."0.7.3"."byte_tools"}" deps)
    (features_.byteorder."${deps."block_buffer"."0.7.3"."byteorder"}" deps)
    (features_.generic_array."${deps."block_buffer"."0.7.3"."generic_array"}" deps)
  ];


# end
# block-padding-0.1.5

  crates.block_padding."0.1.5" = deps: { features?(features_.block_padding."0.1.5" deps {}) }: buildRustCrate {
    crateName = "block-padding";
    version = "0.1.5";
    description = "Padding and unpadding of messages divided into blocks.";
    authors = [ "RustCrypto Developers" ];
    sha256 = "1v1xjpkkkb1skjniy75f2vg1g8s8wma29a8xph11fjarrimjk5sr";
    dependencies = mapFeatures features ([
      (crates."byte_tools"."${deps."block_padding"."0.1.5"."byte_tools"}" deps)
    ]);
  };
  features_.block_padding."0.1.5" = deps: f: updateFeatures f (rec {
    block_padding."0.1.5".default = (f.block_padding."0.1.5".default or true);
    byte_tools."${deps.block_padding."0.1.5".byte_tools}".default = true;
  }) [
    (features_.byte_tools."${deps."block_padding"."0.1.5"."byte_tools"}" deps)
  ];


# end
# byte-tools-0.3.1

  crates.byte_tools."0.3.1" = deps: { features?(features_.byte_tools."0.3.1" deps {}) }: buildRustCrate {
    crateName = "byte-tools";
    version = "0.3.1";
    description = "Bytes related utility functions";
    authors = [ "RustCrypto Developers" ];
    sha256 = "01hfp59bxq74glhfmhvm9wma2migq2kfmvcvqq5pssk5k52g8ja0";
  };
  features_.byte_tools."0.3.1" = deps: f: updateFeatures f (rec {
    byte_tools."0.3.1".default = (f.byte_tools."0.3.1".default or true);
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
# cfg-if-0.1.10

  crates.cfg_if."0.1.10" = deps: { features?(features_.cfg_if."0.1.10" deps {}) }: buildRustCrate {
    crateName = "cfg-if";
    version = "0.1.10";
    description = "A macro to ergonomically define an item depending on a large number of #[cfg]\nparameters. Structured like an if-else chain, the first matching branch is the\nitem that gets emitted.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "0x52qzpbyl2f2jqs7kkqzgfki2cpq99gpfjjigdp8pwwfqk01007";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."cfg_if"."0.1.10" or {});
  };
  features_.cfg_if."0.1.10" = deps: f: updateFeatures f (rec {
    cfg_if = fold recursiveUpdate {} [
      { "0.1.10"."compiler_builtins" =
        (f.cfg_if."0.1.10"."compiler_builtins" or false) ||
        (f.cfg_if."0.1.10".rustc-dep-of-std or false) ||
        (cfg_if."0.1.10"."rustc-dep-of-std" or false); }
      { "0.1.10"."core" =
        (f.cfg_if."0.1.10"."core" or false) ||
        (f.cfg_if."0.1.10".rustc-dep-of-std or false) ||
        (cfg_if."0.1.10"."rustc-dep-of-std" or false); }
      { "0.1.10".default = (f.cfg_if."0.1.10".default or true); }
    ];
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
# cloudabi-0.0.3

  crates.cloudabi."0.0.3" = deps: { features?(features_.cloudabi."0.0.3" deps {}) }: buildRustCrate {
    crateName = "cloudabi";
    version = "0.0.3";
    description = "Low level interface to CloudABI. Contains all syscalls and related types.";
    authors = [ "Nuxi (https://nuxi.nl/) and contributors" ];
    sha256 = "1z9lby5sr6vslfd14d6igk03s7awf91mxpsfmsp3prxbxlk0x7h5";
    libPath = "cloudabi.rs";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.cloudabi."0.0.3".bitflags or false then [ (crates.bitflags."${deps."cloudabi"."0.0.3".bitflags}" deps) ] else []));
    features = mkFeatures (features."cloudabi"."0.0.3" or {});
  };
  features_.cloudabi."0.0.3" = deps: f: updateFeatures f (rec {
    bitflags."${deps.cloudabi."0.0.3".bitflags}".default = true;
    cloudabi = fold recursiveUpdate {} [
      { "0.0.3"."bitflags" =
        (f.cloudabi."0.0.3"."bitflags" or false) ||
        (f.cloudabi."0.0.3".default or false) ||
        (cloudabi."0.0.3"."default" or false); }
      { "0.0.3".default = (f.cloudabi."0.0.3".default or true); }
    ];
  }) [
    (features_.bitflags."${deps."cloudabi"."0.0.3"."bitflags"}" deps)
  ];


# end
# cookie-factory-0.3.1

  crates.cookie_factory."0.3.1" = deps: { features?(features_.cookie_factory."0.3.1" deps {}) }: buildRustCrate {
    crateName = "cookie-factory";
    version = "0.3.1";
    description = "nom inspired serialization library";
    authors = [ "Geoffroy Couprie <geo.couprie@gmail.com>" "Pierre Chifflier <chifflier@wzdftpd.net>" ];
    edition = "2018";
    sha256 = "080cxl2a2a762bmv59y4480c73mv908596nhk4v5cdb9azrk0bp3";
    features = mkFeatures (features."cookie_factory"."0.3.1" or {});
  };
  features_.cookie_factory."0.3.1" = deps: f: updateFeatures f (rec {
    cookie_factory = fold recursiveUpdate {} [
      { "0.3.1"."std" =
        (f.cookie_factory."0.3.1"."std" or false) ||
        (f.cookie_factory."0.3.1".default or false) ||
        (cookie_factory."0.3.1"."default" or false); }
      { "0.3.1".default = (f.cookie_factory."0.3.1".default or true); }
    ];
  }) [];


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
# core-foundation-0.7.0

  crates.core_foundation."0.7.0" = deps: { features?(features_.core_foundation."0.7.0" deps {}) }: buildRustCrate {
    crateName = "core-foundation";
    version = "0.7.0";
    description = "Bindings to Core Foundation for macOS";
    authors = [ "The Servo Project Developers" ];
    sha256 = "0ylwql6qpz328yni1gp5dq9pqzzdcnxjavq4chg2vxlb9fvilpal";
    dependencies = mapFeatures features ([
      (crates."core_foundation_sys"."${deps."core_foundation"."0.7.0"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."core_foundation"."0.7.0"."libc"}" deps)
    ]);
    features = mkFeatures (features."core_foundation"."0.7.0" or {});
  };
  features_.core_foundation."0.7.0" = deps: f: updateFeatures f (rec {
    core_foundation = fold recursiveUpdate {} [
      { "0.7.0"."chrono" =
        (f.core_foundation."0.7.0"."chrono" or false) ||
        (f.core_foundation."0.7.0".with-chrono or false) ||
        (core_foundation."0.7.0"."with-chrono" or false); }
      { "0.7.0"."uuid" =
        (f.core_foundation."0.7.0"."uuid" or false) ||
        (f.core_foundation."0.7.0".with-uuid or false) ||
        (core_foundation."0.7.0"."with-uuid" or false); }
      { "0.7.0".default = (f.core_foundation."0.7.0".default or true); }
    ];
    core_foundation_sys = fold recursiveUpdate {} [
      { "${deps.core_foundation."0.7.0".core_foundation_sys}"."mac_os_10_7_support" =
        (f.core_foundation_sys."${deps.core_foundation."0.7.0".core_foundation_sys}"."mac_os_10_7_support" or false) ||
        (core_foundation."0.7.0"."mac_os_10_7_support" or false) ||
        (f."core_foundation"."0.7.0"."mac_os_10_7_support" or false); }
      { "${deps.core_foundation."0.7.0".core_foundation_sys}"."mac_os_10_8_features" =
        (f.core_foundation_sys."${deps.core_foundation."0.7.0".core_foundation_sys}"."mac_os_10_8_features" or false) ||
        (core_foundation."0.7.0"."mac_os_10_8_features" or false) ||
        (f."core_foundation"."0.7.0"."mac_os_10_8_features" or false); }
      { "${deps.core_foundation."0.7.0".core_foundation_sys}".default = true; }
    ];
    libc."${deps.core_foundation."0.7.0".libc}".default = true;
  }) [
    (features_.core_foundation_sys."${deps."core_foundation"."0.7.0"."core_foundation_sys"}" deps)
    (features_.libc."${deps."core_foundation"."0.7.0"."libc"}" deps)
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
# core-foundation-sys-0.7.0

  crates.core_foundation_sys."0.7.0" = deps: { features?(features_.core_foundation_sys."0.7.0" deps {}) }: buildRustCrate {
    crateName = "core-foundation-sys";
    version = "0.7.0";
    description = "Bindings to Core Foundation for macOS";
    authors = [ "The Servo Project Developers" ];
    sha256 = "09la5dp2a2s8zbzx7bxrvj5f2ncjy7blla8ljfpk6rwpcn2phxmj";
    build = "build.rs";
    features = mkFeatures (features."core_foundation_sys"."0.7.0" or {});
  };
  features_.core_foundation_sys."0.7.0" = deps: f: updateFeatures f (rec {
    core_foundation_sys."0.7.0".default = (f.core_foundation_sys."0.7.0".default or true);
  }) [];


# end
# crossbeam-channel-0.4.2

  crates.crossbeam_channel."0.4.2" = deps: { features?(features_.crossbeam_channel."0.4.2" deps {}) }: buildRustCrate {
    crateName = "crossbeam-channel";
    version = "0.4.2";
    description = "Multi-producer multi-consumer channels for message passing";
    authors = [ "The Crossbeam Project Developers" ];
    sha256 = "0rlr1pzhfb5jyrpb026p37g12qaaz6sv2gd6qszcgwdmmmaw8ly6";
    dependencies = mapFeatures features ([
      (crates."crossbeam_utils"."${deps."crossbeam_channel"."0.4.2"."crossbeam_utils"}" deps)
      (crates."maybe_uninit"."${deps."crossbeam_channel"."0.4.2"."maybe_uninit"}" deps)
    ]);
  };
  features_.crossbeam_channel."0.4.2" = deps: f: updateFeatures f (rec {
    crossbeam_channel."0.4.2".default = (f.crossbeam_channel."0.4.2".default or true);
    crossbeam_utils."${deps.crossbeam_channel."0.4.2".crossbeam_utils}".default = true;
    maybe_uninit."${deps.crossbeam_channel."0.4.2".maybe_uninit}".default = true;
  }) [
    (features_.crossbeam_utils."${deps."crossbeam_channel"."0.4.2"."crossbeam_utils"}" deps)
    (features_.maybe_uninit."${deps."crossbeam_channel"."0.4.2"."maybe_uninit"}" deps)
  ];


# end
# crossbeam-deque-0.7.3

  crates.crossbeam_deque."0.7.3" = deps: { features?(features_.crossbeam_deque."0.7.3" deps {}) }: buildRustCrate {
    crateName = "crossbeam-deque";
    version = "0.7.3";
    description = "Concurrent work-stealing deque";
    authors = [ "The Crossbeam Project Developers" ];
    sha256 = "1ib3h4brflwmkbaiv351p8ahcd6srp98c4rxwxq876grl9jarp53";
    dependencies = mapFeatures features ([
      (crates."crossbeam_epoch"."${deps."crossbeam_deque"."0.7.3"."crossbeam_epoch"}" deps)
      (crates."crossbeam_utils"."${deps."crossbeam_deque"."0.7.3"."crossbeam_utils"}" deps)
      (crates."maybe_uninit"."${deps."crossbeam_deque"."0.7.3"."maybe_uninit"}" deps)
    ]);
  };
  features_.crossbeam_deque."0.7.3" = deps: f: updateFeatures f (rec {
    crossbeam_deque."0.7.3".default = (f.crossbeam_deque."0.7.3".default or true);
    crossbeam_epoch."${deps.crossbeam_deque."0.7.3".crossbeam_epoch}".default = true;
    crossbeam_utils."${deps.crossbeam_deque."0.7.3".crossbeam_utils}".default = true;
    maybe_uninit."${deps.crossbeam_deque."0.7.3".maybe_uninit}".default = true;
  }) [
    (features_.crossbeam_epoch."${deps."crossbeam_deque"."0.7.3"."crossbeam_epoch"}" deps)
    (features_.crossbeam_utils."${deps."crossbeam_deque"."0.7.3"."crossbeam_utils"}" deps)
    (features_.maybe_uninit."${deps."crossbeam_deque"."0.7.3"."maybe_uninit"}" deps)
  ];


# end
# crossbeam-epoch-0.8.2

  crates.crossbeam_epoch."0.8.2" = deps: { features?(features_.crossbeam_epoch."0.8.2" deps {}) }: buildRustCrate {
    crateName = "crossbeam-epoch";
    version = "0.8.2";
    description = "Epoch-based garbage collection";
    authors = [ "The Crossbeam Project Developers" ];
    sha256 = "050dkgjrxgag2lj2zwxqdaz72y4kjpqr2pa36nm40szx8crfhq3v";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."crossbeam_epoch"."0.8.2"."cfg_if"}" deps)
      (crates."crossbeam_utils"."${deps."crossbeam_epoch"."0.8.2"."crossbeam_utils"}" deps)
      (crates."maybe_uninit"."${deps."crossbeam_epoch"."0.8.2"."maybe_uninit"}" deps)
      (crates."memoffset"."${deps."crossbeam_epoch"."0.8.2"."memoffset"}" deps)
      (crates."scopeguard"."${deps."crossbeam_epoch"."0.8.2"."scopeguard"}" deps)
    ]
      ++ (if features.crossbeam_epoch."0.8.2".lazy_static or false then [ (crates.lazy_static."${deps."crossbeam_epoch"."0.8.2".lazy_static}" deps) ] else []));

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."crossbeam_epoch"."0.8.2"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."crossbeam_epoch"."0.8.2" or {});
  };
  features_.crossbeam_epoch."0.8.2" = deps: f: updateFeatures f (rec {
    autocfg."${deps.crossbeam_epoch."0.8.2".autocfg}".default = true;
    cfg_if."${deps.crossbeam_epoch."0.8.2".cfg_if}".default = true;
    crossbeam_epoch = fold recursiveUpdate {} [
      { "0.8.2"."lazy_static" =
        (f.crossbeam_epoch."0.8.2"."lazy_static" or false) ||
        (f.crossbeam_epoch."0.8.2".std or false) ||
        (crossbeam_epoch."0.8.2"."std" or false); }
      { "0.8.2"."std" =
        (f.crossbeam_epoch."0.8.2"."std" or false) ||
        (f.crossbeam_epoch."0.8.2".default or false) ||
        (crossbeam_epoch."0.8.2"."default" or false); }
      { "0.8.2".default = (f.crossbeam_epoch."0.8.2".default or true); }
    ];
    crossbeam_utils = fold recursiveUpdate {} [
      { "${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."alloc" =
        (f.crossbeam_utils."${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."alloc" or false) ||
        (crossbeam_epoch."0.8.2"."alloc" or false) ||
        (f."crossbeam_epoch"."0.8.2"."alloc" or false); }
      { "${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."nightly" =
        (f.crossbeam_utils."${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."nightly" or false) ||
        (crossbeam_epoch."0.8.2"."nightly" or false) ||
        (f."crossbeam_epoch"."0.8.2"."nightly" or false); }
      { "${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."std" =
        (f.crossbeam_utils."${deps.crossbeam_epoch."0.8.2".crossbeam_utils}"."std" or false) ||
        (crossbeam_epoch."0.8.2"."std" or false) ||
        (f."crossbeam_epoch"."0.8.2"."std" or false); }
      { "${deps.crossbeam_epoch."0.8.2".crossbeam_utils}".default = (f.crossbeam_utils."${deps.crossbeam_epoch."0.8.2".crossbeam_utils}".default or false); }
    ];
    lazy_static."${deps.crossbeam_epoch."0.8.2".lazy_static}".default = true;
    maybe_uninit."${deps.crossbeam_epoch."0.8.2".maybe_uninit}".default = true;
    memoffset."${deps.crossbeam_epoch."0.8.2".memoffset}".default = true;
    scopeguard."${deps.crossbeam_epoch."0.8.2".scopeguard}".default = (f.scopeguard."${deps.crossbeam_epoch."0.8.2".scopeguard}".default or false);
  }) [
    (features_.cfg_if."${deps."crossbeam_epoch"."0.8.2"."cfg_if"}" deps)
    (features_.crossbeam_utils."${deps."crossbeam_epoch"."0.8.2"."crossbeam_utils"}" deps)
    (features_.lazy_static."${deps."crossbeam_epoch"."0.8.2"."lazy_static"}" deps)
    (features_.maybe_uninit."${deps."crossbeam_epoch"."0.8.2"."maybe_uninit"}" deps)
    (features_.memoffset."${deps."crossbeam_epoch"."0.8.2"."memoffset"}" deps)
    (features_.scopeguard."${deps."crossbeam_epoch"."0.8.2"."scopeguard"}" deps)
    (features_.autocfg."${deps."crossbeam_epoch"."0.8.2"."autocfg"}" deps)
  ];


# end
# crossbeam-utils-0.7.2

  crates.crossbeam_utils."0.7.2" = deps: { features?(features_.crossbeam_utils."0.7.2" deps {}) }: buildRustCrate {
    crateName = "crossbeam-utils";
    version = "0.7.2";
    description = "Utilities for concurrent programming";
    authors = [ "The Crossbeam Project Developers" ];
    sha256 = "17n0299c5y4d9pv4zr72shlx6klc0kl3mqmdgrvh70yg4bjr3837";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."crossbeam_utils"."0.7.2"."cfg_if"}" deps)
    ]
      ++ (if features.crossbeam_utils."0.7.2".lazy_static or false then [ (crates.lazy_static."${deps."crossbeam_utils"."0.7.2".lazy_static}" deps) ] else []));

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."crossbeam_utils"."0.7.2"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."crossbeam_utils"."0.7.2" or {});
  };
  features_.crossbeam_utils."0.7.2" = deps: f: updateFeatures f (rec {
    autocfg."${deps.crossbeam_utils."0.7.2".autocfg}".default = true;
    cfg_if."${deps.crossbeam_utils."0.7.2".cfg_if}".default = true;
    crossbeam_utils = fold recursiveUpdate {} [
      { "0.7.2"."lazy_static" =
        (f.crossbeam_utils."0.7.2"."lazy_static" or false) ||
        (f.crossbeam_utils."0.7.2".std or false) ||
        (crossbeam_utils."0.7.2"."std" or false); }
      { "0.7.2"."std" =
        (f.crossbeam_utils."0.7.2"."std" or false) ||
        (f.crossbeam_utils."0.7.2".default or false) ||
        (crossbeam_utils."0.7.2"."default" or false); }
      { "0.7.2".default = (f.crossbeam_utils."0.7.2".default or true); }
    ];
    lazy_static."${deps.crossbeam_utils."0.7.2".lazy_static}".default = true;
  }) [
    (features_.cfg_if."${deps."crossbeam_utils"."0.7.2"."cfg_if"}" deps)
    (features_.lazy_static."${deps."crossbeam_utils"."0.7.2"."lazy_static"}" deps)
    (features_.autocfg."${deps."crossbeam_utils"."0.7.2"."autocfg"}" deps)
  ];


# end
# digest-0.8.1

  crates.digest."0.8.1" = deps: { features?(features_.digest."0.8.1" deps {}) }: buildRustCrate {
    crateName = "digest";
    version = "0.8.1";
    description = "Traits for cryptographic hash functions";
    authors = [ "RustCrypto Developers" ];
    sha256 = "18jzwdsfl90bzhbk5ny4rgakhwn3l7pqk2mmqvl4ccb0qy4lhbyr";
    dependencies = mapFeatures features ([
      (crates."generic_array"."${deps."digest"."0.8.1"."generic_array"}" deps)
    ]);
    features = mkFeatures (features."digest"."0.8.1" or {});
  };
  features_.digest."0.8.1" = deps: f: updateFeatures f (rec {
    digest = fold recursiveUpdate {} [
      { "0.8.1"."blobby" =
        (f.digest."0.8.1"."blobby" or false) ||
        (f.digest."0.8.1".dev or false) ||
        (digest."0.8.1"."dev" or false); }
      { "0.8.1".default = (f.digest."0.8.1".default or true); }
    ];
    generic_array."${deps.digest."0.8.1".generic_array}".default = true;
  }) [
    (features_.generic_array."${deps."digest"."0.8.1"."generic_array"}" deps)
  ];


# end
# doc-comment-0.3.3

  crates.doc_comment."0.3.3" = deps: { features?(features_.doc_comment."0.3.3" deps {}) }: buildRustCrate {
    crateName = "doc-comment";
    version = "0.3.3";
    description = "Macro to generate doc comments";
    authors = [ "Guillaume Gomez <guillaume1.gomez@gmail.com>" ];
    sha256 = "1vn62nwly7h6s05zsn8k5h83110fxynj91v84nyv7czwq1zqam77";
    libName = "doc_comment";
    build = "build.rs";
    features = mkFeatures (features."doc_comment"."0.3.3" or {});
  };
  features_.doc_comment."0.3.3" = deps: f: updateFeatures f (rec {
    doc_comment."0.3.3".default = (f.doc_comment."0.3.3".default or true);
  }) [];


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
# fake-simd-0.1.2

  crates.fake_simd."0.1.2" = deps: { features?(features_.fake_simd."0.1.2" deps {}) }: buildRustCrate {
    crateName = "fake-simd";
    version = "0.1.2";
    description = "Crate for mimicking simd crate on stable Rust";
    authors = [ "The Rust-Crypto Project Developers" ];
    sha256 = "1a0f1j66nkwfy17s06vm2bn9vh8vy8llcijfhh9m10p58v08661a";
  };
  features_.fake_simd."0.1.2" = deps: f: updateFeatures f (rec {
    fake_simd."0.1.2".default = (f.fake_simd."0.1.2".default or true);
  }) [];


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
# futures-core-0.3.4

  crates.futures_core."0.3.4" = deps: { features?(features_.futures_core."0.3.4" deps {}) }: buildRustCrate {
    crateName = "futures-core";
    version = "0.3.4";
    description = "The core traits and types in for the `futures` library.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "03046fyq5s9qyfsary392jc1h65vdw4piya5ksnajd21g8ma6kdz";
    features = mkFeatures (features."futures_core"."0.3.4" or {});
  };
  features_.futures_core."0.3.4" = deps: f: updateFeatures f (rec {
    futures_core = fold recursiveUpdate {} [
      { "0.3.4"."alloc" =
        (f.futures_core."0.3.4"."alloc" or false) ||
        (f.futures_core."0.3.4".std or false) ||
        (futures_core."0.3.4"."std" or false); }
      { "0.3.4"."std" =
        (f.futures_core."0.3.4"."std" or false) ||
        (f.futures_core."0.3.4".default or false) ||
        (futures_core."0.3.4"."default" or false); }
      { "0.3.4".default = (f.futures_core."0.3.4".default or true); }
    ];
  }) [];


# end
# futures-io-0.3.4

  crates.futures_io."0.3.4" = deps: { features?(features_.futures_io."0.3.4" deps {}) }: buildRustCrate {
    crateName = "futures-io";
    version = "0.3.4";
    description = "The `AsyncRead`, `AsyncWrite`, `AsyncSeek`, and `AsyncBufRead` traits for the futures-rs library.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "0nmhb0lfw5h79qfkklr1hrrvpzz1cdnjq7xqq60qbii71b5mp7qk";
    features = mkFeatures (features."futures_io"."0.3.4" or {});
  };
  features_.futures_io."0.3.4" = deps: f: updateFeatures f (rec {
    futures_io = fold recursiveUpdate {} [
      { "0.3.4"."std" =
        (f.futures_io."0.3.4"."std" or false) ||
        (f.futures_io."0.3.4".default or false) ||
        (futures_io."0.3.4"."default" or false); }
      { "0.3.4".default = (f.futures_io."0.3.4".default or true); }
    ];
  }) [];


# end
# futures-timer-2.0.2

  crates.futures_timer."2.0.2" = deps: { features?(features_.futures_timer."2.0.2" deps {}) }: buildRustCrate {
    crateName = "futures-timer";
    version = "2.0.2";
    description = "Timeouts for futures.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "08l5pkyv5n4wbw4y5vwsx4r5r1v09xm5n4grkml684ci6826bcam";
  };
  features_.futures_timer."2.0.2" = deps: f: updateFeatures f (rec {
    futures_timer."2.0.2".default = (f.futures_timer."2.0.2".default or true);
  }) [];


# end
# generic-array-0.12.3

  crates.generic_array."0.12.3" = deps: { features?(features_.generic_array."0.12.3" deps {}) }: buildRustCrate {
    crateName = "generic-array";
    version = "0.12.3";
    description = "Generic types implementing functionality of arrays";
    authors = [ "Bartłomiej Kamiński <fizyk20@gmail.com>" "Aaron Trent <novacrazy@gmail.com>" ];
    sha256 = "1b6bvl1zsh5v9d85szkqfq4sw33xsw03mhchjk3zwxs29psg3nns";
    libName = "generic_array";
    dependencies = mapFeatures features ([
      (crates."typenum"."${deps."generic_array"."0.12.3"."typenum"}" deps)
    ]);
  };
  features_.generic_array."0.12.3" = deps: f: updateFeatures f (rec {
    generic_array."0.12.3".default = (f.generic_array."0.12.3".default or true);
    typenum."${deps.generic_array."0.12.3".typenum}".default = true;
  }) [
    (features_.typenum."${deps."generic_array"."0.12.3"."typenum"}" deps)
  ];


# end
# getrandom-0.1.14

  crates.getrandom."0.1.14" = deps: { features?(features_.getrandom."0.1.14" deps {}) }: buildRustCrate {
    crateName = "getrandom";
    version = "0.1.14";
    description = "A small cross-platform library for retrieving random data from system source";
    authors = [ "The Rand Project Developers" ];
    edition = "2018";
    sha256 = "1i6r4q7i24zdy6v5h3l966a1cf8a1aip2fi1pmdsi71sk1m3w7wr";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."getrandom"."0.1.14"."cfg_if"}" deps)
    ])
      ++ (if kernel == "wasi" then mapFeatures features ([
      (crates."wasi"."${deps."getrandom"."0.1.14"."wasi"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."getrandom"."0.1.14"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "wasm32-unknown-unknown" then mapFeatures features ([
]) else []);
    features = mkFeatures (features."getrandom"."0.1.14" or {});
  };
  features_.getrandom."0.1.14" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.getrandom."0.1.14".cfg_if}".default = true;
    getrandom = fold recursiveUpdate {} [
      { "0.1.14"."compiler_builtins" =
        (f.getrandom."0.1.14"."compiler_builtins" or false) ||
        (f.getrandom."0.1.14".rustc-dep-of-std or false) ||
        (getrandom."0.1.14"."rustc-dep-of-std" or false); }
      { "0.1.14"."core" =
        (f.getrandom."0.1.14"."core" or false) ||
        (f.getrandom."0.1.14".rustc-dep-of-std or false) ||
        (getrandom."0.1.14"."rustc-dep-of-std" or false); }
      { "0.1.14"."wasm-bindgen" =
        (f.getrandom."0.1.14"."wasm-bindgen" or false) ||
        (f.getrandom."0.1.14".test-in-browser or false) ||
        (getrandom."0.1.14"."test-in-browser" or false); }
      { "0.1.14".default = (f.getrandom."0.1.14".default or true); }
    ];
    libc."${deps.getrandom."0.1.14".libc}".default = (f.libc."${deps.getrandom."0.1.14".libc}".default or false);
    wasi."${deps.getrandom."0.1.14".wasi}".default = true;
  }) [
    (features_.cfg_if."${deps."getrandom"."0.1.14"."cfg_if"}" deps)
    (features_.wasi."${deps."getrandom"."0.1.14"."wasi"}" deps)
    (features_.libc."${deps."getrandom"."0.1.14"."libc"}" deps)
  ];


# end
# handlebars-3.0.1

  crates.handlebars."3.0.1" = deps: { features?(features_.handlebars."3.0.1" deps {}) }: buildRustCrate {
    crateName = "handlebars";
    version = "3.0.1";
    description = "Handlebars templating implemented in Rust.";
    authors = [ "Ning Sun <sunng@pm.me>" ];
    edition = "2018";
    sha256 = "176fqf1w22rbm24cypccb48rsbdvzillv8dmvfww0gr8ykkga1xh";
    libPath = "src/lib.rs";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."handlebars"."3.0.1"."log"}" deps)
      (crates."pest"."${deps."handlebars"."3.0.1"."pest"}" deps)
      (crates."pest_derive"."${deps."handlebars"."3.0.1"."pest_derive"}" deps)
      (crates."quick_error"."${deps."handlebars"."3.0.1"."quick_error"}" deps)
      (crates."serde"."${deps."handlebars"."3.0.1"."serde"}" deps)
      (crates."serde_json"."${deps."handlebars"."3.0.1"."serde_json"}" deps)
    ]);
    features = mkFeatures (features."handlebars"."3.0.1" or {});
  };
  features_.handlebars."3.0.1" = deps: f: updateFeatures f (rec {
    handlebars = fold recursiveUpdate {} [
      { "3.0.1"."walkdir" =
        (f.handlebars."3.0.1"."walkdir" or false) ||
        (f.handlebars."3.0.1".dir_source or false) ||
        (handlebars."3.0.1"."dir_source" or false); }
      { "3.0.1".default = (f.handlebars."3.0.1".default or true); }
    ];
    log."${deps.handlebars."3.0.1".log}".default = true;
    pest."${deps.handlebars."3.0.1".pest}".default = true;
    pest_derive."${deps.handlebars."3.0.1".pest_derive}".default = true;
    quick_error."${deps.handlebars."3.0.1".quick_error}".default = true;
    serde."${deps.handlebars."3.0.1".serde}".default = true;
    serde_json."${deps.handlebars."3.0.1".serde_json}".default = true;
  }) [
    (features_.log."${deps."handlebars"."3.0.1"."log"}" deps)
    (features_.pest."${deps."handlebars"."3.0.1"."pest"}" deps)
    (features_.pest_derive."${deps."handlebars"."3.0.1"."pest_derive"}" deps)
    (features_.quick_error."${deps."handlebars"."3.0.1"."quick_error"}" deps)
    (features_.serde."${deps."handlebars"."3.0.1"."serde"}" deps)
    (features_.serde_json."${deps."handlebars"."3.0.1"."serde_json"}" deps)
  ];


# end
# hermit-abi-0.1.12

  crates.hermit_abi."0.1.12" = deps: { features?(features_.hermit_abi."0.1.12" deps {}) }: buildRustCrate {
    crateName = "hermit-abi";
    version = "0.1.12";
    description = "hermit-abi is small interface to call functions from the unikernel RustyHermit.\nIt is used to build the target `x86_64-unknown-hermit`.\n";
    authors = [ "Stefan Lankes" ];
    sha256 = "0dm71xaxz2qakzpzrfjwk7ay6xivlqy1im7bf823is37frkm0hk3";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."hermit_abi"."0.1.12"."libc"}" deps)
    ]);
    features = mkFeatures (features."hermit_abi"."0.1.12" or {});
  };
  features_.hermit_abi."0.1.12" = deps: f: updateFeatures f (rec {
    hermit_abi = fold recursiveUpdate {} [
      { "0.1.12"."core" =
        (f.hermit_abi."0.1.12"."core" or false) ||
        (f.hermit_abi."0.1.12".rustc-dep-of-std or false) ||
        (hermit_abi."0.1.12"."rustc-dep-of-std" or false); }
      { "0.1.12".default = (f.hermit_abi."0.1.12".default or true); }
    ];
    libc = fold recursiveUpdate {} [
      { "${deps.hermit_abi."0.1.12".libc}"."rustc-dep-of-std" =
        (f.libc."${deps.hermit_abi."0.1.12".libc}"."rustc-dep-of-std" or false) ||
        (hermit_abi."0.1.12"."rustc-dep-of-std" or false) ||
        (f."hermit_abi"."0.1.12"."rustc-dep-of-std" or false); }
      { "${deps.hermit_abi."0.1.12".libc}".default = (f.libc."${deps.hermit_abi."0.1.12".libc}".default or false); }
    ];
  }) [
    (features_.libc."${deps."hermit_abi"."0.1.12"."libc"}" deps)
  ];


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
# idna-0.2.0

  crates.idna."0.2.0" = deps: { features?(features_.idna."0.2.0" deps {}) }: buildRustCrate {
    crateName = "idna";
    version = "0.2.0";
    description = "IDNA (Internationalizing Domain Names in Applications) and Punycode.";
    authors = [ "The rust-url developers" ];
    sha256 = "1mm05aq43qc5n492njnac5xn4rhiraii25xc0hwppr471jzijh8d";
    dependencies = mapFeatures features ([
      (crates."matches"."${deps."idna"."0.2.0"."matches"}" deps)
      (crates."unicode_bidi"."${deps."idna"."0.2.0"."unicode_bidi"}" deps)
      (crates."unicode_normalization"."${deps."idna"."0.2.0"."unicode_normalization"}" deps)
    ]);
  };
  features_.idna."0.2.0" = deps: f: updateFeatures f (rec {
    idna."0.2.0".default = (f.idna."0.2.0".default or true);
    matches."${deps.idna."0.2.0".matches}".default = true;
    unicode_bidi."${deps.idna."0.2.0".unicode_bidi}".default = true;
    unicode_normalization."${deps.idna."0.2.0".unicode_normalization}".default = true;
  }) [
    (features_.matches."${deps."idna"."0.2.0"."matches"}" deps)
    (features_.unicode_bidi."${deps."idna"."0.2.0"."unicode_bidi"}" deps)
    (features_.unicode_normalization."${deps."idna"."0.2.0"."unicode_normalization"}" deps)
  ];


# end
# iovec-0.1.4

  crates.iovec."0.1.4" = deps: { features?(features_.iovec."0.1.4" deps {}) }: buildRustCrate {
    crateName = "iovec";
    version = "0.1.4";
    description = "Portable buffer type for scatter/gather I/O operations\n";
    authors = [ "Carl Lerche <me@carllerche.com>" ];
    sha256 = "1wy7rsm8rx6y4rjy98jws1aqxdy0v5wbz9whz73p45cwpsg4prfa";
    dependencies = (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."iovec"."0.1.4"."libc"}" deps)
    ]) else []);
  };
  features_.iovec."0.1.4" = deps: f: updateFeatures f (rec {
    iovec."0.1.4".default = (f.iovec."0.1.4".default or true);
    libc."${deps.iovec."0.1.4".libc}".default = true;
  }) [
    (features_.libc."${deps."iovec"."0.1.4"."libc"}" deps)
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
# kv-log-macro-1.0.4

  crates.kv_log_macro."1.0.4" = deps: { features?(features_.kv_log_macro."1.0.4" deps {}) }: buildRustCrate {
    crateName = "kv-log-macro";
    version = "1.0.4";
    description = "Log macro for log's kv-unstable backend.";
    authors = [ "Yoshua Wuyts <yoshuawuyts@gmail.com>" ];
    edition = "2018";
    sha256 = "0s0q0zghvlb68ipkvgnihazfk0rp8fmds8p3fmbzfrpqmdw48k76";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."kv_log_macro"."1.0.4"."log"}" deps)
    ]);
  };
  features_.kv_log_macro."1.0.4" = deps: f: updateFeatures f (rec {
    kv_log_macro."1.0.4".default = (f.kv_log_macro."1.0.4".default or true);
    log = fold recursiveUpdate {} [
      { "${deps.kv_log_macro."1.0.4".log}"."kv_unstable" = true; }
      { "${deps.kv_log_macro."1.0.4".log}".default = true; }
    ];
  }) [
    (features_.log."${deps."kv_log_macro"."1.0.4"."log"}" deps)
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
# lapin-1.0.0-beta3

  crates.lapin."1.0.0-beta3" = deps: { features?(features_.lapin."1.0.0-beta3" deps {}) }: buildRustCrate {
    crateName = "lapin";
    version = "1.0.0-beta3";
    description = "AMQP client library";
    authors = [ "Geoffroy Couprie <geo.couprie@gmail.com>" "Marc-Antoine Perennou <Marc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "0lgjgzn9y33kwxaff05qm8m9fybqkm11f8a1j8qp842a2h7h5jrq";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."amq_protocol"."${deps."lapin"."1.0.0-beta3"."amq_protocol"}" deps)
      (crates."async_task"."${deps."lapin"."1.0.0-beta3"."async_task"}" deps)
      (crates."crossbeam_channel"."${deps."lapin"."1.0.0-beta3"."crossbeam_channel"}" deps)
      (crates."futures_core"."${deps."lapin"."1.0.0-beta3"."futures_core"}" deps)
      (crates."log"."${deps."lapin"."1.0.0-beta3"."log"}" deps)
      (crates."mio"."${deps."lapin"."1.0.0-beta3"."mio"}" deps)
      (crates."parking_lot"."${deps."lapin"."1.0.0-beta3"."parking_lot"}" deps)
      (crates."pinky_swear"."${deps."lapin"."1.0.0-beta3"."pinky_swear"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."amq_protocol_codegen"."${deps."lapin"."1.0.0-beta3"."amq_protocol_codegen"}" deps)
      (crates."serde_json"."${deps."lapin"."1.0.0-beta3"."serde_json"}" deps)
    ]);
    features = mkFeatures (features."lapin"."1.0.0-beta3" or {});
  };
  features_.lapin."1.0.0-beta3" = deps: f: updateFeatures f (rec {
    amq_protocol = fold recursiveUpdate {} [
      { "${deps.lapin."1.0.0-beta3".amq_protocol}"."native-tls" =
        (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}"."native-tls" or false) ||
        (lapin."1.0.0-beta3"."native-tls" or false) ||
        (f."lapin"."1.0.0-beta3"."native-tls" or false); }
      { "${deps.lapin."1.0.0-beta3".amq_protocol}"."openssl" =
        (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}"."openssl" or false) ||
        (lapin."1.0.0-beta3"."openssl" or false) ||
        (f."lapin"."1.0.0-beta3"."openssl" or false); }
      { "${deps.lapin."1.0.0-beta3".amq_protocol}"."rustls-native-certs" =
        (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}"."rustls-native-certs" or false) ||
        (lapin."1.0.0-beta3"."rustls-native-certs" or false) ||
        (f."lapin"."1.0.0-beta3"."rustls-native-certs" or false); }
      { "${deps.lapin."1.0.0-beta3".amq_protocol}"."rustls-webpki-roots-certs" =
        (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}"."rustls-webpki-roots-certs" or false) ||
        (lapin."1.0.0-beta3"."rustls-webpki-roots-certs" or false) ||
        (f."lapin"."1.0.0-beta3"."rustls-webpki-roots-certs" or false); }
      { "${deps.lapin."1.0.0-beta3".amq_protocol}"."vendored-openssl" =
        (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}"."vendored-openssl" or false) ||
        (lapin."1.0.0-beta3"."vendored-openssl" or false) ||
        (f."lapin"."1.0.0-beta3"."vendored-openssl" or false); }
      { "${deps.lapin."1.0.0-beta3".amq_protocol}".default = (f.amq_protocol."${deps.lapin."1.0.0-beta3".amq_protocol}".default or false); }
    ];
    amq_protocol_codegen."${deps.lapin."1.0.0-beta3".amq_protocol_codegen}".default = true;
    async_task."${deps.lapin."1.0.0-beta3".async_task}".default = true;
    crossbeam_channel."${deps.lapin."1.0.0-beta3".crossbeam_channel}".default = true;
    futures_core."${deps.lapin."1.0.0-beta3".futures_core}".default = true;
    lapin = fold recursiveUpdate {} [
      { "1.0.0-beta3"."native-tls" =
        (f.lapin."1.0.0-beta3"."native-tls" or false) ||
        (f.lapin."1.0.0-beta3".default or false) ||
        (lapin."1.0.0-beta3"."default" or false); }
      { "1.0.0-beta3"."rustls-native-certs" =
        (f.lapin."1.0.0-beta3"."rustls-native-certs" or false) ||
        (f.lapin."1.0.0-beta3".rustls or false) ||
        (lapin."1.0.0-beta3"."rustls" or false); }
      { "1.0.0-beta3".default = (f.lapin."1.0.0-beta3".default or true); }
    ];
    log."${deps.lapin."1.0.0-beta3".log}".default = true;
    mio = fold recursiveUpdate {} [
      { "${deps.lapin."1.0.0-beta3".mio}"."os-poll" = true; }
      { "${deps.lapin."1.0.0-beta3".mio}"."tcp" = true; }
      { "${deps.lapin."1.0.0-beta3".mio}".default = true; }
    ];
    parking_lot."${deps.lapin."1.0.0-beta3".parking_lot}".default = true;
    pinky_swear."${deps.lapin."1.0.0-beta3".pinky_swear}".default = true;
    serde_json."${deps.lapin."1.0.0-beta3".serde_json}".default = true;
  }) [
    (features_.amq_protocol."${deps."lapin"."1.0.0-beta3"."amq_protocol"}" deps)
    (features_.async_task."${deps."lapin"."1.0.0-beta3"."async_task"}" deps)
    (features_.crossbeam_channel."${deps."lapin"."1.0.0-beta3"."crossbeam_channel"}" deps)
    (features_.futures_core."${deps."lapin"."1.0.0-beta3"."futures_core"}" deps)
    (features_.log."${deps."lapin"."1.0.0-beta3"."log"}" deps)
    (features_.mio."${deps."lapin"."1.0.0-beta3"."mio"}" deps)
    (features_.parking_lot."${deps."lapin"."1.0.0-beta3"."parking_lot"}" deps)
    (features_.pinky_swear."${deps."lapin"."1.0.0-beta3"."pinky_swear"}" deps)
    (features_.amq_protocol_codegen."${deps."lapin"."1.0.0-beta3"."amq_protocol_codegen"}" deps)
    (features_.serde_json."${deps."lapin"."1.0.0-beta3"."serde_json"}" deps)
  ];


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
# lazy_static-1.4.0

  crates.lazy_static."1.4.0" = deps: { features?(features_.lazy_static."1.4.0" deps {}) }: buildRustCrate {
    crateName = "lazy_static";
    version = "1.4.0";
    description = "A macro for declaring lazily evaluated statics in Rust.";
    authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
    sha256 = "13h6sdghdcy7vcqsm2gasfw3qg7ssa0fl3sw7lq6pdkbk52wbyfr";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."lazy_static"."1.4.0" or {});
  };
  features_.lazy_static."1.4.0" = deps: f: updateFeatures f (rec {
    lazy_static = fold recursiveUpdate {} [
      { "1.4.0"."spin" =
        (f.lazy_static."1.4.0"."spin" or false) ||
        (f.lazy_static."1.4.0".spin_no_std or false) ||
        (lazy_static."1.4.0"."spin_no_std" or false); }
      { "1.4.0".default = (f.lazy_static."1.4.0".default or true); }
    ];
  }) [];


# end
# lexical-core-0.7.4

  crates.lexical_core."0.7.4" = deps: { features?(features_.lexical_core."0.7.4" deps {}) }: buildRustCrate {
    crateName = "lexical-core";
    version = "0.7.4";
    description = "Lexical, to- and from-string conversion routines.";
    authors = [ "Alex Huszagh <ahuszagh@gmail.com>" ];
    edition = "2018";
    sha256 = "1afy5hyajdrh0yi6zas62bsazz4zxmplik8xxmsfdbin7yff997k";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."lexical_core"."0.7.4"."bitflags"}" deps)
      (crates."cfg_if"."${deps."lexical_core"."0.7.4"."cfg_if"}" deps)
    ]
      ++ (if features.lexical_core."0.7.4".arrayvec or false then [ (crates.arrayvec."${deps."lexical_core"."0.7.4".arrayvec}" deps) ] else [])
      ++ (if features.lexical_core."0.7.4".ryu or false then [ (crates.ryu."${deps."lexical_core"."0.7.4".ryu}" deps) ] else [])
      ++ (if features.lexical_core."0.7.4".static_assertions or false then [ (crates.static_assertions."${deps."lexical_core"."0.7.4".static_assertions}" deps) ] else []));
    features = mkFeatures (features."lexical_core"."0.7.4" or {});
  };
  features_.lexical_core."0.7.4" = deps: f: updateFeatures f (rec {
    arrayvec = fold recursiveUpdate {} [
      { "${deps.lexical_core."0.7.4".arrayvec}"."array-sizes-33-128" = true; }
      { "${deps.lexical_core."0.7.4".arrayvec}".default = true; }
    ];
    bitflags."${deps.lexical_core."0.7.4".bitflags}".default = true;
    cfg_if."${deps.lexical_core."0.7.4".cfg_if}".default = true;
    lexical_core = fold recursiveUpdate {} [
      { "0.7.4"."arrayvec" =
        (f.lexical_core."0.7.4"."arrayvec" or false) ||
        (f.lexical_core."0.7.4".correct or false) ||
        (lexical_core."0.7.4"."correct" or false); }
      { "0.7.4"."correct" =
        (f.lexical_core."0.7.4"."correct" or false) ||
        (f.lexical_core."0.7.4".default or false) ||
        (lexical_core."0.7.4"."default" or false); }
      { "0.7.4"."dtoa" =
        (f.lexical_core."0.7.4"."dtoa" or false) ||
        (f.lexical_core."0.7.4".grisu3 or false) ||
        (lexical_core."0.7.4"."grisu3" or false); }
      { "0.7.4"."ryu" =
        (f.lexical_core."0.7.4"."ryu" or false) ||
        (f.lexical_core."0.7.4".default or false) ||
        (lexical_core."0.7.4"."default" or false); }
      { "0.7.4"."static_assertions" =
        (f.lexical_core."0.7.4"."static_assertions" or false) ||
        (f.lexical_core."0.7.4".correct or false) ||
        (lexical_core."0.7.4"."correct" or false) ||
        (f.lexical_core."0.7.4".format or false) ||
        (lexical_core."0.7.4"."format" or false); }
      { "0.7.4"."std" =
        (f.lexical_core."0.7.4"."std" or false) ||
        (f.lexical_core."0.7.4".default or false) ||
        (lexical_core."0.7.4"."default" or false); }
      { "0.7.4"."table" =
        (f.lexical_core."0.7.4"."table" or false) ||
        (f.lexical_core."0.7.4".correct or false) ||
        (lexical_core."0.7.4"."correct" or false); }
      { "0.7.4".default = (f.lexical_core."0.7.4".default or true); }
    ];
    ryu."${deps.lexical_core."0.7.4".ryu}".default = true;
    static_assertions."${deps.lexical_core."0.7.4".static_assertions}".default = true;
  }) [
    (features_.arrayvec."${deps."lexical_core"."0.7.4"."arrayvec"}" deps)
    (features_.bitflags."${deps."lexical_core"."0.7.4"."bitflags"}" deps)
    (features_.cfg_if."${deps."lexical_core"."0.7.4"."cfg_if"}" deps)
    (features_.ryu."${deps."lexical_core"."0.7.4"."ryu"}" deps)
    (features_.static_assertions."${deps."lexical_core"."0.7.4"."static_assertions"}" deps)
  ];


# end
# libc-0.2.69

  crates.libc."0.2.69" = deps: { features?(features_.libc."0.2.69" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.69";
    description = "Raw FFI bindings to platform libraries like libc.\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0fwi6rxklsaqcig432fg3cjamiilvv2c4jz0i3dxw7c33ipprhsz";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."libc"."0.2.69" or {});
  };
  features_.libc."0.2.69" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.69"."align" =
        (f.libc."0.2.69"."align" or false) ||
        (f.libc."0.2.69".rustc-dep-of-std or false) ||
        (libc."0.2.69"."rustc-dep-of-std" or false); }
      { "0.2.69"."rustc-std-workspace-core" =
        (f.libc."0.2.69"."rustc-std-workspace-core" or false) ||
        (f.libc."0.2.69".rustc-dep-of-std or false) ||
        (libc."0.2.69"."rustc-dep-of-std" or false); }
      { "0.2.69"."std" =
        (f.libc."0.2.69"."std" or false) ||
        (f.libc."0.2.69".default or false) ||
        (libc."0.2.69"."default" or false) ||
        (f.libc."0.2.69".use_std or false) ||
        (libc."0.2.69"."use_std" or false); }
      { "0.2.69".default = (f.libc."0.2.69".default or true); }
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
# lock_api-0.3.4

  crates.lock_api."0.3.4" = deps: { features?(features_.lock_api."0.3.4" deps {}) }: buildRustCrate {
    crateName = "lock_api";
    version = "0.3.4";
    description = "Wrappers to create fully-featured Mutex and RwLock types. Compatible with no_std.";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    edition = "2018";
    sha256 = "1wcx8y20igp1qnqh5vckrcz4xl2bsxi9p8ydcbssp6na41084pdv";
    dependencies = mapFeatures features ([
      (crates."scopeguard"."${deps."lock_api"."0.3.4"."scopeguard"}" deps)
    ]);
    features = mkFeatures (features."lock_api"."0.3.4" or {});
  };
  features_.lock_api."0.3.4" = deps: f: updateFeatures f (rec {
    lock_api."0.3.4".default = (f.lock_api."0.3.4".default or true);
    scopeguard."${deps.lock_api."0.3.4".scopeguard}".default = (f.scopeguard."${deps.lock_api."0.3.4".scopeguard}".default or false);
  }) [
    (features_.scopeguard."${deps."lock_api"."0.3.4"."scopeguard"}" deps)
  ];


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
# log-0.4.8

  crates.log."0.4.8" = deps: { features?(features_.log."0.4.8" deps {}) }: buildRustCrate {
    crateName = "log";
    version = "0.4.8";
    description = "A lightweight logging facade for Rust\n";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0wvzzzcn89dai172rrqcyz06pzldyyy0lf0w71csmn206rdpnb15";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."log"."0.4.8"."cfg_if"}" deps)
    ]);
    features = mkFeatures (features."log"."0.4.8" or {});
  };
  features_.log."0.4.8" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.log."0.4.8".cfg_if}".default = true;
    log = fold recursiveUpdate {} [
      { "0.4.8"."kv_unstable" =
        (f.log."0.4.8"."kv_unstable" or false) ||
        (f.log."0.4.8".kv_unstable_sval or false) ||
        (log."0.4.8"."kv_unstable_sval" or false); }
      { "0.4.8".default = (f.log."0.4.8".default or true); }
    ];
  }) [
    (features_.cfg_if."${deps."log"."0.4.8"."cfg_if"}" deps)
  ];


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
# maplit-1.0.2

  crates.maplit."1.0.2" = deps: { features?(features_.maplit."1.0.2" deps {}) }: buildRustCrate {
    crateName = "maplit";
    version = "1.0.2";
    description = "Collection “literal” macros for HashMap, HashSet, BTreeMap, and BTreeSet.";
    authors = [ "bluss" ];
    sha256 = "1zkg0klbbqdxf5wlz2d961pk4xm7bw6d6yhlv54mg3phly2ri9fv";
  };
  features_.maplit."1.0.2" = deps: f: updateFeatures f (rec {
    maplit."1.0.2".default = (f.maplit."1.0.2".default or true);
  }) [];


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
# maybe-uninit-2.0.0

  crates.maybe_uninit."2.0.0" = deps: { features?(features_.maybe_uninit."2.0.0" deps {}) }: buildRustCrate {
    crateName = "maybe-uninit";
    version = "2.0.0";
    description = "MaybeUninit for friends of backwards compatibility";
    authors = [ "est31 <MTest31@outlook.com>" "The Rust Project Developers" ];
    sha256 = "0crrwlngxjswhcnw8dvsccx8qnm2cbp4fvq6xhz3akmznvnv77gk";
  };
  features_.maybe_uninit."2.0.0" = deps: f: updateFeatures f (rec {
    maybe_uninit."2.0.0".default = (f.maybe_uninit."2.0.0".default or true);
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
# memchr-2.3.3

  crates.memchr."2.3.3" = deps: { features?(features_.memchr."2.3.3" deps {}) }: buildRustCrate {
    crateName = "memchr";
    version = "2.3.3";
    description = "Safe interface to memchr.";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
    sha256 = "1ivxvlswglk6wd46gadkbbsknr94gwryk6y21v64ja7x4icrpihw";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."memchr"."2.3.3" or {});
  };
  features_.memchr."2.3.3" = deps: f: updateFeatures f (rec {
    memchr = fold recursiveUpdate {} [
      { "2.3.3"."std" =
        (f.memchr."2.3.3"."std" or false) ||
        (f.memchr."2.3.3".default or false) ||
        (memchr."2.3.3"."default" or false) ||
        (f.memchr."2.3.3".use_std or false) ||
        (memchr."2.3.3"."use_std" or false); }
      { "2.3.3".default = (f.memchr."2.3.3".default or true); }
    ];
  }) [];


# end
# memoffset-0.5.4

  crates.memoffset."0.5.4" = deps: { features?(features_.memoffset."0.5.4" deps {}) }: buildRustCrate {
    crateName = "memoffset";
    version = "0.5.4";
    description = "offset_of functionality for Rust structs.";
    authors = [ "Gilad Naaman <gilad.naaman@gmail.com>" ];
    sha256 = "1c0bbna4ji5brc5jxdmkv39lxp1hnlp1b8yanigk1xj8k0929p7c";

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."memoffset"."0.5.4"."autocfg"}" deps)
    ]);
    features = mkFeatures (features."memoffset"."0.5.4" or {});
  };
  features_.memoffset."0.5.4" = deps: f: updateFeatures f (rec {
    autocfg."${deps.memoffset."0.5.4".autocfg}".default = true;
    memoffset."0.5.4".default = (f.memoffset."0.5.4".default or true);
  }) [
    (features_.autocfg."${deps."memoffset"."0.5.4"."autocfg"}" deps)
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
# mio-0.6.21

  crates.mio."0.6.21" = deps: { features?(features_.mio."0.6.21" deps {}) }: buildRustCrate {
    crateName = "mio";
    version = "0.6.21";
    description = "Lightweight non-blocking IO";
    authors = [ "Carl Lerche <me@carllerche.com>" ];
    sha256 = "08z31q5fx4irmp3hsvlzqy541swda8ixhs69adm95j97xz5ikmys";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."mio"."0.6.21"."cfg_if"}" deps)
      (crates."iovec"."${deps."mio"."0.6.21"."iovec"}" deps)
      (crates."log"."${deps."mio"."0.6.21"."log"}" deps)
      (crates."net2"."${deps."mio"."0.6.21"."net2"}" deps)
      (crates."slab"."${deps."mio"."0.6.21"."slab"}" deps)
    ])
      ++ (if kernel == "fuchsia" then mapFeatures features ([
      (crates."fuchsia_zircon"."${deps."mio"."0.6.21"."fuchsia_zircon"}" deps)
      (crates."fuchsia_zircon_sys"."${deps."mio"."0.6.21"."fuchsia_zircon_sys"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."mio"."0.6.21"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."kernel32_sys"."${deps."mio"."0.6.21"."kernel32_sys"}" deps)
      (crates."miow"."${deps."mio"."0.6.21"."miow"}" deps)
      (crates."winapi"."${deps."mio"."0.6.21"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."mio"."0.6.21" or {});
  };
  features_.mio."0.6.21" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.mio."0.6.21".cfg_if}".default = true;
    fuchsia_zircon."${deps.mio."0.6.21".fuchsia_zircon}".default = true;
    fuchsia_zircon_sys."${deps.mio."0.6.21".fuchsia_zircon_sys}".default = true;
    iovec."${deps.mio."0.6.21".iovec}".default = true;
    kernel32_sys."${deps.mio."0.6.21".kernel32_sys}".default = true;
    libc."${deps.mio."0.6.21".libc}".default = true;
    log."${deps.mio."0.6.21".log}".default = true;
    mio = fold recursiveUpdate {} [
      { "0.6.21"."with-deprecated" =
        (f.mio."0.6.21"."with-deprecated" or false) ||
        (f.mio."0.6.21".default or false) ||
        (mio."0.6.21"."default" or false); }
      { "0.6.21".default = (f.mio."0.6.21".default or true); }
    ];
    miow."${deps.mio."0.6.21".miow}".default = true;
    net2."${deps.mio."0.6.21".net2}".default = true;
    slab."${deps.mio."0.6.21".slab}".default = true;
    winapi."${deps.mio."0.6.21".winapi}".default = true;
  }) [
    (features_.cfg_if."${deps."mio"."0.6.21"."cfg_if"}" deps)
    (features_.iovec."${deps."mio"."0.6.21"."iovec"}" deps)
    (features_.log."${deps."mio"."0.6.21"."log"}" deps)
    (features_.net2."${deps."mio"."0.6.21"."net2"}" deps)
    (features_.slab."${deps."mio"."0.6.21"."slab"}" deps)
    (features_.fuchsia_zircon."${deps."mio"."0.6.21"."fuchsia_zircon"}" deps)
    (features_.fuchsia_zircon_sys."${deps."mio"."0.6.21"."fuchsia_zircon_sys"}" deps)
    (features_.libc."${deps."mio"."0.6.21"."libc"}" deps)
    (features_.kernel32_sys."${deps."mio"."0.6.21"."kernel32_sys"}" deps)
    (features_.miow."${deps."mio"."0.6.21"."miow"}" deps)
    (features_.winapi."${deps."mio"."0.6.21"."winapi"}" deps)
  ];


# end
# mio-0.7.0

  crates.mio."0.7.0" = deps: { features?(features_.mio."0.7.0" deps {}) }: buildRustCrate {
    crateName = "mio";
    version = "0.7.0";
    description = "Lightweight non-blocking IO";
    authors = [ "Carl Lerche <me@carllerche.com>" ];
    edition = "2018";
    sha256 = "0pzvb5ycxjh04m9nhqfmma3qji8jhc4nppwg7m2yyzk1hy3pwg3d";
    dependencies = mapFeatures features ([
      (crates."log"."${deps."mio"."0.7.0"."log"}" deps)
    ])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."mio"."0.7.0"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."lazy_static"."${deps."mio"."0.7.0"."lazy_static"}" deps)
      (crates."miow"."${deps."mio"."0.7.0"."miow"}" deps)
      (crates."ntapi"."${deps."mio"."0.7.0"."ntapi"}" deps)
      (crates."winapi"."${deps."mio"."0.7.0"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."mio"."0.7.0" or {});
  };
  features_.mio."0.7.0" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.mio."0.7.0".lazy_static}".default = true;
    libc."${deps.mio."0.7.0".libc}".default = true;
    log."${deps.mio."0.7.0".log}".default = true;
    mio."0.7.0".default = (f.mio."0.7.0".default or true);
    miow."${deps.mio."0.7.0".miow}".default = true;
    ntapi."${deps.mio."0.7.0".ntapi}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.mio."0.7.0".winapi}"."mswsock" = true; }
      { "${deps.mio."0.7.0".winapi}"."winsock2" = true; }
      { "${deps.mio."0.7.0".winapi}".default = true; }
    ];
  }) [
    (features_.log."${deps."mio"."0.7.0"."log"}" deps)
    (features_.libc."${deps."mio"."0.7.0"."libc"}" deps)
    (features_.lazy_static."${deps."mio"."0.7.0"."lazy_static"}" deps)
    (features_.miow."${deps."mio"."0.7.0"."miow"}" deps)
    (features_.ntapi."${deps."mio"."0.7.0"."ntapi"}" deps)
    (features_.winapi."${deps."mio"."0.7.0"."winapi"}" deps)
  ];


# end
# mio-uds-0.6.7

  crates.mio_uds."0.6.7" = deps: { features?(features_.mio_uds."0.6.7" deps {}) }: buildRustCrate {
    crateName = "mio-uds";
    version = "0.6.7";
    description = "Unix domain socket bindings for mio\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "1gff9908pvvysv7zgxvyxy7x34fnhs088cr0j8mgwj8j24mswrhm";
    dependencies = (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."iovec"."${deps."mio_uds"."0.6.7"."iovec"}" deps)
      (crates."libc"."${deps."mio_uds"."0.6.7"."libc"}" deps)
      (crates."mio"."${deps."mio_uds"."0.6.7"."mio"}" deps)
    ]) else []);
  };
  features_.mio_uds."0.6.7" = deps: f: updateFeatures f (rec {
    iovec."${deps.mio_uds."0.6.7".iovec}".default = true;
    libc."${deps.mio_uds."0.6.7".libc}".default = true;
    mio."${deps.mio_uds."0.6.7".mio}".default = true;
    mio_uds."0.6.7".default = (f.mio_uds."0.6.7".default or true);
  }) [
    (features_.iovec."${deps."mio_uds"."0.6.7"."iovec"}" deps)
    (features_.libc."${deps."mio_uds"."0.6.7"."libc"}" deps)
    (features_.mio."${deps."mio_uds"."0.6.7"."mio"}" deps)
  ];


# end
# miow-0.2.1

  crates.miow."0.2.1" = deps: { features?(features_.miow."0.2.1" deps {}) }: buildRustCrate {
    crateName = "miow";
    version = "0.2.1";
    description = "A zero overhead I/O library for Windows, focusing on IOCP and Async I/O\nabstractions.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "14f8zkc6ix7mkyis1vsqnim8m29b6l55abkba3p2yz7j1ibcvrl0";
    dependencies = mapFeatures features ([
      (crates."kernel32_sys"."${deps."miow"."0.2.1"."kernel32_sys"}" deps)
      (crates."net2"."${deps."miow"."0.2.1"."net2"}" deps)
      (crates."winapi"."${deps."miow"."0.2.1"."winapi"}" deps)
      (crates."ws2_32_sys"."${deps."miow"."0.2.1"."ws2_32_sys"}" deps)
    ]);
  };
  features_.miow."0.2.1" = deps: f: updateFeatures f (rec {
    kernel32_sys."${deps.miow."0.2.1".kernel32_sys}".default = true;
    miow."0.2.1".default = (f.miow."0.2.1".default or true);
    net2."${deps.miow."0.2.1".net2}".default = (f.net2."${deps.miow."0.2.1".net2}".default or false);
    winapi."${deps.miow."0.2.1".winapi}".default = true;
    ws2_32_sys."${deps.miow."0.2.1".ws2_32_sys}".default = true;
  }) [
    (features_.kernel32_sys."${deps."miow"."0.2.1"."kernel32_sys"}" deps)
    (features_.net2."${deps."miow"."0.2.1"."net2"}" deps)
    (features_.winapi."${deps."miow"."0.2.1"."winapi"}" deps)
    (features_.ws2_32_sys."${deps."miow"."0.2.1"."ws2_32_sys"}" deps)
  ];


# end
# miow-0.3.3

  crates.miow."0.3.3" = deps: { features?(features_.miow."0.3.3" deps {}) }: buildRustCrate {
    crateName = "miow";
    version = "0.3.3";
    description = "A zero overhead I/O library for Windows, focusing on IOCP and Async I/O\nabstractions.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "1mlk5mn00cl6bmf8qlpc6r85dxf4l45vbkbzshsr1mrkb3hn1j57";
    dependencies = mapFeatures features ([
      (crates."socket2"."${deps."miow"."0.3.3"."socket2"}" deps)
      (crates."winapi"."${deps."miow"."0.3.3"."winapi"}" deps)
    ]);
  };
  features_.miow."0.3.3" = deps: f: updateFeatures f (rec {
    miow."0.3.3".default = (f.miow."0.3.3".default or true);
    socket2."${deps.miow."0.3.3".socket2}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.miow."0.3.3".winapi}"."fileapi" = true; }
      { "${deps.miow."0.3.3".winapi}"."handleapi" = true; }
      { "${deps.miow."0.3.3".winapi}"."ioapiset" = true; }
      { "${deps.miow."0.3.3".winapi}"."minwindef" = true; }
      { "${deps.miow."0.3.3".winapi}"."namedpipeapi" = true; }
      { "${deps.miow."0.3.3".winapi}"."ntdef" = true; }
      { "${deps.miow."0.3.3".winapi}"."std" = true; }
      { "${deps.miow."0.3.3".winapi}"."synchapi" = true; }
      { "${deps.miow."0.3.3".winapi}"."winerror" = true; }
      { "${deps.miow."0.3.3".winapi}"."winsock2" = true; }
      { "${deps.miow."0.3.3".winapi}"."ws2def" = true; }
      { "${deps.miow."0.3.3".winapi}"."ws2ipdef" = true; }
      { "${deps.miow."0.3.3".winapi}".default = true; }
    ];
  }) [
    (features_.socket2."${deps."miow"."0.3.3"."socket2"}" deps)
    (features_.winapi."${deps."miow"."0.3.3"."winapi"}" deps)
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
# native-tls-0.2.4

  crates.native_tls."0.2.4" = deps: { features?(features_.native_tls."0.2.4" deps {}) }: buildRustCrate {
    crateName = "native-tls";
    version = "0.2.4";
    description = "A wrapper over a platform's native TLS implementation";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "05da1ai360zkdflh47dbaja3v5d8x4wl23g4zi32hh4n5g5adrm5";
    dependencies = (if kernel == "darwin" || kernel == "ios" then mapFeatures features ([
      (crates."lazy_static"."${deps."native_tls"."0.2.4"."lazy_static"}" deps)
      (crates."libc"."${deps."native_tls"."0.2.4"."libc"}" deps)
      (crates."security_framework"."${deps."native_tls"."0.2.4"."security_framework"}" deps)
      (crates."security_framework_sys"."${deps."native_tls"."0.2.4"."security_framework_sys"}" deps)
      (crates."tempfile"."${deps."native_tls"."0.2.4"."tempfile"}" deps)
    ]) else [])
      ++ (if !(kernel == "windows" || kernel == "darwin" || kernel == "ios") then mapFeatures features ([
      (crates."log"."${deps."native_tls"."0.2.4"."log"}" deps)
      (crates."openssl"."${deps."native_tls"."0.2.4"."openssl"}" deps)
      (crates."openssl_probe"."${deps."native_tls"."0.2.4"."openssl_probe"}" deps)
      (crates."openssl_sys"."${deps."native_tls"."0.2.4"."openssl_sys"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."schannel"."${deps."native_tls"."0.2.4"."schannel"}" deps)
    ]) else []);
    features = mkFeatures (features."native_tls"."0.2.4" or {});
  };
  features_.native_tls."0.2.4" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.native_tls."0.2.4".lazy_static}".default = true;
    libc."${deps.native_tls."0.2.4".libc}".default = true;
    log."${deps.native_tls."0.2.4".log}".default = true;
    native_tls."0.2.4".default = (f.native_tls."0.2.4".default or true);
    openssl."${deps.native_tls."0.2.4".openssl}".default = true;
    openssl_probe."${deps.native_tls."0.2.4".openssl_probe}".default = true;
    openssl_sys."${deps.native_tls."0.2.4".openssl_sys}".default = true;
    schannel."${deps.native_tls."0.2.4".schannel}".default = true;
    security_framework."${deps.native_tls."0.2.4".security_framework}".default = true;
    security_framework_sys."${deps.native_tls."0.2.4".security_framework_sys}".default = true;
    tempfile."${deps.native_tls."0.2.4".tempfile}".default = true;
  }) [
    (features_.lazy_static."${deps."native_tls"."0.2.4"."lazy_static"}" deps)
    (features_.libc."${deps."native_tls"."0.2.4"."libc"}" deps)
    (features_.security_framework."${deps."native_tls"."0.2.4"."security_framework"}" deps)
    (features_.security_framework_sys."${deps."native_tls"."0.2.4"."security_framework_sys"}" deps)
    (features_.tempfile."${deps."native_tls"."0.2.4"."tempfile"}" deps)
    (features_.log."${deps."native_tls"."0.2.4"."log"}" deps)
    (features_.openssl."${deps."native_tls"."0.2.4"."openssl"}" deps)
    (features_.openssl_probe."${deps."native_tls"."0.2.4"."openssl_probe"}" deps)
    (features_.openssl_sys."${deps."native_tls"."0.2.4"."openssl_sys"}" deps)
    (features_.schannel."${deps."native_tls"."0.2.4"."schannel"}" deps)
  ];


# end
# net2-0.2.33

  crates.net2."0.2.33" = deps: { features?(features_.net2."0.2.33" deps {}) }: buildRustCrate {
    crateName = "net2";
    version = "0.2.33";
    description = "Extensions to the standard library's networking types as proposed in RFC 1158.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "1qnmajafgybj5wyxz9iffa8x5wgbwd2znfklmhqj7vl6lw1m65mq";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."net2"."0.2.33"."cfg_if"}" deps)
    ])
      ++ (if kernel == "redox" || (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."net2"."0.2.33"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."net2"."0.2.33"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."net2"."0.2.33" or {});
  };
  features_.net2."0.2.33" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.net2."0.2.33".cfg_if}".default = true;
    libc."${deps.net2."0.2.33".libc}".default = true;
    net2 = fold recursiveUpdate {} [
      { "0.2.33"."duration" =
        (f.net2."0.2.33"."duration" or false) ||
        (f.net2."0.2.33".default or false) ||
        (net2."0.2.33"."default" or false); }
      { "0.2.33".default = (f.net2."0.2.33".default or true); }
    ];
    winapi = fold recursiveUpdate {} [
      { "${deps.net2."0.2.33".winapi}"."handleapi" = true; }
      { "${deps.net2."0.2.33".winapi}"."winsock2" = true; }
      { "${deps.net2."0.2.33".winapi}"."ws2def" = true; }
      { "${deps.net2."0.2.33".winapi}"."ws2ipdef" = true; }
      { "${deps.net2."0.2.33".winapi}"."ws2tcpip" = true; }
      { "${deps.net2."0.2.33".winapi}".default = true; }
    ];
  }) [
    (features_.cfg_if."${deps."net2"."0.2.33"."cfg_if"}" deps)
    (features_.libc."${deps."net2"."0.2.33"."libc"}" deps)
    (features_.winapi."${deps."net2"."0.2.33"."winapi"}" deps)
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
# nom-6.0.0-alpha1

  crates.nom."6.0.0-alpha1" = deps: { features?(features_.nom."6.0.0-alpha1" deps {}) }: buildRustCrate {
    crateName = "nom";
    version = "6.0.0-alpha1";
    description = "A byte-oriented, zero-copy, parser combinators library";
    authors = [ "contact@geoffroycouprie.com" ];
    edition = "2018";
    sha256 = "1m0lxczl72a4ih1rkp0pdnpfndbl4xcxgcaadfvmzn5bqs1953lr";
    dependencies = mapFeatures features ([
      (crates."memchr"."${deps."nom"."6.0.0-alpha1"."memchr"}" deps)
    ]
      ++ (if features.nom."6.0.0-alpha1".lexical-core or false then [ (crates.lexical_core."${deps."nom"."6.0.0-alpha1".lexical_core}" deps) ] else []));

    buildDependencies = mapFeatures features ([
      (crates."version_check"."${deps."nom"."6.0.0-alpha1"."version_check"}" deps)
    ]);
    features = mkFeatures (features."nom"."6.0.0-alpha1" or {});
  };
  features_.nom."6.0.0-alpha1" = deps: f: updateFeatures f (rec {
    lexical_core."${deps.nom."6.0.0-alpha1".lexical_core}".default = true;
    memchr = fold recursiveUpdate {} [
      { "${deps.nom."6.0.0-alpha1".memchr}"."use_std" =
        (f.memchr."${deps.nom."6.0.0-alpha1".memchr}"."use_std" or false) ||
        (nom."6.0.0-alpha1"."std" or false) ||
        (f."nom"."6.0.0-alpha1"."std" or false); }
      { "${deps.nom."6.0.0-alpha1".memchr}".default = (f.memchr."${deps.nom."6.0.0-alpha1".memchr}".default or false); }
    ];
    nom = fold recursiveUpdate {} [
      { "6.0.0-alpha1"."alloc" =
        (f.nom."6.0.0-alpha1"."alloc" or false) ||
        (f.nom."6.0.0-alpha1".std or false) ||
        (nom."6.0.0-alpha1"."std" or false); }
      { "6.0.0-alpha1"."lexical" =
        (f.nom."6.0.0-alpha1"."lexical" or false) ||
        (f.nom."6.0.0-alpha1".default or false) ||
        (nom."6.0.0-alpha1"."default" or false); }
      { "6.0.0-alpha1"."lexical-core" =
        (f.nom."6.0.0-alpha1"."lexical-core" or false) ||
        (f.nom."6.0.0-alpha1".lexical or false) ||
        (nom."6.0.0-alpha1"."lexical" or false); }
      { "6.0.0-alpha1"."regex" =
        (f.nom."6.0.0-alpha1"."regex" or false) ||
        (f.nom."6.0.0-alpha1".regexp or false) ||
        (nom."6.0.0-alpha1"."regexp" or false); }
      { "6.0.0-alpha1"."std" =
        (f.nom."6.0.0-alpha1"."std" or false) ||
        (f.nom."6.0.0-alpha1".default or false) ||
        (nom."6.0.0-alpha1"."default" or false); }
      { "6.0.0-alpha1".default = (f.nom."6.0.0-alpha1".default or true); }
    ];
    version_check."${deps.nom."6.0.0-alpha1".version_check}".default = true;
  }) [
    (features_.lexical_core."${deps."nom"."6.0.0-alpha1"."lexical_core"}" deps)
    (features_.memchr."${deps."nom"."6.0.0-alpha1"."memchr"}" deps)
    (features_.version_check."${deps."nom"."6.0.0-alpha1"."version_check"}" deps)
  ];


# end
# ntapi-0.3.3

  crates.ntapi."0.3.3" = deps: { features?(features_.ntapi."0.3.3" deps {}) }: buildRustCrate {
    crateName = "ntapi";
    version = "0.3.3";
    description = "FFI bindings for Native API";
    authors = [ "MSxDOS <melcodos@gmail.com>" ];
    edition = "2018";
    sha256 = "0y5shrkzclgr6wvn25jqpzy9wdy8n4zhiy0bj9d6gl91zr5gnh1v";
    dependencies = mapFeatures features ([
      (crates."winapi"."${deps."ntapi"."0.3.3"."winapi"}" deps)
    ]);
    features = mkFeatures (features."ntapi"."0.3.3" or {});
  };
  features_.ntapi."0.3.3" = deps: f: updateFeatures f (rec {
    ntapi = fold recursiveUpdate {} [
      { "0.3.3"."user" =
        (f.ntapi."0.3.3"."user" or false) ||
        (f.ntapi."0.3.3".default or false) ||
        (ntapi."0.3.3"."default" or false); }
      { "0.3.3".default = (f.ntapi."0.3.3".default or true); }
    ];
    winapi = fold recursiveUpdate {} [
      { "${deps.ntapi."0.3.3".winapi}"."cfg" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."evntrace" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."impl-default" =
        (f.winapi."${deps.ntapi."0.3.3".winapi}"."impl-default" or false) ||
        (ntapi."0.3.3"."impl-default" or false) ||
        (f."ntapi"."0.3.3"."impl-default" or false); }
      { "${deps.ntapi."0.3.3".winapi}"."in6addr" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."inaddr" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."minwinbase" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."ntsecapi" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."windef" = true; }
      { "${deps.ntapi."0.3.3".winapi}"."winioctl" = true; }
      { "${deps.ntapi."0.3.3".winapi}".default = true; }
    ];
  }) [
    (features_.winapi."${deps."ntapi"."0.3.3"."winapi"}" deps)
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
# num_cpus-1.13.0

  crates.num_cpus."1.13.0" = deps: { features?(features_.num_cpus."1.13.0" deps {}) }: buildRustCrate {
    crateName = "num_cpus";
    version = "1.13.0";
    description = "Get the number of CPUs on a machine.";
    authors = [ "Sean McArthur <sean@seanmonstar.com>" ];
    sha256 = "15pqq0ldi8zrqbr3cn539xlzl2hhyhka5d1z6ix0vk15qzj3nw46";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."num_cpus"."1.13.0"."libc"}" deps)
    ])
      ++ (if cpu == "x86_64" || cpu == "aarch64" && kernel == "hermit" then mapFeatures features ([
      (crates."hermit_abi"."${deps."num_cpus"."1.13.0"."hermit_abi"}" deps)
    ]) else []);
  };
  features_.num_cpus."1.13.0" = deps: f: updateFeatures f (rec {
    hermit_abi."${deps.num_cpus."1.13.0".hermit_abi}".default = true;
    libc."${deps.num_cpus."1.13.0".libc}".default = true;
    num_cpus."1.13.0".default = (f.num_cpus."1.13.0".default or true);
  }) [
    (features_.libc."${deps."num_cpus"."1.13.0"."libc"}" deps)
    (features_.hermit_abi."${deps."num_cpus"."1.13.0"."hermit_abi"}" deps)
  ];


# end
# once_cell-1.3.1

  crates.once_cell."1.3.1" = deps: { features?(features_.once_cell."1.3.1" deps {}) }: buildRustCrate {
    crateName = "once_cell";
    version = "1.3.1";
    description = "Single assignment cells and lazy values.";
    authors = [ "Aleksey Kladov <aleksey.kladov@gmail.com>" ];
    edition = "2018";
    sha256 = "04zxdzxs689n7jl34nxwapkx4kp24vwq37xnnjm4scinnp50y1k5";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."once_cell"."1.3.1" or {});
  };
  features_.once_cell."1.3.1" = deps: f: updateFeatures f (rec {
    once_cell = fold recursiveUpdate {} [
      { "1.3.1"."std" =
        (f.once_cell."1.3.1"."std" or false) ||
        (f.once_cell."1.3.1".default or false) ||
        (once_cell."1.3.1"."default" or false); }
      { "1.3.1".default = (f.once_cell."1.3.1".default or true); }
    ];
  }) [];


# end
# opaque-debug-0.2.3

  crates.opaque_debug."0.2.3" = deps: { features?(features_.opaque_debug."0.2.3" deps {}) }: buildRustCrate {
    crateName = "opaque-debug";
    version = "0.2.3";
    description = "Macro for opaque Debug trait implementation";
    authors = [ "RustCrypto Developers" ];
    sha256 = "1did2dvmc88chf7qvs3c0qj5filfp6q75rmf2x9ljwlbwywv8lj5";
  };
  features_.opaque_debug."0.2.3" = deps: f: updateFeatures f (rec {
    opaque_debug."0.2.3".default = (f.opaque_debug."0.2.3".default or true);
  }) [];


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
# openssl-0.10.29

  crates.openssl."0.10.29" = deps: { features?(features_.openssl."0.10.29" deps {}) }: buildRustCrate {
    crateName = "openssl";
    version = "0.10.29";
    description = "OpenSSL bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "02vjmz0pm29s6s869q1153pskjdkyd1qqwj8j03linrm3j7609b3";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."openssl"."0.10.29"."bitflags"}" deps)
      (crates."cfg_if"."${deps."openssl"."0.10.29"."cfg_if"}" deps)
      (crates."foreign_types"."${deps."openssl"."0.10.29"."foreign_types"}" deps)
      (crates."lazy_static"."${deps."openssl"."0.10.29"."lazy_static"}" deps)
      (crates."libc"."${deps."openssl"."0.10.29"."libc"}" deps)
      (crates."openssl_sys"."${deps."openssl"."0.10.29"."openssl_sys"}" deps)
    ]);
    features = mkFeatures (features."openssl"."0.10.29" or {});
  };
  features_.openssl."0.10.29" = deps: f: updateFeatures f (rec {
    bitflags."${deps.openssl."0.10.29".bitflags}".default = true;
    cfg_if."${deps.openssl."0.10.29".cfg_if}".default = true;
    foreign_types."${deps.openssl."0.10.29".foreign_types}".default = true;
    lazy_static."${deps.openssl."0.10.29".lazy_static}".default = true;
    libc."${deps.openssl."0.10.29".libc}".default = true;
    openssl."0.10.29".default = (f.openssl."0.10.29".default or true);
    openssl_sys = fold recursiveUpdate {} [
      { "${deps.openssl."0.10.29".openssl_sys}"."vendored" =
        (f.openssl_sys."${deps.openssl."0.10.29".openssl_sys}"."vendored" or false) ||
        (openssl."0.10.29"."vendored" or false) ||
        (f."openssl"."0.10.29"."vendored" or false); }
      { "${deps.openssl."0.10.29".openssl_sys}".default = true; }
    ];
  }) [
    (features_.bitflags."${deps."openssl"."0.10.29"."bitflags"}" deps)
    (features_.cfg_if."${deps."openssl"."0.10.29"."cfg_if"}" deps)
    (features_.foreign_types."${deps."openssl"."0.10.29"."foreign_types"}" deps)
    (features_.lazy_static."${deps."openssl"."0.10.29"."lazy_static"}" deps)
    (features_.libc."${deps."openssl"."0.10.29"."libc"}" deps)
    (features_.openssl_sys."${deps."openssl"."0.10.29"."openssl_sys"}" deps)
  ];


# end
# openssl-probe-0.1.2

  crates.openssl_probe."0.1.2" = deps: { features?(features_.openssl_probe."0.1.2" deps {}) }: buildRustCrate {
    crateName = "openssl-probe";
    version = "0.1.2";
    description = "Tool for helping to find SSL certificate locations on the system for OpenSSL\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "1a89fznx26vvaxyrxdvgf6iwai5xvs6xjvpjin68fgvrslv6n15a";
  };
  features_.openssl_probe."0.1.2" = deps: f: updateFeatures f (rec {
    openssl_probe."0.1.2".default = (f.openssl_probe."0.1.2".default or true);
  }) [];


# end
# openssl-sys-0.9.55

  crates.openssl_sys."0.9.55" = deps: { features?(features_.openssl_sys."0.9.55" deps {}) }: buildRustCrate {
    crateName = "openssl-sys";
    version = "0.9.55";
    description = "FFI bindings to OpenSSL";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" "Steven Fackler <sfackler@gmail.com>" ];
    sha256 = "1c05nicx77cfsi4g6vx0sq8blk7075p4wh07hzzy5l6awp5vw0m4";
    build = "build/main.rs";
    dependencies = mapFeatures features ([
      (crates."libc"."${deps."openssl_sys"."0.9.55"."libc"}" deps)
    ])
      ++ (if abi == "msvc" then mapFeatures features ([
]) else []);

    buildDependencies = mapFeatures features ([
      (crates."autocfg"."${deps."openssl_sys"."0.9.55"."autocfg"}" deps)
      (crates."cc"."${deps."openssl_sys"."0.9.55"."cc"}" deps)
      (crates."pkg_config"."${deps."openssl_sys"."0.9.55"."pkg_config"}" deps)
    ]);
    features = mkFeatures (features."openssl_sys"."0.9.55" or {});
  };
  features_.openssl_sys."0.9.55" = deps: f: updateFeatures f (rec {
    autocfg."${deps.openssl_sys."0.9.55".autocfg}".default = true;
    cc."${deps.openssl_sys."0.9.55".cc}".default = true;
    libc."${deps.openssl_sys."0.9.55".libc}".default = true;
    openssl_sys = fold recursiveUpdate {} [
      { "0.9.55"."openssl-src" =
        (f.openssl_sys."0.9.55"."openssl-src" or false) ||
        (f.openssl_sys."0.9.55".vendored or false) ||
        (openssl_sys."0.9.55"."vendored" or false); }
      { "0.9.55".default = (f.openssl_sys."0.9.55".default or true); }
    ];
    pkg_config."${deps.openssl_sys."0.9.55".pkg_config}".default = true;
  }) [
    (features_.libc."${deps."openssl_sys"."0.9.55"."libc"}" deps)
    (features_.autocfg."${deps."openssl_sys"."0.9.55"."autocfg"}" deps)
    (features_.cc."${deps."openssl_sys"."0.9.55"."cc"}" deps)
    (features_.pkg_config."${deps."openssl_sys"."0.9.55"."pkg_config"}" deps)
  ];


# end
# parking_lot-0.10.2

  crates.parking_lot."0.10.2" = deps: { features?(features_.parking_lot."0.10.2" deps {}) }: buildRustCrate {
    crateName = "parking_lot";
    version = "0.10.2";
    description = "More compact and efficient implementations of the standard synchronization primitives.";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    edition = "2018";
    sha256 = "16l3b5abidd0v0dhr15fphl0caxnfnrln7lr5mzqkmg6rx1mq0ls";
    dependencies = mapFeatures features ([
      (crates."lock_api"."${deps."parking_lot"."0.10.2"."lock_api"}" deps)
      (crates."parking_lot_core"."${deps."parking_lot"."0.10.2"."parking_lot_core"}" deps)
    ]);
    features = mkFeatures (features."parking_lot"."0.10.2" or {});
  };
  features_.parking_lot."0.10.2" = deps: f: updateFeatures f (rec {
    lock_api = fold recursiveUpdate {} [
      { "${deps.parking_lot."0.10.2".lock_api}"."nightly" =
        (f.lock_api."${deps.parking_lot."0.10.2".lock_api}"."nightly" or false) ||
        (parking_lot."0.10.2"."nightly" or false) ||
        (f."parking_lot"."0.10.2"."nightly" or false); }
      { "${deps.parking_lot."0.10.2".lock_api}"."owning_ref" =
        (f.lock_api."${deps.parking_lot."0.10.2".lock_api}"."owning_ref" or false) ||
        (parking_lot."0.10.2"."owning_ref" or false) ||
        (f."parking_lot"."0.10.2"."owning_ref" or false); }
      { "${deps.parking_lot."0.10.2".lock_api}"."serde" =
        (f.lock_api."${deps.parking_lot."0.10.2".lock_api}"."serde" or false) ||
        (parking_lot."0.10.2"."serde" or false) ||
        (f."parking_lot"."0.10.2"."serde" or false); }
      { "${deps.parking_lot."0.10.2".lock_api}".default = true; }
    ];
    parking_lot."0.10.2".default = (f.parking_lot."0.10.2".default or true);
    parking_lot_core = fold recursiveUpdate {} [
      { "${deps.parking_lot."0.10.2".parking_lot_core}"."deadlock_detection" =
        (f.parking_lot_core."${deps.parking_lot."0.10.2".parking_lot_core}"."deadlock_detection" or false) ||
        (parking_lot."0.10.2"."deadlock_detection" or false) ||
        (f."parking_lot"."0.10.2"."deadlock_detection" or false); }
      { "${deps.parking_lot."0.10.2".parking_lot_core}"."nightly" =
        (f.parking_lot_core."${deps.parking_lot."0.10.2".parking_lot_core}"."nightly" or false) ||
        (parking_lot."0.10.2"."nightly" or false) ||
        (f."parking_lot"."0.10.2"."nightly" or false); }
      { "${deps.parking_lot."0.10.2".parking_lot_core}".default = true; }
    ];
  }) [
    (features_.lock_api."${deps."parking_lot"."0.10.2"."lock_api"}" deps)
    (features_.parking_lot_core."${deps."parking_lot"."0.10.2"."parking_lot_core"}" deps)
  ];


# end
# parking_lot_core-0.7.2

  crates.parking_lot_core."0.7.2" = deps: { features?(features_.parking_lot_core."0.7.2" deps {}) }: buildRustCrate {
    crateName = "parking_lot_core";
    version = "0.7.2";
    description = "An advanced API for creating custom synchronization primitives.";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    edition = "2018";
    sha256 = "0ac64bpq6hx17099c0izfj52hk57012c7rp77arm2d5aapqcr2la";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."parking_lot_core"."0.7.2"."cfg_if"}" deps)
      (crates."smallvec"."${deps."parking_lot_core"."0.7.2"."smallvec"}" deps)
    ])
      ++ (if kernel == "cloudabi" then mapFeatures features ([
      (crates."cloudabi"."${deps."parking_lot_core"."0.7.2"."cloudabi"}" deps)
    ]) else [])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."parking_lot_core"."0.7.2"."redox_syscall"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."parking_lot_core"."0.7.2"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."parking_lot_core"."0.7.2"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."parking_lot_core"."0.7.2" or {});
  };
  features_.parking_lot_core."0.7.2" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.parking_lot_core."0.7.2".cfg_if}".default = true;
    cloudabi."${deps.parking_lot_core."0.7.2".cloudabi}".default = true;
    libc."${deps.parking_lot_core."0.7.2".libc}".default = true;
    parking_lot_core = fold recursiveUpdate {} [
      { "0.7.2"."backtrace" =
        (f.parking_lot_core."0.7.2"."backtrace" or false) ||
        (f.parking_lot_core."0.7.2".deadlock_detection or false) ||
        (parking_lot_core."0.7.2"."deadlock_detection" or false); }
      { "0.7.2"."petgraph" =
        (f.parking_lot_core."0.7.2"."petgraph" or false) ||
        (f.parking_lot_core."0.7.2".deadlock_detection or false) ||
        (parking_lot_core."0.7.2"."deadlock_detection" or false); }
      { "0.7.2"."thread-id" =
        (f.parking_lot_core."0.7.2"."thread-id" or false) ||
        (f.parking_lot_core."0.7.2".deadlock_detection or false) ||
        (parking_lot_core."0.7.2"."deadlock_detection" or false); }
      { "0.7.2".default = (f.parking_lot_core."0.7.2".default or true); }
    ];
    redox_syscall."${deps.parking_lot_core."0.7.2".redox_syscall}".default = true;
    smallvec."${deps.parking_lot_core."0.7.2".smallvec}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.parking_lot_core."0.7.2".winapi}"."errhandlingapi" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."handleapi" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."minwindef" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."ntstatus" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."winbase" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."winerror" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}"."winnt" = true; }
      { "${deps.parking_lot_core."0.7.2".winapi}".default = true; }
    ];
  }) [
    (features_.cfg_if."${deps."parking_lot_core"."0.7.2"."cfg_if"}" deps)
    (features_.smallvec."${deps."parking_lot_core"."0.7.2"."smallvec"}" deps)
    (features_.cloudabi."${deps."parking_lot_core"."0.7.2"."cloudabi"}" deps)
    (features_.redox_syscall."${deps."parking_lot_core"."0.7.2"."redox_syscall"}" deps)
    (features_.libc."${deps."parking_lot_core"."0.7.2"."libc"}" deps)
    (features_.winapi."${deps."parking_lot_core"."0.7.2"."winapi"}" deps)
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
# percent-encoding-2.1.0

  crates.percent_encoding."2.1.0" = deps: { features?(features_.percent_encoding."2.1.0" deps {}) }: buildRustCrate {
    crateName = "percent-encoding";
    version = "2.1.0";
    description = "Percent encoding and decoding";
    authors = [ "The rust-url developers" ];
    sha256 = "0i838f2nr81585ckmfymf8l1x1vdmx6n8xqvli0lgcy60yl2axy3";
    libPath = "lib.rs";
  };
  features_.percent_encoding."2.1.0" = deps: f: updateFeatures f (rec {
    percent_encoding."2.1.0".default = (f.percent_encoding."2.1.0".default or true);
  }) [];


# end
# pest-2.1.3

  crates.pest."2.1.3" = deps: { features?(features_.pest."2.1.3" deps {}) }: buildRustCrate {
    crateName = "pest";
    version = "2.1.3";
    description = "The Elegant Parser";
    authors = [ "Dragoș Tiselice <dragostiselice@gmail.com>" ];
    sha256 = "07v3dc56isy9r6bxs4jhm3w09jbq4w9fjg0ncdwmm1wliia5xgh4";
    dependencies = mapFeatures features ([
      (crates."ucd_trie"."${deps."pest"."2.1.3"."ucd_trie"}" deps)
    ]);
    features = mkFeatures (features."pest"."2.1.3" or {});
  };
  features_.pest."2.1.3" = deps: f: updateFeatures f (rec {
    pest = fold recursiveUpdate {} [
      { "2.1.3"."serde" =
        (f.pest."2.1.3"."serde" or false) ||
        (f.pest."2.1.3".pretty-print or false) ||
        (pest."2.1.3"."pretty-print" or false); }
      { "2.1.3"."serde_json" =
        (f.pest."2.1.3"."serde_json" or false) ||
        (f.pest."2.1.3".pretty-print or false) ||
        (pest."2.1.3"."pretty-print" or false); }
      { "2.1.3".default = (f.pest."2.1.3".default or true); }
    ];
    ucd_trie."${deps.pest."2.1.3".ucd_trie}".default = true;
  }) [
    (features_.ucd_trie."${deps."pest"."2.1.3"."ucd_trie"}" deps)
  ];


# end
# pest_derive-2.1.0

  crates.pest_derive."2.1.0" = deps: { features?(features_.pest_derive."2.1.0" deps {}) }: buildRustCrate {
    crateName = "pest_derive";
    version = "2.1.0";
    description = "pest's derive macro";
    authors = [ "Dragoș Tiselice <dragostiselice@gmail.com>" ];
    sha256 = "03bsaw7jpsk6x3dbrs9bjx5najjdvslb9y77azfn1n403khrqvnm";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."pest"."${deps."pest_derive"."2.1.0"."pest"}" deps)
      (crates."pest_generator"."${deps."pest_derive"."2.1.0"."pest_generator"}" deps)
    ]);
  };
  features_.pest_derive."2.1.0" = deps: f: updateFeatures f (rec {
    pest."${deps.pest_derive."2.1.0".pest}".default = true;
    pest_derive."2.1.0".default = (f.pest_derive."2.1.0".default or true);
    pest_generator."${deps.pest_derive."2.1.0".pest_generator}".default = true;
  }) [
    (features_.pest."${deps."pest_derive"."2.1.0"."pest"}" deps)
    (features_.pest_generator."${deps."pest_derive"."2.1.0"."pest_generator"}" deps)
  ];


# end
# pest_generator-2.1.3

  crates.pest_generator."2.1.3" = deps: { features?(features_.pest_generator."2.1.3" deps {}) }: buildRustCrate {
    crateName = "pest_generator";
    version = "2.1.3";
    description = "pest code generator";
    authors = [ "Dragoș Tiselice <dragostiselice@gmail.com>" ];
    sha256 = "1fryqsrx8ks46ppch8386sh5a0mp6rdzw5gnk11z11y68wjf9di1";
    dependencies = mapFeatures features ([
      (crates."pest"."${deps."pest_generator"."2.1.3"."pest"}" deps)
      (crates."pest_meta"."${deps."pest_generator"."2.1.3"."pest_meta"}" deps)
      (crates."proc_macro2"."${deps."pest_generator"."2.1.3"."proc_macro2"}" deps)
      (crates."quote"."${deps."pest_generator"."2.1.3"."quote"}" deps)
      (crates."syn"."${deps."pest_generator"."2.1.3"."syn"}" deps)
    ]);
  };
  features_.pest_generator."2.1.3" = deps: f: updateFeatures f (rec {
    pest."${deps.pest_generator."2.1.3".pest}".default = true;
    pest_generator."2.1.3".default = (f.pest_generator."2.1.3".default or true);
    pest_meta."${deps.pest_generator."2.1.3".pest_meta}".default = true;
    proc_macro2."${deps.pest_generator."2.1.3".proc_macro2}".default = true;
    quote."${deps.pest_generator."2.1.3".quote}".default = true;
    syn."${deps.pest_generator."2.1.3".syn}".default = true;
  }) [
    (features_.pest."${deps."pest_generator"."2.1.3"."pest"}" deps)
    (features_.pest_meta."${deps."pest_generator"."2.1.3"."pest_meta"}" deps)
    (features_.proc_macro2."${deps."pest_generator"."2.1.3"."proc_macro2"}" deps)
    (features_.quote."${deps."pest_generator"."2.1.3"."quote"}" deps)
    (features_.syn."${deps."pest_generator"."2.1.3"."syn"}" deps)
  ];


# end
# pest_meta-2.1.3

  crates.pest_meta."2.1.3" = deps: { features?(features_.pest_meta."2.1.3" deps {}) }: buildRustCrate {
    crateName = "pest_meta";
    version = "2.1.3";
    description = "pest meta language parser and validator";
    authors = [ "Dragoș Tiselice <dragostiselice@gmail.com>" ];
    sha256 = "0krcqyzz12hdq8f05zxb3qqhkws8lwdpp9gwpgdvr9zz5nb7sshk";
    dependencies = mapFeatures features ([
      (crates."maplit"."${deps."pest_meta"."2.1.3"."maplit"}" deps)
      (crates."pest"."${deps."pest_meta"."2.1.3"."pest"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."sha_1"."${deps."pest_meta"."2.1.3"."sha_1"}" deps)
    ]);
  };
  features_.pest_meta."2.1.3" = deps: f: updateFeatures f (rec {
    maplit."${deps.pest_meta."2.1.3".maplit}".default = true;
    pest."${deps.pest_meta."2.1.3".pest}".default = true;
    pest_meta."2.1.3".default = (f.pest_meta."2.1.3".default or true);
    sha_1."${deps.pest_meta."2.1.3".sha_1}".default = (f.sha_1."${deps.pest_meta."2.1.3".sha_1}".default or false);
  }) [
    (features_.maplit."${deps."pest_meta"."2.1.3"."maplit"}" deps)
    (features_.pest."${deps."pest_meta"."2.1.3"."pest"}" deps)
    (features_.sha_1."${deps."pest_meta"."2.1.3"."sha_1"}" deps)
  ];


# end
# pin-project-lite-0.1.4

  crates.pin_project_lite."0.1.4" = deps: { features?(features_.pin_project_lite."0.1.4" deps {}) }: buildRustCrate {
    crateName = "pin-project-lite";
    version = "0.1.4";
    description = "A lightweight version of pin-project written with declarative macros.\n";
    authors = [ "Taiki Endo <te316e89@gmail.com>" ];
    edition = "2018";
    sha256 = "01pjrqsjxh1ypqcy07wk7jwmns89mkq8zgjmanjnzx261sw69s79";
  };
  features_.pin_project_lite."0.1.4" = deps: f: updateFeatures f (rec {
    pin_project_lite."0.1.4".default = (f.pin_project_lite."0.1.4".default or true);
  }) [];


# end
# pin-utils-0.1.0

  crates.pin_utils."0.1.0" = deps: { features?(features_.pin_utils."0.1.0" deps {}) }: buildRustCrate {
    crateName = "pin-utils";
    version = "0.1.0";
    description = "Utilities for pinning\n";
    authors = [ "Josef Brandl <mail@josefbrandl.de>" ];
    edition = "2018";
    sha256 = "0cskzbx38gqdj7ij3i73xf7f54sccnd2pb4jq4ka5l1fb3kvpxjz";
  };
  features_.pin_utils."0.1.0" = deps: f: updateFeatures f (rec {
    pin_utils."0.1.0".default = (f.pin_utils."0.1.0".default or true);
  }) [];


# end
# pinky-swear-4.0.0

  crates.pinky_swear."4.0.0" = deps: { features?(features_.pinky_swear."4.0.0" deps {}) }: buildRustCrate {
    crateName = "pinky-swear";
    version = "4.0.0";
    description = "Futures and async/await-ready Promises";
    authors = [ "Marc-Antoine Perennou <Marc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "1s28hx525554s53svxrizsffqxrnxxbpxnwzc34my00djs7vp0fz";
    dependencies = mapFeatures features ([
      (crates."doc_comment"."${deps."pinky_swear"."4.0.0"."doc_comment"}" deps)
      (crates."log"."${deps."pinky_swear"."4.0.0"."log"}" deps)
      (crates."parking_lot"."${deps."pinky_swear"."4.0.0"."parking_lot"}" deps)
    ]);
  };
  features_.pinky_swear."4.0.0" = deps: f: updateFeatures f (rec {
    doc_comment."${deps.pinky_swear."4.0.0".doc_comment}".default = true;
    log."${deps.pinky_swear."4.0.0".log}".default = true;
    parking_lot."${deps.pinky_swear."4.0.0".parking_lot}".default = true;
    pinky_swear."4.0.0".default = (f.pinky_swear."4.0.0".default or true);
  }) [
    (features_.doc_comment."${deps."pinky_swear"."4.0.0"."doc_comment"}" deps)
    (features_.log."${deps."pinky_swear"."4.0.0"."log"}" deps)
    (features_.parking_lot."${deps."pinky_swear"."4.0.0"."parking_lot"}" deps)
  ];


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
# ppv-lite86-0.2.6

  crates.ppv_lite86."0.2.6" = deps: { features?(features_.ppv_lite86."0.2.6" deps {}) }: buildRustCrate {
    crateName = "ppv-lite86";
    version = "0.2.6";
    description = "Implementation of the crypto-simd API for x86";
    authors = [ "The CryptoCorrosion Contributors" ];
    edition = "2018";
    sha256 = "1mlbp0713frbyvcbjmc5vl062b0vr58agkv3ar2qqi5plgy9b7ib";
    features = mkFeatures (features."ppv_lite86"."0.2.6" or {});
  };
  features_.ppv_lite86."0.2.6" = deps: f: updateFeatures f (rec {
    ppv_lite86 = fold recursiveUpdate {} [
      { "0.2.6"."simd" =
        (f.ppv_lite86."0.2.6"."simd" or false) ||
        (f.ppv_lite86."0.2.6".default or false) ||
        (ppv_lite86."0.2.6"."default" or false); }
      { "0.2.6"."std" =
        (f.ppv_lite86."0.2.6"."std" or false) ||
        (f.ppv_lite86."0.2.6".default or false) ||
        (ppv_lite86."0.2.6"."default" or false); }
      { "0.2.6".default = (f.ppv_lite86."0.2.6".default or true); }
    ];
  }) [];


# end
# proc-macro2-1.0.10

  crates.proc_macro2."1.0.10" = deps: { features?(features_.proc_macro2."1.0.10" deps {}) }: buildRustCrate {
    crateName = "proc-macro2";
    version = "1.0.10";
    description = "A substitute implementation of the compiler's `proc_macro` API to decouple\ntoken-based libraries from the procedural macro use case.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "1sb317587iwq1554s0ksap6718w2l73qa07h2amg3716h8llg6zv";
    dependencies = mapFeatures features ([
      (crates."unicode_xid"."${deps."proc_macro2"."1.0.10"."unicode_xid"}" deps)
    ]);
    features = mkFeatures (features."proc_macro2"."1.0.10" or {});
  };
  features_.proc_macro2."1.0.10" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "1.0.10"."proc-macro" =
        (f.proc_macro2."1.0.10"."proc-macro" or false) ||
        (f.proc_macro2."1.0.10".default or false) ||
        (proc_macro2."1.0.10"."default" or false); }
      { "1.0.10".default = (f.proc_macro2."1.0.10".default or true); }
    ];
    unicode_xid."${deps.proc_macro2."1.0.10".unicode_xid}".default = true;
  }) [
    (features_.unicode_xid."${deps."proc_macro2"."1.0.10"."unicode_xid"}" deps)
  ];


# end
# quick-error-1.2.3

  crates.quick_error."1.2.3" = deps: { features?(features_.quick_error."1.2.3" deps {}) }: buildRustCrate {
    crateName = "quick-error";
    version = "1.2.3";
    description = "    A macro which makes error types pleasant to write.\n";
    authors = [ "Paul Colomiets <paul@colomiets.name>" "Colin Kiegel <kiegel@gmx.de>" ];
    sha256 = "17gqp7ifp6j5pcnk450f964a5jkqmy71848x69ahmsa9gyzhkh7x";
  };
  features_.quick_error."1.2.3" = deps: f: updateFeatures f (rec {
    quick_error."1.2.3".default = (f.quick_error."1.2.3".default or true);
  }) [];


# end
# quote-1.0.3

  crates.quote."1.0.3" = deps: { features?(features_.quote."1.0.3" deps {}) }: buildRustCrate {
    crateName = "quote";
    version = "1.0.3";
    description = "Quasi-quoting macro quote!(...)";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "093chkpg7dc8f86kz0hlxzyfxvbix3xpkmlbhilf0wji228ad35c";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."quote"."1.0.3"."proc_macro2"}" deps)
    ]);
    features = mkFeatures (features."quote"."1.0.3" or {});
  };
  features_.quote."1.0.3" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.quote."1.0.3".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.quote."1.0.3".proc_macro2}"."proc-macro" or false) ||
        (quote."1.0.3"."proc-macro" or false) ||
        (f."quote"."1.0.3"."proc-macro" or false); }
      { "${deps.quote."1.0.3".proc_macro2}".default = (f.proc_macro2."${deps.quote."1.0.3".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "1.0.3"."proc-macro" =
        (f.quote."1.0.3"."proc-macro" or false) ||
        (f.quote."1.0.3".default or false) ||
        (quote."1.0.3"."default" or false); }
      { "1.0.3".default = (f.quote."1.0.3".default or true); }
    ];
  }) [
    (features_.proc_macro2."${deps."quote"."1.0.3"."proc_macro2"}" deps)
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
# rand-0.7.3

  crates.rand."0.7.3" = deps: { features?(features_.rand."0.7.3" deps {}) }: buildRustCrate {
    crateName = "rand";
    version = "0.7.3";
    description = "Random number generators and other randomness functionality.\n";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" ];
    edition = "2018";
    sha256 = "1amg6qj53ylq3ix22n27kmj1gyj6i15my36mkayr30ndymny0b8r";
    dependencies = mapFeatures features ([
      (crates."rand_core"."${deps."rand"."0.7.3"."rand_core"}" deps)
    ])
      ++ (if !(kernel == "emscripten") then mapFeatures features ([
      (crates."rand_chacha"."${deps."rand"."0.7.3"."rand_chacha"}" deps)
    ]) else [])
      ++ (if kernel == "emscripten" then mapFeatures features ([
      (crates."rand_hc"."${deps."rand"."0.7.3"."rand_hc"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
    ]
      ++ (if features.rand."0.7.3".libc or false then [ (crates.libc."${deps."rand"."0.7.3".libc}" deps) ] else [])) else []);
    features = mkFeatures (features."rand"."0.7.3" or {});
  };
  features_.rand."0.7.3" = deps: f: updateFeatures f (rec {
    libc."${deps.rand."0.7.3".libc}".default = (f.libc."${deps.rand."0.7.3".libc}".default or false);
    rand = fold recursiveUpdate {} [
      { "0.7.3"."alloc" =
        (f.rand."0.7.3"."alloc" or false) ||
        (f.rand."0.7.3".std or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."getrandom" =
        (f.rand."0.7.3"."getrandom" or false) ||
        (f.rand."0.7.3".std or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."getrandom_package" =
        (f.rand."0.7.3"."getrandom_package" or false) ||
        (f.rand."0.7.3".getrandom or false) ||
        (rand."0.7.3"."getrandom" or false); }
      { "0.7.3"."libc" =
        (f.rand."0.7.3"."libc" or false) ||
        (f.rand."0.7.3".std or false) ||
        (rand."0.7.3"."std" or false); }
      { "0.7.3"."packed_simd" =
        (f.rand."0.7.3"."packed_simd" or false) ||
        (f.rand."0.7.3".simd_support or false) ||
        (rand."0.7.3"."simd_support" or false); }
      { "0.7.3"."rand_pcg" =
        (f.rand."0.7.3"."rand_pcg" or false) ||
        (f.rand."0.7.3".small_rng or false) ||
        (rand."0.7.3"."small_rng" or false); }
      { "0.7.3"."simd_support" =
        (f.rand."0.7.3"."simd_support" or false) ||
        (f.rand."0.7.3".nightly or false) ||
        (rand."0.7.3"."nightly" or false); }
      { "0.7.3"."std" =
        (f.rand."0.7.3"."std" or false) ||
        (f.rand."0.7.3".default or false) ||
        (rand."0.7.3"."default" or false); }
      { "0.7.3".default = (f.rand."0.7.3".default or true); }
    ];
    rand_chacha."${deps.rand."0.7.3".rand_chacha}".default = (f.rand_chacha."${deps.rand."0.7.3".rand_chacha}".default or false);
    rand_core = fold recursiveUpdate {} [
      { "${deps.rand."0.7.3".rand_core}"."alloc" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."alloc" or false) ||
        (rand."0.7.3"."alloc" or false) ||
        (f."rand"."0.7.3"."alloc" or false); }
      { "${deps.rand."0.7.3".rand_core}"."getrandom" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."getrandom" or false) ||
        (rand."0.7.3"."getrandom" or false) ||
        (f."rand"."0.7.3"."getrandom" or false); }
      { "${deps.rand."0.7.3".rand_core}"."std" =
        (f.rand_core."${deps.rand."0.7.3".rand_core}"."std" or false) ||
        (rand."0.7.3"."std" or false) ||
        (f."rand"."0.7.3"."std" or false); }
      { "${deps.rand."0.7.3".rand_core}".default = true; }
    ];
    rand_hc."${deps.rand."0.7.3".rand_hc}".default = true;
  }) [
    (features_.rand_core."${deps."rand"."0.7.3"."rand_core"}" deps)
    (features_.rand_chacha."${deps."rand"."0.7.3"."rand_chacha"}" deps)
    (features_.rand_hc."${deps."rand"."0.7.3"."rand_hc"}" deps)
    (features_.libc."${deps."rand"."0.7.3"."libc"}" deps)
  ];


# end
# rand_chacha-0.2.2

  crates.rand_chacha."0.2.2" = deps: { features?(features_.rand_chacha."0.2.2" deps {}) }: buildRustCrate {
    crateName = "rand_chacha";
    version = "0.2.2";
    description = "ChaCha random number generator\n";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" "The CryptoCorrosion Contributors" ];
    edition = "2018";
    sha256 = "1adx0h0h17y6sxlx1zpwg0hnyccnwlp5ad7dxn2jib9caw1s7ghk";
    dependencies = mapFeatures features ([
      (crates."ppv_lite86"."${deps."rand_chacha"."0.2.2"."ppv_lite86"}" deps)
      (crates."rand_core"."${deps."rand_chacha"."0.2.2"."rand_core"}" deps)
    ]);
    features = mkFeatures (features."rand_chacha"."0.2.2" or {});
  };
  features_.rand_chacha."0.2.2" = deps: f: updateFeatures f (rec {
    ppv_lite86 = fold recursiveUpdate {} [
      { "${deps.rand_chacha."0.2.2".ppv_lite86}"."simd" = true; }
      { "${deps.rand_chacha."0.2.2".ppv_lite86}"."std" =
        (f.ppv_lite86."${deps.rand_chacha."0.2.2".ppv_lite86}"."std" or false) ||
        (rand_chacha."0.2.2"."std" or false) ||
        (f."rand_chacha"."0.2.2"."std" or false); }
      { "${deps.rand_chacha."0.2.2".ppv_lite86}".default = (f.ppv_lite86."${deps.rand_chacha."0.2.2".ppv_lite86}".default or false); }
    ];
    rand_chacha = fold recursiveUpdate {} [
      { "0.2.2"."simd" =
        (f.rand_chacha."0.2.2"."simd" or false) ||
        (f.rand_chacha."0.2.2".default or false) ||
        (rand_chacha."0.2.2"."default" or false); }
      { "0.2.2"."std" =
        (f.rand_chacha."0.2.2"."std" or false) ||
        (f.rand_chacha."0.2.2".default or false) ||
        (rand_chacha."0.2.2"."default" or false); }
      { "0.2.2".default = (f.rand_chacha."0.2.2".default or true); }
    ];
    rand_core."${deps.rand_chacha."0.2.2".rand_core}".default = true;
  }) [
    (features_.ppv_lite86."${deps."rand_chacha"."0.2.2"."ppv_lite86"}" deps)
    (features_.rand_core."${deps."rand_chacha"."0.2.2"."rand_core"}" deps)
  ];


# end
# rand_core-0.5.1

  crates.rand_core."0.5.1" = deps: { features?(features_.rand_core."0.5.1" deps {}) }: buildRustCrate {
    crateName = "rand_core";
    version = "0.5.1";
    description = "Core random number generator traits and tools for implementation.\n";
    authors = [ "The Rand Project Developers" "The Rust Project Developers" ];
    edition = "2018";
    sha256 = "19qfnh77bzz0x2gfsk91h0gygy0z1s5l3yyc2j91gmprq60d6s3r";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.rand_core."0.5.1".getrandom or false then [ (crates.getrandom."${deps."rand_core"."0.5.1".getrandom}" deps) ] else []));
    features = mkFeatures (features."rand_core"."0.5.1" or {});
  };
  features_.rand_core."0.5.1" = deps: f: updateFeatures f (rec {
    getrandom = fold recursiveUpdate {} [
      { "${deps.rand_core."0.5.1".getrandom}"."std" =
        (f.getrandom."${deps.rand_core."0.5.1".getrandom}"."std" or false) ||
        (rand_core."0.5.1"."std" or false) ||
        (f."rand_core"."0.5.1"."std" or false); }
      { "${deps.rand_core."0.5.1".getrandom}".default = true; }
    ];
    rand_core = fold recursiveUpdate {} [
      { "0.5.1"."alloc" =
        (f.rand_core."0.5.1"."alloc" or false) ||
        (f.rand_core."0.5.1".std or false) ||
        (rand_core."0.5.1"."std" or false); }
      { "0.5.1"."getrandom" =
        (f.rand_core."0.5.1"."getrandom" or false) ||
        (f.rand_core."0.5.1".std or false) ||
        (rand_core."0.5.1"."std" or false); }
      { "0.5.1"."serde" =
        (f.rand_core."0.5.1"."serde" or false) ||
        (f.rand_core."0.5.1".serde1 or false) ||
        (rand_core."0.5.1"."serde1" or false); }
      { "0.5.1".default = (f.rand_core."0.5.1".default or true); }
    ];
  }) [
    (features_.getrandom."${deps."rand_core"."0.5.1"."getrandom"}" deps)
  ];


# end
# rand_hc-0.2.0

  crates.rand_hc."0.2.0" = deps: { features?(features_.rand_hc."0.2.0" deps {}) }: buildRustCrate {
    crateName = "rand_hc";
    version = "0.2.0";
    description = "HC128 random number generator\n";
    authors = [ "The Rand Project Developers" ];
    edition = "2018";
    sha256 = "0592q9kqcna9aiyzy6vp3fadxkkbpfkmi2cnkv48zhybr0v2yf01";
    dependencies = mapFeatures features ([
      (crates."rand_core"."${deps."rand_hc"."0.2.0"."rand_core"}" deps)
    ]);
  };
  features_.rand_hc."0.2.0" = deps: f: updateFeatures f (rec {
    rand_core."${deps.rand_hc."0.2.0".rand_core}".default = true;
    rand_hc."0.2.0".default = (f.rand_hc."0.2.0".default or true);
  }) [
    (features_.rand_core."${deps."rand_hc"."0.2.0"."rand_core"}" deps)
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
# ryu-1.0.4

  crates.ryu."1.0.4" = deps: { features?(features_.ryu."1.0.4" deps {}) }: buildRustCrate {
    crateName = "ryu";
    version = "1.0.4";
    description = "Fast floating point to string conversion";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1kv9ca7vwsgaggrjcm656ym3w93pazd35fq14mx6cr42k8s41145";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."ryu"."1.0.4" or {});
  };
  features_.ryu."1.0.4" = deps: f: updateFeatures f (rec {
    ryu."1.0.4".default = (f.ryu."1.0.4".default or true);
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
# schannel-0.1.18

  crates.schannel."0.1.18" = deps: { features?(features_.schannel."0.1.18" deps {}) }: buildRustCrate {
    crateName = "schannel";
    version = "0.1.18";
    description = "Schannel bindings for rust, allowing SSL/TLS (e.g. https) without openssl";
    authors = [ "Steven Fackler <sfackler@gmail.com>" "Steffen Butzer <steffen.butzer@outlook.com>" ];
    sha256 = "1b4gj4080gc2s3y170d6q9d3s4wrmmgj6ybpwh17rk8sx8vawdzv";
    dependencies = mapFeatures features ([
      (crates."lazy_static"."${deps."schannel"."0.1.18"."lazy_static"}" deps)
      (crates."winapi"."${deps."schannel"."0.1.18"."winapi"}" deps)
    ]);
  };
  features_.schannel."0.1.18" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.schannel."0.1.18".lazy_static}".default = true;
    schannel."0.1.18".default = (f.schannel."0.1.18".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.schannel."0.1.18".winapi}"."lmcons" = true; }
      { "${deps.schannel."0.1.18".winapi}"."minschannel" = true; }
      { "${deps.schannel."0.1.18".winapi}"."schannel" = true; }
      { "${deps.schannel."0.1.18".winapi}"."securitybaseapi" = true; }
      { "${deps.schannel."0.1.18".winapi}"."sspi" = true; }
      { "${deps.schannel."0.1.18".winapi}"."sysinfoapi" = true; }
      { "${deps.schannel."0.1.18".winapi}"."timezoneapi" = true; }
      { "${deps.schannel."0.1.18".winapi}"."winbase" = true; }
      { "${deps.schannel."0.1.18".winapi}"."wincrypt" = true; }
      { "${deps.schannel."0.1.18".winapi}"."winerror" = true; }
      { "${deps.schannel."0.1.18".winapi}".default = true; }
    ];
  }) [
    (features_.lazy_static."${deps."schannel"."0.1.18"."lazy_static"}" deps)
    (features_.winapi."${deps."schannel"."0.1.18"."winapi"}" deps)
  ];


# end
# scopeguard-1.1.0

  crates.scopeguard."1.1.0" = deps: { features?(features_.scopeguard."1.1.0" deps {}) }: buildRustCrate {
    crateName = "scopeguard";
    version = "1.1.0";
    description = "A RAII scope guard that will run a given closure when it goes out of scope,\neven if the code between panics (assuming unwinding panic).\n\nDefines the macros `defer!`, `defer_on_unwind!`, `defer_on_success!` as\nshorthands for guards with one of the implemented strategies.\n";
    authors = [ "bluss" ];
    sha256 = "1smjw88w17v19g0ya4hv8c74q4z8pg7vcj0xqdn1bwk71xsg5pih";
    features = mkFeatures (features."scopeguard"."1.1.0" or {});
  };
  features_.scopeguard."1.1.0" = deps: f: updateFeatures f (rec {
    scopeguard = fold recursiveUpdate {} [
      { "1.1.0"."use_std" =
        (f.scopeguard."1.1.0"."use_std" or false) ||
        (f.scopeguard."1.1.0".default or false) ||
        (scopeguard."1.1.0"."default" or false); }
      { "1.1.0".default = (f.scopeguard."1.1.0".default or true); }
    ];
  }) [];


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
# security-framework-0.4.3

  crates.security_framework."0.4.3" = deps: { features?(features_.security_framework."0.4.3" deps {}) }: buildRustCrate {
    crateName = "security-framework";
    version = "0.4.3";
    description = "Security.framework bindings for macOS and iOS";
    authors = [ "Steven Fackler <sfackler@gmail.com>" "Kornel <kornel@geekhood.net>" ];
    sha256 = "1hwbxd5v6bzqnp3mw5mmc2dzrgjlbm5pplbpfk5cg9i9c6isqzjq";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."security_framework"."0.4.3"."bitflags"}" deps)
      (crates."core_foundation"."${deps."security_framework"."0.4.3"."core_foundation"}" deps)
      (crates."core_foundation_sys"."${deps."security_framework"."0.4.3"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."security_framework"."0.4.3"."libc"}" deps)
      (crates."security_framework_sys"."${deps."security_framework"."0.4.3"."security_framework_sys"}" deps)
    ]);
    features = mkFeatures (features."security_framework"."0.4.3" or {});
  };
  features_.security_framework."0.4.3" = deps: f: updateFeatures f (rec {
    bitflags."${deps.security_framework."0.4.3".bitflags}".default = true;
    core_foundation."${deps.security_framework."0.4.3".core_foundation}".default = true;
    core_foundation_sys."${deps.security_framework."0.4.3".core_foundation_sys}".default = true;
    libc."${deps.security_framework."0.4.3".libc}".default = true;
    security_framework = fold recursiveUpdate {} [
      { "0.4.3"."OSX_10_10" =
        (f.security_framework."0.4.3"."OSX_10_10" or false) ||
        (f.security_framework."0.4.3".OSX_10_11 or false) ||
        (security_framework."0.4.3"."OSX_10_11" or false); }
      { "0.4.3"."OSX_10_11" =
        (f.security_framework."0.4.3"."OSX_10_11" or false) ||
        (f.security_framework."0.4.3".OSX_10_12 or false) ||
        (security_framework."0.4.3"."OSX_10_12" or false); }
      { "0.4.3"."OSX_10_12" =
        (f.security_framework."0.4.3"."OSX_10_12" or false) ||
        (f.security_framework."0.4.3".OSX_10_13 or false) ||
        (security_framework."0.4.3"."OSX_10_13" or false); }
      { "0.4.3"."OSX_10_9" =
        (f.security_framework."0.4.3"."OSX_10_9" or false) ||
        (f.security_framework."0.4.3".OSX_10_10 or false) ||
        (security_framework."0.4.3"."OSX_10_10" or false); }
      { "0.4.3"."alpn" =
        (f.security_framework."0.4.3"."alpn" or false) ||
        (f.security_framework."0.4.3".OSX_10_13 or false) ||
        (security_framework."0.4.3"."OSX_10_13" or false); }
      { "0.4.3"."session-tickets" =
        (f.security_framework."0.4.3"."session-tickets" or false) ||
        (f.security_framework."0.4.3".OSX_10_13 or false) ||
        (security_framework."0.4.3"."OSX_10_13" or false); }
      { "0.4.3".default = (f.security_framework."0.4.3".default or true); }
    ];
    security_framework_sys = fold recursiveUpdate {} [
      { "${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_10" =
        (f.security_framework_sys."${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_10" or false) ||
        (security_framework."0.4.3"."OSX_10_10" or false) ||
        (f."security_framework"."0.4.3"."OSX_10_10" or false); }
      { "${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_11" =
        (f.security_framework_sys."${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_11" or false) ||
        (security_framework."0.4.3"."OSX_10_11" or false) ||
        (f."security_framework"."0.4.3"."OSX_10_11" or false); }
      { "${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_12" =
        (f.security_framework_sys."${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_12" or false) ||
        (security_framework."0.4.3"."OSX_10_12" or false) ||
        (f."security_framework"."0.4.3"."OSX_10_12" or false); }
      { "${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_13" =
        (f.security_framework_sys."${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_13" or false) ||
        (security_framework."0.4.3"."OSX_10_13" or false) ||
        (f."security_framework"."0.4.3"."OSX_10_13" or false); }
      { "${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_9" =
        (f.security_framework_sys."${deps.security_framework."0.4.3".security_framework_sys}"."OSX_10_9" or false) ||
        (security_framework."0.4.3"."OSX_10_9" or false) ||
        (f."security_framework"."0.4.3"."OSX_10_9" or false); }
      { "${deps.security_framework."0.4.3".security_framework_sys}".default = true; }
    ];
  }) [
    (features_.bitflags."${deps."security_framework"."0.4.3"."bitflags"}" deps)
    (features_.core_foundation."${deps."security_framework"."0.4.3"."core_foundation"}" deps)
    (features_.core_foundation_sys."${deps."security_framework"."0.4.3"."core_foundation_sys"}" deps)
    (features_.libc."${deps."security_framework"."0.4.3"."libc"}" deps)
    (features_.security_framework_sys."${deps."security_framework"."0.4.3"."security_framework_sys"}" deps)
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
# security-framework-sys-0.4.3

  crates.security_framework_sys."0.4.3" = deps: { features?(features_.security_framework_sys."0.4.3" deps {}) }: buildRustCrate {
    crateName = "security-framework-sys";
    version = "0.4.3";
    description = "Apple `Security.framework` low-level FFI bindings";
    authors = [ "Steven Fackler <sfackler@gmail.com>" "Kornel <kornel@geekhood.net>" ];
    sha256 = "1qazvv2c93q8v74c91arqycprvh5bi7ch2j3x17nh1dc0vsbmpjm";
    dependencies = mapFeatures features ([
      (crates."core_foundation_sys"."${deps."security_framework_sys"."0.4.3"."core_foundation_sys"}" deps)
      (crates."libc"."${deps."security_framework_sys"."0.4.3"."libc"}" deps)
    ]);
    features = mkFeatures (features."security_framework_sys"."0.4.3" or {});
  };
  features_.security_framework_sys."0.4.3" = deps: f: updateFeatures f (rec {
    core_foundation_sys."${deps.security_framework_sys."0.4.3".core_foundation_sys}".default = true;
    libc."${deps.security_framework_sys."0.4.3".libc}".default = true;
    security_framework_sys = fold recursiveUpdate {} [
      { "0.4.3"."OSX_10_10" =
        (f.security_framework_sys."0.4.3"."OSX_10_10" or false) ||
        (f.security_framework_sys."0.4.3".OSX_10_11 or false) ||
        (security_framework_sys."0.4.3"."OSX_10_11" or false); }
      { "0.4.3"."OSX_10_11" =
        (f.security_framework_sys."0.4.3"."OSX_10_11" or false) ||
        (f.security_framework_sys."0.4.3".OSX_10_12 or false) ||
        (security_framework_sys."0.4.3"."OSX_10_12" or false); }
      { "0.4.3"."OSX_10_12" =
        (f.security_framework_sys."0.4.3"."OSX_10_12" or false) ||
        (f.security_framework_sys."0.4.3".OSX_10_13 or false) ||
        (security_framework_sys."0.4.3"."OSX_10_13" or false); }
      { "0.4.3"."OSX_10_9" =
        (f.security_framework_sys."0.4.3"."OSX_10_9" or false) ||
        (f.security_framework_sys."0.4.3".OSX_10_10 or false) ||
        (security_framework_sys."0.4.3"."OSX_10_10" or false); }
      { "0.4.3".default = (f.security_framework_sys."0.4.3".default or true); }
    ];
  }) [
    (features_.core_foundation_sys."${deps."security_framework_sys"."0.4.3"."core_foundation_sys"}" deps)
    (features_.libc."${deps."security_framework_sys"."0.4.3"."libc"}" deps)
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
# serde-1.0.106

  crates.serde."1.0.106" = deps: { features?(features_.serde."1.0.106" deps {}) }: buildRustCrate {
    crateName = "serde";
    version = "1.0.106";
    description = "A generic serialization/deserialization framework";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0rag3fqh2yhm9kb99phamhm7g7r0br3hvhjkijkv6nvvqnxnn3ri";
    build = "build.rs";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.serde."1.0.106".serde_derive or false then [ (crates.serde_derive."${deps."serde"."1.0.106".serde_derive}" deps) ] else []));
    features = mkFeatures (features."serde"."1.0.106" or {});
  };
  features_.serde."1.0.106" = deps: f: updateFeatures f (rec {
    serde = fold recursiveUpdate {} [
      { "1.0.106"."serde_derive" =
        (f.serde."1.0.106"."serde_derive" or false) ||
        (f.serde."1.0.106".derive or false) ||
        (serde."1.0.106"."derive" or false); }
      { "1.0.106"."std" =
        (f.serde."1.0.106"."std" or false) ||
        (f.serde."1.0.106".default or false) ||
        (serde."1.0.106"."default" or false); }
      { "1.0.106".default = (f.serde."1.0.106".default or true); }
    ];
    serde_derive."${deps.serde."1.0.106".serde_derive}".default = true;
  }) [
    (features_.serde_derive."${deps."serde"."1.0.106"."serde_derive"}" deps)
  ];


# end
# serde_derive-1.0.106

  crates.serde_derive."1.0.106" = deps: { features?(features_.serde_derive."1.0.106" deps {}) }: buildRustCrate {
    crateName = "serde_derive";
    version = "1.0.106";
    description = "Macros 1.1 implementation of #[derive(Serialize, Deserialize)]";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "03wq260g5prkgxgfq4yhbmznqm2rr3qmhqah6mh6ddvpmq6axz3p";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."serde_derive"."1.0.106"."proc_macro2"}" deps)
      (crates."quote"."${deps."serde_derive"."1.0.106"."quote"}" deps)
      (crates."syn"."${deps."serde_derive"."1.0.106"."syn"}" deps)
    ]);
    features = mkFeatures (features."serde_derive"."1.0.106" or {});
  };
  features_.serde_derive."1.0.106" = deps: f: updateFeatures f (rec {
    proc_macro2."${deps.serde_derive."1.0.106".proc_macro2}".default = true;
    quote."${deps.serde_derive."1.0.106".quote}".default = true;
    serde_derive."1.0.106".default = (f.serde_derive."1.0.106".default or true);
    syn = fold recursiveUpdate {} [
      { "${deps.serde_derive."1.0.106".syn}"."visit" = true; }
      { "${deps.serde_derive."1.0.106".syn}".default = true; }
    ];
  }) [
    (features_.proc_macro2."${deps."serde_derive"."1.0.106"."proc_macro2"}" deps)
    (features_.quote."${deps."serde_derive"."1.0.106"."quote"}" deps)
    (features_.syn."${deps."serde_derive"."1.0.106"."syn"}" deps)
  ];


# end
# serde_json-1.0.52

  crates.serde_json."1.0.52" = deps: { features?(features_.serde_json."1.0.52" deps {}) }: buildRustCrate {
    crateName = "serde_json";
    version = "1.0.52";
    description = "A JSON serialization file format";
    authors = [ "Erick Tryzelaar <erick.tryzelaar@gmail.com>" "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "1idn87yjb9qhjcgm4hw5g0sc125l4yvml5s2z9jplh28hyz76r23";
    dependencies = mapFeatures features ([
      (crates."itoa"."${deps."serde_json"."1.0.52"."itoa"}" deps)
      (crates."ryu"."${deps."serde_json"."1.0.52"."ryu"}" deps)
      (crates."serde"."${deps."serde_json"."1.0.52"."serde"}" deps)
    ]);
    features = mkFeatures (features."serde_json"."1.0.52" or {});
  };
  features_.serde_json."1.0.52" = deps: f: updateFeatures f (rec {
    itoa."${deps.serde_json."1.0.52".itoa}".default = (f.itoa."${deps.serde_json."1.0.52".itoa}".default or false);
    ryu."${deps.serde_json."1.0.52".ryu}".default = true;
    serde = fold recursiveUpdate {} [
      { "${deps.serde_json."1.0.52".serde}"."alloc" =
        (f.serde."${deps.serde_json."1.0.52".serde}"."alloc" or false) ||
        (serde_json."1.0.52"."alloc" or false) ||
        (f."serde_json"."1.0.52"."alloc" or false); }
      { "${deps.serde_json."1.0.52".serde}"."std" =
        (f.serde."${deps.serde_json."1.0.52".serde}"."std" or false) ||
        (serde_json."1.0.52"."std" or false) ||
        (f."serde_json"."1.0.52"."std" or false); }
      { "${deps.serde_json."1.0.52".serde}".default = (f.serde."${deps.serde_json."1.0.52".serde}".default or false); }
    ];
    serde_json = fold recursiveUpdate {} [
      { "1.0.52"."indexmap" =
        (f.serde_json."1.0.52"."indexmap" or false) ||
        (f.serde_json."1.0.52".preserve_order or false) ||
        (serde_json."1.0.52"."preserve_order" or false); }
      { "1.0.52"."std" =
        (f.serde_json."1.0.52"."std" or false) ||
        (f.serde_json."1.0.52".default or false) ||
        (serde_json."1.0.52"."default" or false); }
      { "1.0.52".default = (f.serde_json."1.0.52".default or true); }
    ];
  }) [
    (features_.itoa."${deps."serde_json"."1.0.52"."itoa"}" deps)
    (features_.ryu."${deps."serde_json"."1.0.52"."ryu"}" deps)
    (features_.serde."${deps."serde_json"."1.0.52"."serde"}" deps)
  ];


# end
# sha-1-0.8.2

  crates.sha_1."0.8.2" = deps: { features?(features_.sha_1."0.8.2" deps {}) }: buildRustCrate {
    crateName = "sha-1";
    version = "0.8.2";
    description = "SHA-1 hash function";
    authors = [ "RustCrypto Developers" ];
    sha256 = "1w66c3fah5yj7as7djcvwnv2579p2qswksmnm5fr9hxhn43p1nw6";
    libName = "sha1";
    dependencies = mapFeatures features ([
      (crates."block_buffer"."${deps."sha_1"."0.8.2"."block_buffer"}" deps)
      (crates."digest"."${deps."sha_1"."0.8.2"."digest"}" deps)
      (crates."fake_simd"."${deps."sha_1"."0.8.2"."fake_simd"}" deps)
      (crates."opaque_debug"."${deps."sha_1"."0.8.2"."opaque_debug"}" deps)
    ]);
    features = mkFeatures (features."sha_1"."0.8.2" or {});
  };
  features_.sha_1."0.8.2" = deps: f: updateFeatures f (rec {
    block_buffer."${deps.sha_1."0.8.2".block_buffer}".default = true;
    digest = fold recursiveUpdate {} [
      { "${deps.sha_1."0.8.2".digest}"."std" =
        (f.digest."${deps.sha_1."0.8.2".digest}"."std" or false) ||
        (sha_1."0.8.2"."std" or false) ||
        (f."sha_1"."0.8.2"."std" or false); }
      { "${deps.sha_1."0.8.2".digest}".default = true; }
    ];
    fake_simd."${deps.sha_1."0.8.2".fake_simd}".default = true;
    opaque_debug."${deps.sha_1."0.8.2".opaque_debug}".default = true;
    sha_1 = fold recursiveUpdate {} [
      { "0.8.2"."asm" =
        (f.sha_1."0.8.2"."asm" or false) ||
        (f.sha_1."0.8.2".asm-aarch64 or false) ||
        (sha_1."0.8.2"."asm-aarch64" or false); }
      { "0.8.2"."libc" =
        (f.sha_1."0.8.2"."libc" or false) ||
        (f.sha_1."0.8.2".asm-aarch64 or false) ||
        (sha_1."0.8.2"."asm-aarch64" or false); }
      { "0.8.2"."sha1-asm" =
        (f.sha_1."0.8.2"."sha1-asm" or false) ||
        (f.sha_1."0.8.2".asm or false) ||
        (sha_1."0.8.2"."asm" or false); }
      { "0.8.2"."std" =
        (f.sha_1."0.8.2"."std" or false) ||
        (f.sha_1."0.8.2".default or false) ||
        (sha_1."0.8.2"."default" or false); }
      { "0.8.2".default = (f.sha_1."0.8.2".default or true); }
    ];
  }) [
    (features_.block_buffer."${deps."sha_1"."0.8.2"."block_buffer"}" deps)
    (features_.digest."${deps."sha_1"."0.8.2"."digest"}" deps)
    (features_.fake_simd."${deps."sha_1"."0.8.2"."fake_simd"}" deps)
    (features_.opaque_debug."${deps."sha_1"."0.8.2"."opaque_debug"}" deps)
  ];


# end
# slab-0.4.2

  crates.slab."0.4.2" = deps: { features?(features_.slab."0.4.2" deps {}) }: buildRustCrate {
    crateName = "slab";
    version = "0.4.2";
    description = "Pre-allocated storage for a uniform data type";
    authors = [ "Carl Lerche <me@carllerche.com>" ];
    sha256 = "0h1l2z7qy6207kv0v3iigdf2xfk9yrhbwj1svlxk6wxjmdxvgdl7";
  };
  features_.slab."0.4.2" = deps: f: updateFeatures f (rec {
    slab."0.4.2".default = (f.slab."0.4.2".default or true);
  }) [];


# end
# smallvec-1.4.0

  crates.smallvec."1.4.0" = deps: { features?(features_.smallvec."1.4.0" deps {}) }: buildRustCrate {
    crateName = "smallvec";
    version = "1.4.0";
    description = "'Small vector' optimization: store up to a small number of items on the stack";
    authors = [ "Simon Sapin <simon.sapin@exyr.org>" ];
    edition = "2018";
    sha256 = "1hq3fx46jnjcrcqwrkkwsaq10d6sz6ibccm3gmqni6map7cpcx45";
    libPath = "lib.rs";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."smallvec"."1.4.0" or {});
  };
  features_.smallvec."1.4.0" = deps: f: updateFeatures f (rec {
    smallvec."1.4.0".default = (f.smallvec."1.4.0".default or true);
  }) [];


# end
# socket2-0.3.12

  crates.socket2."0.3.12" = deps: { features?(features_.socket2."0.3.12" deps {}) }: buildRustCrate {
    crateName = "socket2";
    version = "0.3.12";
    description = "Utilities for handling networking sockets with a maximal amount of configuration\npossible intended.\n";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    edition = "2018";
    sha256 = "1raci7p3yi5yhcyz2sybn63px0kdy5wv7wjkcyhwhvzfxs9kd3gw";
    dependencies = (if (kernel == "linux" || kernel == "darwin") || kernel == "redox" then mapFeatures features ([
      (crates."cfg_if"."${deps."socket2"."0.3.12"."cfg_if"}" deps)
      (crates."libc"."${deps."socket2"."0.3.12"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."socket2"."0.3.12"."redox_syscall"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."socket2"."0.3.12"."winapi"}" deps)
    ]) else []);
    features = mkFeatures (features."socket2"."0.3.12" or {});
  };
  features_.socket2."0.3.12" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.socket2."0.3.12".cfg_if}".default = true;
    libc."${deps.socket2."0.3.12".libc}".default = true;
    redox_syscall."${deps.socket2."0.3.12".redox_syscall}".default = true;
    socket2."0.3.12".default = (f.socket2."0.3.12".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.socket2."0.3.12".winapi}"."handleapi" = true; }
      { "${deps.socket2."0.3.12".winapi}"."minwindef" = true; }
      { "${deps.socket2."0.3.12".winapi}"."ws2def" = true; }
      { "${deps.socket2."0.3.12".winapi}"."ws2ipdef" = true; }
      { "${deps.socket2."0.3.12".winapi}"."ws2tcpip" = true; }
      { "${deps.socket2."0.3.12".winapi}".default = true; }
    ];
  }) [
    (features_.cfg_if."${deps."socket2"."0.3.12"."cfg_if"}" deps)
    (features_.libc."${deps."socket2"."0.3.12"."libc"}" deps)
    (features_.redox_syscall."${deps."socket2"."0.3.12"."redox_syscall"}" deps)
    (features_.winapi."${deps."socket2"."0.3.12"."winapi"}" deps)
  ];


# end
# static_assertions-1.1.0

  crates.static_assertions."1.1.0" = deps: { features?(features_.static_assertions."1.1.0" deps {}) }: buildRustCrate {
    crateName = "static_assertions";
    version = "1.1.0";
    description = "Compile-time assertions to ensure that invariants are met.";
    authors = [ "Nikolai Vazquez" ];
    sha256 = "0ll610anmi0kskiz58sv98b5zbj1m890zzlnd4impc9r5241vxay";
    features = mkFeatures (features."static_assertions"."1.1.0" or {});
  };
  features_.static_assertions."1.1.0" = deps: f: updateFeatures f (rec {
    static_assertions."1.1.0".default = (f.static_assertions."1.1.0".default or true);
  }) [];


# end
# syn-1.0.18

  crates.syn."1.0.18" = deps: { features?(features_.syn."1.0.18" deps {}) }: buildRustCrate {
    crateName = "syn";
    version = "1.0.18";
    description = "Parser for Rust source code";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    edition = "2018";
    sha256 = "1gjbawjms202h3w4px8ni3ifn3p0fdqn2lp950jx4rcr8yh7nzhr";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."syn"."1.0.18"."proc_macro2"}" deps)
      (crates."unicode_xid"."${deps."syn"."1.0.18"."unicode_xid"}" deps)
    ]
      ++ (if features.syn."1.0.18".quote or false then [ (crates.quote."${deps."syn"."1.0.18".quote}" deps) ] else []));
    features = mkFeatures (features."syn"."1.0.18" or {});
  };
  features_.syn."1.0.18" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.syn."1.0.18".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.syn."1.0.18".proc_macro2}"."proc-macro" or false) ||
        (syn."1.0.18"."proc-macro" or false) ||
        (f."syn"."1.0.18"."proc-macro" or false); }
      { "${deps.syn."1.0.18".proc_macro2}".default = (f.proc_macro2."${deps.syn."1.0.18".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "${deps.syn."1.0.18".quote}"."proc-macro" =
        (f.quote."${deps.syn."1.0.18".quote}"."proc-macro" or false) ||
        (syn."1.0.18"."proc-macro" or false) ||
        (f."syn"."1.0.18"."proc-macro" or false); }
      { "${deps.syn."1.0.18".quote}".default = (f.quote."${deps.syn."1.0.18".quote}".default or false); }
    ];
    syn = fold recursiveUpdate {} [
      { "1.0.18"."clone-impls" =
        (f.syn."1.0.18"."clone-impls" or false) ||
        (f.syn."1.0.18".default or false) ||
        (syn."1.0.18"."default" or false); }
      { "1.0.18"."derive" =
        (f.syn."1.0.18"."derive" or false) ||
        (f.syn."1.0.18".default or false) ||
        (syn."1.0.18"."default" or false); }
      { "1.0.18"."parsing" =
        (f.syn."1.0.18"."parsing" or false) ||
        (f.syn."1.0.18".default or false) ||
        (syn."1.0.18"."default" or false); }
      { "1.0.18"."printing" =
        (f.syn."1.0.18"."printing" or false) ||
        (f.syn."1.0.18".default or false) ||
        (syn."1.0.18"."default" or false); }
      { "1.0.18"."proc-macro" =
        (f.syn."1.0.18"."proc-macro" or false) ||
        (f.syn."1.0.18".default or false) ||
        (syn."1.0.18"."default" or false); }
      { "1.0.18"."quote" =
        (f.syn."1.0.18"."quote" or false) ||
        (f.syn."1.0.18".printing or false) ||
        (syn."1.0.18"."printing" or false); }
      { "1.0.18".default = (f.syn."1.0.18".default or true); }
    ];
    unicode_xid."${deps.syn."1.0.18".unicode_xid}".default = true;
  }) [
    (features_.proc_macro2."${deps."syn"."1.0.18"."proc_macro2"}" deps)
    (features_.quote."${deps."syn"."1.0.18"."quote"}" deps)
    (features_.unicode_xid."${deps."syn"."1.0.18"."unicode_xid"}" deps)
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
# tcp-stream-0.15.0

  crates.tcp_stream."0.15.0" = deps: { features?(features_.tcp_stream."0.15.0" deps {}) }: buildRustCrate {
    crateName = "tcp-stream";
    version = "0.15.0";
    description = "mio's TcpStream on steroids";
    authors = [ "Marc-Antoine Perennou <Marc-Antoine@Perennou.com>" ];
    edition = "2018";
    sha256 = "0kn69r7wdgga9n49hcmxn461k9251hxcl1ig8fy74plcxavg95ch";
    libName = "tcp_stream";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."tcp_stream"."0.15.0"."cfg_if"}" deps)
      (crates."mio"."${deps."tcp_stream"."0.15.0"."mio"}" deps)
    ]
      ++ (if features.tcp_stream."0.15.0".native-tls or false then [ (crates.native_tls."${deps."tcp_stream"."0.15.0".native_tls}" deps) ] else []));
    features = mkFeatures (features."tcp_stream"."0.15.0" or {});
  };
  features_.tcp_stream."0.15.0" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.tcp_stream."0.15.0".cfg_if}".default = true;
    mio = fold recursiveUpdate {} [
      { "${deps.tcp_stream."0.15.0".mio}"."os-poll" = true; }
      { "${deps.tcp_stream."0.15.0".mio}"."tcp" = true; }
      { "${deps.tcp_stream."0.15.0".mio}".default = (f.mio."${deps.tcp_stream."0.15.0".mio}".default or false); }
    ];
    native_tls."${deps.tcp_stream."0.15.0".native_tls}".default = true;
    tcp_stream = fold recursiveUpdate {} [
      { "0.15.0"."native-tls" =
        (f.tcp_stream."0.15.0"."native-tls" or false) ||
        (f.tcp_stream."0.15.0".default or false) ||
        (tcp_stream."0.15.0"."default" or false); }
      { "0.15.0"."rustls-connector" =
        (f.tcp_stream."0.15.0"."rustls-connector" or false) ||
        (f.tcp_stream."0.15.0".rustls-native-certs or false) ||
        (tcp_stream."0.15.0"."rustls-native-certs" or false) ||
        (f.tcp_stream."0.15.0".rustls-webpki-roots-certs or false) ||
        (tcp_stream."0.15.0"."rustls-webpki-roots-certs" or false); }
      { "0.15.0"."rustls-native-certs" =
        (f.tcp_stream."0.15.0"."rustls-native-certs" or false) ||
        (f.tcp_stream."0.15.0".rustls or false) ||
        (tcp_stream."0.15.0"."rustls" or false); }
      { "0.15.0".default = (f.tcp_stream."0.15.0".default or true); }
    ];
  }) [
    (features_.cfg_if."${deps."tcp_stream"."0.15.0"."cfg_if"}" deps)
    (features_.mio."${deps."tcp_stream"."0.15.0"."mio"}" deps)
    (features_.native_tls."${deps."tcp_stream"."0.15.0"."native_tls"}" deps)
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
# tempfile-3.1.0

  crates.tempfile."3.1.0" = deps: { features?(features_.tempfile."3.1.0" deps {}) }: buildRustCrate {
    crateName = "tempfile";
    version = "3.1.0";
    description = "A library for managing temporary files and directories.";
    authors = [ "Steven Allen <steven@stebalien.com>" "The Rust Project Developers" "Ashley Mannix <ashleymannix@live.com.au>" "Jason White <jasonaw0@gmail.com>" ];
    edition = "2018";
    sha256 = "1r7ykxw90p5hm1g46i8ia33j5iwl3q252kbb6b074qhdav3sqndk";
    dependencies = mapFeatures features ([
      (crates."cfg_if"."${deps."tempfile"."3.1.0"."cfg_if"}" deps)
      (crates."rand"."${deps."tempfile"."3.1.0"."rand"}" deps)
      (crates."remove_dir_all"."${deps."tempfile"."3.1.0"."remove_dir_all"}" deps)
    ])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."tempfile"."3.1.0"."redox_syscall"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."tempfile"."3.1.0"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."tempfile"."3.1.0"."winapi"}" deps)
    ]) else []);
  };
  features_.tempfile."3.1.0" = deps: f: updateFeatures f (rec {
    cfg_if."${deps.tempfile."3.1.0".cfg_if}".default = true;
    libc."${deps.tempfile."3.1.0".libc}".default = true;
    rand."${deps.tempfile."3.1.0".rand}".default = true;
    redox_syscall."${deps.tempfile."3.1.0".redox_syscall}".default = true;
    remove_dir_all."${deps.tempfile."3.1.0".remove_dir_all}".default = true;
    tempfile."3.1.0".default = (f.tempfile."3.1.0".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.tempfile."3.1.0".winapi}"."fileapi" = true; }
      { "${deps.tempfile."3.1.0".winapi}"."handleapi" = true; }
      { "${deps.tempfile."3.1.0".winapi}"."winbase" = true; }
      { "${deps.tempfile."3.1.0".winapi}".default = true; }
    ];
  }) [
    (features_.cfg_if."${deps."tempfile"."3.1.0"."cfg_if"}" deps)
    (features_.rand."${deps."tempfile"."3.1.0"."rand"}" deps)
    (features_.remove_dir_all."${deps."tempfile"."3.1.0"."remove_dir_all"}" deps)
    (features_.redox_syscall."${deps."tempfile"."3.1.0"."redox_syscall"}" deps)
    (features_.libc."${deps."tempfile"."3.1.0"."libc"}" deps)
    (features_.winapi."${deps."tempfile"."3.1.0"."winapi"}" deps)
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
# typenum-1.12.0

  crates.typenum."1.12.0" = deps: { features?(features_.typenum."1.12.0" deps {}) }: buildRustCrate {
    crateName = "typenum";
    version = "1.12.0";
    description = "Typenum is a Rust library for type-level numbers evaluated at compile time. It currently supports bits, unsigned integers, and signed integers. It also provides a type-level array of type-level numbers, but its implementation is incomplete.";
    authors = [ "Paho Lurie-Gregg <paho@paholg.com>" "Andre Bogus <bogusandre@gmail.com>" ];
    sha256 = "13rzwc7c43mknd4wls71dd4v0psnwldavgkay0s9wy5jv89fjyxa";
    build = "build/main.rs";
    features = mkFeatures (features."typenum"."1.12.0" or {});
  };
  features_.typenum."1.12.0" = deps: f: updateFeatures f (rec {
    typenum."1.12.0".default = (f.typenum."1.12.0".default or true);
  }) [];


# end
# ucd-trie-0.1.3

  crates.ucd_trie."0.1.3" = deps: { features?(features_.ucd_trie."0.1.3" deps {}) }: buildRustCrate {
    crateName = "ucd-trie";
    version = "0.1.3";
    description = "A trie for storing Unicode codepoint sets and maps.\n";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    edition = "2018";
    sha256 = "1ggxyix5yy1ck0jpxmv37n4dvacx1qxvhjd3y92qawwmwj2wj240";
    features = mkFeatures (features."ucd_trie"."0.1.3" or {});
  };
  features_.ucd_trie."0.1.3" = deps: f: updateFeatures f (rec {
    ucd_trie = fold recursiveUpdate {} [
      { "0.1.3"."std" =
        (f.ucd_trie."0.1.3"."std" or false) ||
        (f.ucd_trie."0.1.3".default or false) ||
        (ucd_trie."0.1.3"."default" or false); }
      { "0.1.3".default = (f.ucd_trie."0.1.3".default or true); }
    ];
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
# unicode-xid-0.2.0

  crates.unicode_xid."0.2.0" = deps: { features?(features_.unicode_xid."0.2.0" deps {}) }: buildRustCrate {
    crateName = "unicode-xid";
    version = "0.2.0";
    description = "Determine whether characters have the XID_Start\nor XID_Continue properties according to\nUnicode Standard Annex #31.\n";
    authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
    sha256 = "1c85gb3p3qhbjvfyjb31m06la4f024jx319k10ig7n47dz2fk8v7";
    features = mkFeatures (features."unicode_xid"."0.2.0" or {});
  };
  features_.unicode_xid."0.2.0" = deps: f: updateFeatures f (rec {
    unicode_xid."0.2.0".default = (f.unicode_xid."0.2.0".default or true);
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
# url-2.1.1

  crates.url."2.1.1" = deps: { features?(features_.url."2.1.1" deps {}) }: buildRustCrate {
    crateName = "url";
    version = "2.1.1";
    description = "URL library for Rust, based on the WHATWG URL Standard";
    authors = [ "The rust-url developers" ];
    sha256 = "0sqrqxfhz6rsc79da0yvmvszspaym8yvlf0780zsv5w8sf86cmxh";
    dependencies = mapFeatures features ([
      (crates."idna"."${deps."url"."2.1.1"."idna"}" deps)
      (crates."matches"."${deps."url"."2.1.1"."matches"}" deps)
      (crates."percent_encoding"."${deps."url"."2.1.1"."percent_encoding"}" deps)
    ]);
  };
  features_.url."2.1.1" = deps: f: updateFeatures f (rec {
    idna."${deps.url."2.1.1".idna}".default = true;
    matches."${deps.url."2.1.1".matches}".default = true;
    percent_encoding."${deps.url."2.1.1".percent_encoding}".default = true;
    url."2.1.1".default = (f.url."2.1.1".default or true);
  }) [
    (features_.idna."${deps."url"."2.1.1"."idna"}" deps)
    (features_.matches."${deps."url"."2.1.1"."matches"}" deps)
    (features_.percent_encoding."${deps."url"."2.1.1"."percent_encoding"}" deps)
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
# vcpkg-0.2.8

  crates.vcpkg."0.2.8" = deps: { features?(features_.vcpkg."0.2.8" deps {}) }: buildRustCrate {
    crateName = "vcpkg";
    version = "0.2.8";
    description = "A library to find native dependencies in a vcpkg tree at build\ntime in order to be used in Cargo build scripts.\n";
    authors = [ "Jim McGrath <jimmc2@gmail.com>" ];
    sha256 = "08ghimjjrfgz20i5glcybka96wyx196c54vl62kn7xg0hsqk0aal";
  };
  features_.vcpkg."0.2.8" = deps: f: updateFeatures f (rec {
    vcpkg."0.2.8".default = (f.vcpkg."0.2.8".default or true);
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
# version_check-0.9.1

  crates.version_check."0.9.1" = deps: { features?(features_.version_check."0.9.1" deps {}) }: buildRustCrate {
    crateName = "version_check";
    version = "0.9.1";
    description = "Tiny crate to check the version of the installed/running rustc.";
    authors = [ "Sergio Benitez <sb@sergio.bz>" ];
    sha256 = "00aywbaicdi81gcxpqdz6g0l96885bz4bml5c9xfna5p01vrh4li";
  };
  features_.version_check."0.9.1" = deps: f: updateFeatures f (rec {
    version_check."0.9.1".default = (f.version_check."0.9.1".default or true);
  }) [];


# end
# wasi-0.9.0+wasi-snapshot-preview1

  crates.wasi."0.9.0+wasi-snapshot-preview1" = deps: { features?(features_.wasi."0.9.0+wasi-snapshot-preview1" deps {}) }: buildRustCrate {
    crateName = "wasi";
    version = "0.9.0+wasi-snapshot-preview1";
    description = "Experimental WASI API bindings for Rust";
    authors = [ "The Cranelift Project Developers" ];
    edition = "2018";
    sha256 = "0xa6b3rnsmhi13nvs9q51wmavx51yzs5qdbc7bvs0pvs6iar3hsd";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."wasi"."0.9.0+wasi-snapshot-preview1" or {});
  };
  features_.wasi."0.9.0+wasi-snapshot-preview1" = deps: f: updateFeatures f (rec {
    wasi = fold recursiveUpdate {} [
      { "0.9.0+wasi-snapshot-preview1"."compiler_builtins" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."compiler_builtins" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1".rustc-dep-of-std or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."core" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."core" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1".rustc-dep-of-std or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."rustc-std-workspace-alloc" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."rustc-std-workspace-alloc" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1".rustc-dep-of-std or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."rustc-dep-of-std" or false); }
      { "0.9.0+wasi-snapshot-preview1"."std" =
        (f.wasi."0.9.0+wasi-snapshot-preview1"."std" or false) ||
        (f.wasi."0.9.0+wasi-snapshot-preview1".default or false) ||
        (wasi."0.9.0+wasi-snapshot-preview1"."default" or false); }
      { "0.9.0+wasi-snapshot-preview1".default = (f.wasi."0.9.0+wasi-snapshot-preview1".default or true); }
    ];
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
# winapi-0.3.8

  crates.winapi."0.3.8" = deps: { features?(features_.winapi."0.3.8" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.3.8";
    description = "Raw FFI bindings for all of Windows API.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "084ialbgww1vxry341fmkg5crgpvab3w52ahx1wa54yqjgym0vxs";
    build = "build.rs";
    dependencies = (if kernel == "i686-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_i686_pc_windows_gnu"."${deps."winapi"."0.3.8"."winapi_i686_pc_windows_gnu"}" deps)
    ]) else [])
      ++ (if kernel == "x86_64-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_x86_64_pc_windows_gnu"."${deps."winapi"."0.3.8"."winapi_x86_64_pc_windows_gnu"}" deps)
    ]) else []);
    features = mkFeatures (features."winapi"."0.3.8" or {});
  };
  features_.winapi."0.3.8" = deps: f: updateFeatures f (rec {
    winapi = fold recursiveUpdate {} [
      { "0.3.8"."impl-debug" =
        (f.winapi."0.3.8"."impl-debug" or false) ||
        (f.winapi."0.3.8".debug or false) ||
        (winapi."0.3.8"."debug" or false); }
      { "0.3.8".default = (f.winapi."0.3.8".default or true); }
    ];
    winapi_i686_pc_windows_gnu."${deps.winapi."0.3.8".winapi_i686_pc_windows_gnu}".default = true;
    winapi_x86_64_pc_windows_gnu."${deps.winapi."0.3.8".winapi_x86_64_pc_windows_gnu}".default = true;
  }) [
    (features_.winapi_i686_pc_windows_gnu."${deps."winapi"."0.3.8"."winapi_i686_pc_windows_gnu"}" deps)
    (features_.winapi_x86_64_pc_windows_gnu."${deps."winapi"."0.3.8"."winapi_x86_64_pc_windows_gnu"}" deps)
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
# ws2_32-sys-0.2.1

  crates.ws2_32_sys."0.2.1" = deps: { features?(features_.ws2_32_sys."0.2.1" deps {}) }: buildRustCrate {
    crateName = "ws2_32-sys";
    version = "0.2.1";
    description = "Contains function definitions for the Windows API library ws2_32. See winapi for types and constants.";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "1zpy9d9wk11sj17fczfngcj28w4xxjs3b4n036yzpy38dxp4f7kc";
    libName = "ws2_32";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."winapi"."${deps."ws2_32_sys"."0.2.1"."winapi"}" deps)
    ]);

    buildDependencies = mapFeatures features ([
      (crates."winapi_build"."${deps."ws2_32_sys"."0.2.1"."winapi_build"}" deps)
    ]);
  };
  features_.ws2_32_sys."0.2.1" = deps: f: updateFeatures f (rec {
    winapi."${deps.ws2_32_sys."0.2.1".winapi}".default = true;
    winapi_build."${deps.ws2_32_sys."0.2.1".winapi_build}".default = true;
    ws2_32_sys."0.2.1".default = (f.ws2_32_sys."0.2.1".default or true);
  }) [
    (features_.winapi."${deps."ws2_32_sys"."0.2.1"."winapi"}" deps)
    (features_.winapi_build."${deps."ws2_32_sys"."0.2.1"."winapi_build"}" deps)
  ];


# end
}
