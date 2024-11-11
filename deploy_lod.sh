#!/bin/bash

echo "Resolving dependencies"
yum install -y \
        make \
        gcc \
        zlib-devel \
        bzip2 \
        bzip2-devel \
        readline-devel \
        sqlite \
        sqlite-devel \
        openssl11-devel \
        tk-devel \
        libffi-devel \
        git

echo "Installing Pyenv"
curl https://pyenv.run | bash
cd ~/.pyenv && src/configure && make -C src
cd
echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bashrc
echo 'command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bashrc
echo 'eval "$(pyenv init -)"' >> ~/.bashrc
echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.profile
echo 'command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.profile
echo 'eval "$(pyenv init -)"' >> ~/.profile
echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bash_profile
echo '[[ -d $PYENV_ROOT/bin ]] && export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bash_profile
echo 'eval "$(pyenv init -)"' >> ~/.bash_profile
source ~/.bashrc

echo "Installing latest Python 3.12"
CPPFLAGS="$(pkg-config --cflags openssl11)" \
LDFLAGS="$(pkg-config --libs openssl11)" \
pyenv install -v 3.12:latest
pyenv global 3.12
pyenv local 3.12

echo "Installing uv"
curl -LsSf https://astral.sh/uv/install.sh | sh
source $HOME/.local/bin/env

echo "Installing Ansible & other libraries"
uv pip install --system ansible netapp_ontap
source ~/.bashrc


echo "Installing Netapp-Ontap collection"
ansible-galaxy collection install netapp.ontap