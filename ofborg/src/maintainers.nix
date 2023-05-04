{ changedattrsjson, changedpathsjson }:
let
  pkgs = import ./. {};
  moduleMaintainers = (import ./nixos {
    configuration = {};
  }).meta.maintainers;

  changedattrs = builtins.fromJSON (builtins.readFile changedattrsjson);
  changedpaths = builtins.fromJSON (builtins.readFile changedpathsjson);

  anyMatchingFile = filename:
    let
      matching = builtins.filter
        (changed: pkgs.lib.strings.hasSuffix changed filename)
        changedpaths;
    in (builtins.length matching) > 0;

  anyMatchingFiles = files:
    (builtins.length (builtins.filter anyMatchingFile files)) > 0;

  enrichedAttrs = builtins.map
    (path: {
      path = path;
      name = builtins.concatStringsSep "." path;
    })
    changedattrs;

  validPackageAttributes = builtins.filter
    (pkg:
      if (pkgs.lib.attrsets.hasAttrByPath pkg.path pkgs)
      then (if (builtins.tryEval (pkgs.lib.attrsets.attrByPath pkg.path null pkgs)).success
        then true
        else builtins.trace "Failed to access ${pkg.name} even though it exists" false)
      else builtins.trace "Failed to locate ${pkg.name}." false
    )
    enrichedAttrs;

  attrsWithPackages = builtins.map
    (pkg: pkg // { package = pkgs.lib.attrsets.attrByPath pkg.path null pkgs; })
    validPackageAttributes;

  attrsWithMaintainers = builtins.map
    (pkg: pkg // { maintainers = (pkg.package.meta or {}).maintainers or []; })
    attrsWithPackages;

  attrsWeCanPing = builtins.filter
    (pkg: if (builtins.length pkg.maintainers) > 0
      then true
      else builtins.trace "Package has no maintainers: ${pkg.name}" false
    )
    attrsWithMaintainers;

  relevantFilenames = drv:
    (pkgs.lib.lists.unique
      (builtins.map
        (pos: pos.file)
        (builtins.filter (x: x != null)
          [
            (builtins.unsafeGetAttrPos "maintainers" (drv.meta or {}))
            (builtins.unsafeGetAttrPos "src" drv)
            # broken because name is always set by stdenv:
            #    # A hack to make `nix-env -qa` and `nix search` ignore broken packages.
            #    # TODO(@oxij): remove this assert when something like NixOS/nix#1771 gets merged into nix.
            #    name = assert validity.handled; name + lib.optionalString
            #(builtins.unsafeGetAttrPos "name" drv)
            (builtins.unsafeGetAttrPos "pname" drv)
            (builtins.unsafeGetAttrPos "version" drv)
          ]
        )));

  attrsWithFilenames = builtins.map
    (pkg: pkg // { filenames = relevantFilenames pkg.package; })
    attrsWithMaintainers;

  attrsWithModifiedFiles = builtins.filter
    (pkg: anyMatchingFiles pkg.filenames)
    attrsWithFilenames;

  modulesWithModifiedFiles = pkgs.lib.filterAttrs
    (filename: _: anyMatchingFiles filename)
    moduleMaintainers;

  modulesWithModifiedFiles' = builtins.attrValues (pkgs.lib.attrsets.mapAttrs
    (filename: maintainers:
      pkgs.lib.attrsets.nameValuePair filename {
        filenames = [ filename ];
        entityName = filename;
        inherit maintainers;
      }
    ) modulesWithModifiedFiles);

  listToPing = modifiedAttrs: pkgs.lib.lists.flatten
    (builtins.map
      (pkg:
        builtins.map (maintainer: {
          handle = pkgs.lib.toLower maintainer.github;
          entityName = pkg.name;
          dueToFiles = pkg.filenames;
        })
        pkg.maintainers
      )
      modifiedAttrs);

  byMaintainer = pingList: pkgs.lib.lists.foldr
    (ping: collector: collector // { "${ping.handle}" = [ { inherit (ping) entityName dueToFiles; } ] ++ (collector."${ping.handle}" or []); })
    {}
    pingList;

  textForPackages = packages:
    pkgs.lib.strings.concatStringsSep ", " (
      builtins.map (pkg: pkg.entityName)
      packages);

  textPerMaintainer = pkgs.lib.attrsets.mapAttrs
    (maintainer: packages: "- @${maintainer} for ${textForPackages packages}")
    byMaintainer;

  packagesPerMaintainer = pkgs.lib.attrsets.mapAttrs
    (maintainer: packages:
      builtins.map (pkg: pkg.entityName)
      packages)
  (byMaintainer (listToPing attrsWithModifiedFiles));

  modulesPerMaintainer = pkgs.lib.attrsets.mapAttrs
    (maintainer: modules:
      builtins.map (module: module.entityName)
      modules
    )
    (byMaintainer (listToPing modulesWithModifiedFiles'));
in {
  inherit packagesPerMaintainer modulesPerMaintainer;
}
