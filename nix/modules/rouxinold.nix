{ lib
, pkgs
, config
, ...
}: with lib; {
  options.services.rouxinold-bot = {
    enable = mkEnableOption "rouxinold-bot";
    envFile = mkOption {
      type = types.str;
      default = "/opt/rouxinold/.env";
    };
  };

  config = let
    rouxinold-bot = config.services.rouxinold-bot;
  in {
    systemd.services.rouxinold-bot = mkIf rouxinold-bot.enable {
      name = "rouxinold-bot";
      wants = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];
      enable = true;
      environment = {
        "ROUXINOLD_ENV_FILE" = rouxinold-bot.envFile;
      };
      serviceConfig = {
        ExecStart = "${pkgs.rouxinold-bot}/bin/rouxinold-bot";
        Restart = "always";
        User = "rouxinold";
        Group = "rouxinold";
      };
    };

    users.users.rouxinold = {
      name = "rouxinold";
      group = "rouxinold";
    };

    environment.systemPackages = with pkgs; [
      oci-cli
    ];
  };
}
