{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };
  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages."x86_64-linux".default = with pkgs; [
      libgcc
      cargo
      rustc
    ];
    devShell."x86_64-linux" = pkgs.mkShell {
      buildInputs = with pkgs; [
        libgcc
        cargo
        rustc
      ];
    };
  };
}
