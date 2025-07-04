#!/usr/bin/bash
# Main Panty
clear

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[0;37m'
export NC='\033[0m' # No Color

# Commands

hea1() {
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
}

# ////////////////// Rovo ///////////////////////////

rovo_install() {
    hea1 "Rovo Installation with packages"fish

    install_commands() {
        sudo apt-get install -y wget gnupg2
        sudo mkdir -p -m 755 /etc/apt/keyrings
        wget -nv -O- https://acli.atlassian.com/gpg/public-key.asc | sudo gpg --dearmor -o /etc/apt/keyrings/acli-archive-keyring.gpg
        sudo chmod go+r /etc/apt/keyrings/acli-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/acli-archive-keyring.gpg] https://acli.atlassian.com/linux/deb stable main" | sudo tee /etc/apt/sources.list.d/acli.list >/dev/null
        sudo apt update
        sudo apt install -y acli
    }

    echo -e "${GREEN} Executing Install: ${NC}"
    install_commands

    echo -e "${GREEN}***** Installation Completed *****${NC}"
    echo -e "+++++++++++++++++++++++++++++++++++++++++++++++++++++++"
    echo -e "${BLUE} "
    echo -e "Rovo Installed Successfully"
    echo -e "1. Gety Atlassian API"
    echo -e "2. Run Auth Login - acli rovodev auth login "
    echo -e "3. Get API - https://id.atlassian.com/manage-profile/security/api-tokens "
    echo -e "3. Execute - acli rovodev run"
    echo -e "4. Signup for Rovo Agents -  https://www.atlassian.com/try/cloud/signup?bundle=devai"
    echo -e "+++++++++++++++++++++++++++++++++++++++++++++++++++++++"
}

rovo_run() {
    hea1 "Rovo Run"
    echo -e "${GREEN} Executing Run: ${NC}"
    co1="acli rovodev --help"
    echo -e "--- Executing ${co1} ---"
    eval "$co1"
    echo -e "${GREEN}***** Installation Completed *****${NC}"
}

# ///////////////// Gemini Api //////////////////////////

gemini_install() {
    hea1 "Gemini Installation"
    echo -e "${GREEN} Executing Install: ${NC}"
    co1="bun install -g @google/gemini-cli"
    echo -e "--- Executing ${co1} ---"
    eval "$co1"
    echo -e "${GREEN}***** Installation Completed *****${NC}"
}

# ////////////////// UV Setup ///////////////////////////

## UV Tavily Setup
uv_tavily_setup() {
    hea1 "UV Tavily Installation with packages"

    # Get Name of project
    echo -e "Enter the name of the project: "
    read -r name_of_project
    if [ -z "$name_of_project" ]; then
        echo -e "${RED}BASTARD ! Project name cannot be empty${NC}"
        exit 1
    fi

    # UC Commands
    CO1="uv init $name_of_project"
    CO2="cd $name_of_project"

    DEPS="rich tavily-python python-dotenv"
    CO3="uv add  $DEPS"
    CO4="uv tree"

    ## RUN Above Commands
    echo -e "--- Executing ${CO1} ---"
    eval "$CO1"
    echo -e "--- Executing ${CO2} ---"
    eval "$CO2"
    echo -e "--- Executing ${CO3} ---"
    eval "$CO3"
    echo -e "--- Executing ${CO4} ---"
    eval "$CO4"
    echo -e "${GREEN}***** Installation Completed *****${NC}"
}

# Execution
gemini_install
rovo_install
