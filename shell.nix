{ system, nixpkgs, edinix }:

let
  pkgs = import nixpkgs { inherit system; };
  emacs = edinix.emacs.${system} {
    profiles.nix.enable = true;
    profiles.clojure.enable = true;
  };
in pkgs.mkShell {
  buildInputs = with pkgs; [ babashka emacs.editor emacs.tooling ];
}
