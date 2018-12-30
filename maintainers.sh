#!/usr/bin/env nix-shell
#!nix-shell -i bash -p jq -p curl

set -euxo pipefail

statuses_url=$(curl --fail -L "https://api.github.com/repos/nixos/nixpkgs/pulls/$1" \
                   | jq -r .statuses_url)

changed_url=$(curl --fail -L "$statuses_url" \
                  | jq -r '.[] | select (.state == "success" and .description == "^.^!" and .target_url != "") | .target_url' | head -n1 \
                  | sed -e 's#gist.github.com#api.github.com/gists#' \
       )
changed_text_url=$(curl --fail -L "$changed_url" \
        | jq -r '.files."Changed Paths".raw_url')

if [ "x$changed_url" == "x" ]; then
   echo "Nothing changed"
   exit 1
fi

(
    cat <<EOF
let
pkgs = import ./default.nix {};

EOF
    echo 'changedattrs = ['
    curl --fail -L "$changed_text_url" \
        | cut -f2 \
        | sort | uniq \
        | sed -e 's/\./" "/g' | sed -e 's/^/[ "/' | sed -e 's/$/" ]/'
    echo '];

changedpaths = ['

    curl --fail -L "https://github.com/NixOS/nixpkgs/pull/$1.patch" \
        | grep "^+++ b/"  | sed -e "s/^+++ b//" \
        | sed -e 's/^/"/' | sed -e 's/$/"/' \
        | sort | uniq

    echo '];'

    cat <<'EOF'

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
  name = "pkgs.${builtins.concatStringsSep "." path}";
})
changedattrs;

validPackageAttributes = builtins.filter
  (pkg:
  if (pkgs.lib.attrsets.hasAttrByPath pkg.path pkgs) then
  (if (builtins.tryEval (pkgs.lib.attrsets.attrByPath pkg.path null pkgs)).success
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
          # broken because name is always set in stdenv:
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
  builtins.map
  (maintainer: {
    handle = maintainer.github;
    packageName = pkg.name;
    dueToFiles = pkg.filenames;
  })
  pkg.maintainers
)
attrsWithModifiedFiles);

byMaintainer = pkgs.lib.lists.foldr
  (ping: collector: collector // { "${ping.handle}" = [ { inherit (ping) packageName dueToFiles; } ] ++ (collector."${ping.handle}" or []); })
  {}
  listToPing;

textForPackages = packages:
pkgs.lib.strings.concatStringsSep ", " (
builtins.map
(pkg: pkg.packageName)
packages);

textPerMaintainer = pkgs.lib.attrsets.mapAttrs
(maintainer: packages: "- @${maintainer} for ${textForPackages packages}")
byMaintainer;

text = pkgs.lib.strings.concatStringsSep "\n" (builtins.attrValues textPerMaintainer);
in builtins.trace text ""
EOF
) > data.nix

nix-instantiate --eval ./data.nix
