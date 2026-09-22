{ self }:

{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.skwd-deck;
in
{
  options.services.skwd-deck = {
    enable = lib.mkEnableOption "Skwd Deck user service";

    extraPackages = lib.mkOption {
      type = lib.types.listOf lib.types.package;
      default = [ ];
      description = "Optional Deck backends such as skwd-deck-steamworks.";
    };

    modelPackage = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.system}.skwd-lens-model;
      description = "Semantic model pack used by Skwd Lens. The default SigLIP 2 pack is installed with the suite.";
    };

  };

  config = lib.mkIf cfg.enable {

    environment.systemPackages = [
      self.packages.${pkgs.system}.default
      cfg.modelPackage
    ]
    ++ cfg.extraPackages;

    environment.sessionVariables.SKWD_LENS_HOME = "${cfg.modelPackage}/share/skwd-lens/models/semantic";

    systemd.user.services.skwd-walld = {

      description = "skwd-wall control daemon";
      conflicts = [ "skwd-daemon.service" ];
      wantedBy = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      partOf = [ "graphical-session.target" ];
      path = [
        self.packages.${pkgs.system}.paper
        self.packages.${pkgs.system}.lens
      ]
      ++ cfg.extraPackages;

      environment = {
        SKWD_LENS_HOME = "${cfg.modelPackage}/share/skwd-lens/models/semantic";
      };

      serviceConfig = {
        ExecStart = "${self.packages.${pkgs.system}.deck}/bin/skwd-walld";
        Restart = "on-failure";
      };
    };

  };
}
