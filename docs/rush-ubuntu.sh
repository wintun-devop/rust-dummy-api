#!/bin/bash

# run ./rush-ubuntu.sh without sudo

#Update/Download package information from all configured sources
sudo apt update -y

#Upgrading the downloaded packages
sudo apt upgrade -y

#install necessary packages for prerequireties
sudo apt install -y wget curl make unzip network-manager gcc net-tools lsb-release ca-certificates apt-transport-https gnupg2 software-properties-common

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source "$HOME/.cargo/env"

rustc --version

cargo --version