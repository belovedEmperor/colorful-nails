{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = with pkgs; [
    leptosfmt
    cargo-leptos
    trunk
    wasm-bindgen-cli_0_2_121

    tailwindcss
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = ["wasm32-unknown-unknown"];
  };

  services.postgres = {
    enable = true;
    listen_addresses = "localhost";
    port = 5432;
    initialDatabases = [
      {
        name = "colorful-nails";
        schema = ./migrations/20260602170730_create_appointments.sql;
      }
    ];
  };

  env = {
    DATABASE_URL = "postgresql://${builtins.getEnv "USER"}@localhost:5432/colorful-nails";
  };

  # See full reference at https://devenv.sh/reference/options/
}
