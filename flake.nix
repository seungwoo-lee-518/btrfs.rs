{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: {
    devShells.x86_64-linux.defeault = nixpkgs.legacyPackages.x86_64-linux.mkShell {
      nativeBuildInputs = with nixpkgs.legacyPackages.x86_64-linux; [
        cargo
        rustc
        rustfmt
        rust-analyzer

        openssl
        pkg-config
      ];
    };
  };
}
