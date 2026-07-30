{ system, attrsJSON, ... }:
let
  lib = import <ofborg-nixpkgs-pr/lib>;
  source = import <ofborg-nixpkgs-pr>;
  pkgs = if builtins.isFunction source then source { inherit system; } else source;
  attrPaths = builtins.fromJSON attrsJSON;

  lookupAttrPath = path: value:
    if path == [ ] then
      { found = true; inherit value; }
    else
      let
        name = builtins.head path;
      in
      if builtins.isAttrs value && builtins.hasAttr name value then
        lookupAttrPath (builtins.tail path) (builtins.getAttr name value)
      else
        { found = false; };

  isDerivation = value:
    builtins.isAttrs value && value.type or null == "derivation";

  instantiateValue = value:
    if isDerivation value then
      builtins.deepSeq value.drvPath true
    else if builtins.isAttrs value then
      builtins.deepSeq (map instantiateValue (builtins.attrValues value)) true
    else if builtins.isList value then
      builtins.deepSeq (map instantiateValue value) true
    else
      false;

  isInstantiable = path:
    let
      attempted = builtins.tryEval (
        let
          lookup = lookupAttrPath path pkgs;
        in
        lookup.found && instantiateValue lookup.value
      );
    in
    attempted.success && attempted.value;
in
if builtins.elem system lib.systems.flakeExposed then
  builtins.seq pkgs (builtins.any isInstantiable attrPaths)
else
  false
