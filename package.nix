{
  rustPlatform,
  libGL,
  libxkbcommon,
  wayland,
  makeWrapper,
  lib
}:

rustPlatform.buildRustPackage {
  name = "reloc8";
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ makeWrapper ];

  buildInputs = [
    libGL
    libxkbcommon
    wayland
  ];

  postInstall = ''
    wrapProgram $out/bin/reloc8 --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ]}

  '';
}
