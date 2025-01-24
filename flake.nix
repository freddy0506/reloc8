{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };
  outputs = { nixpkgs, self, ... }:
  let 
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in
  {
    packages.x86_64-linux.reloc8 = pkgs.callPackage ./package.nix {};
    packages.x86_64-linux.default = self.packages.x86_64-linux.reloc8;
  };
}
