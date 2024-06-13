{ config, pkgs, ... }:

{
  home.packages = with pkgs; [
    git
    curl
    neovim
    bash
  ];

  home.file.".bashrc".text = builtins.readFile ./dotfiles/default/bashrc;
  home.file.".config/nvim/init.lua".text = builtins.readFile ./dotfiles/default/nvim/init.lua;
}
