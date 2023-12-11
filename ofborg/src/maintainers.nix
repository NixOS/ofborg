{ changedattrsjson, changedpathsjson }:
let
  pkgs = import ./. {};

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

  listToPing = pkgs.lib.lists.flatten
    (builtins.map
      (pkg:
        builtins.map (maintainer: {
          handle = pkgs.lib.toLower maintainer.github;
          packageName = pkg.name;
          dueToFiles = pkg.filenames;
        })
        (builtins.filter
          (maintainer:
            pkgs.lib.hasAttrByPath ["github"] maintainer)
          pkg.maintainers)
      )
      attrsWithModifiedFiles);

  byMaintainer = pkgs.lib.lists.foldr
    (ping: collector: collector // { "${ping.handle}" = [ { inherit (ping) packageName dueToFiles; } ] ++ (collector."${ping.handle}" or []); })
    {}
    listToPing;

  textForPackages = packages:
    pkgs.lib.strings.concatStringsSep ", " (
      builtins.map (pkg: pkg.packageName)
      packages);

  textPerMaintainer = pkgs.lib.attrsets.mapAttrs
    (maintainer: packages: "- @${maintainer} for ${textForPackages packages}")
    byMaintainer;

  packagesPerMaintainer = pkgs.lib.attrsets.mapAttrs
    (maintainer: packages:
      builtins.map (pkg: pkg.packageName)
      packages)
    byMaintainer;
in packagesPerMaintainer
