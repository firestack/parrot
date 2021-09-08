{
	description = "A command-line snapshot testing tool for your scripts and CLI programs.";
	
	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
	inputs.flkutl.url = "github:numtide/flake-utils";

	outputs = { self, nixpkgs, flkutl }: flkutl.lib.eachSystem ["x86_64-linux"] (system: let
		pkgs = import nixpkgs {
				inherit system;
		}; in {

			packages.parrot = pkgs.rustPlatform.buildRustPackage {
				src = ./.;
				pname = "parrot";
				version = "0.0.3";

				cargoLock = { lockFile = ./Cargo.lock; };
			};

			defaultPackage = self.packages.${system}.parrot;

	});
}