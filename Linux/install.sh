red() { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }
yellow() { printf '\033[33m%s\033[0m\n' "$*"; }
blue() { printf '\033[34m%s\033[0m\n' "$*"; }

if [ "$EUID" -ne 0 ]; then
    red "The installation script requires root permission. Enhancing permission..."
    exec sudo -E "$0" "$@"
else
    yellow "Now running as root"
fi

install_path='/opt/NoitaSaveManager'
current_path="$(pwd)"
icon_url='https://noita.wiki.gg/images/Spell_nuke.png?7cb2d5&format=original'
icon_path='res/Nuke.png'
desktop_entry_url="https://raw.githubusercontent.com/Xiaomony/NoitaSaveManager/main/Linux/noita-save-manager.desktop"
desktop_entry_name='noita-save-manager.desktop'
desktop_entry_link_path="/usr/share/applications/${desktop_entry_name}"
echo "Install the program to '${install_path}'"

# existing installed files detection
if [ -e "${install_path}" ]; then
    confirm_prompt="$(yellow "The install path '${install_path}' exists.
Proceeding installation will remove them first(y/n):")"
    read -p "$confirm_prompt" confirm
    if [ "$confirm" = "y" ] || [ "$confirm" = "yes" ]; then
        rm -rf "${install_path}"
        green "'${install_path}' removed"
    else
        exit 0
    fi
fi

# give all users read permission for the following created files and dirs
umask 022

mkdir "${install_path}"
green "'${install_path}' created"
cd "${install_path}" || {
    red "Fail to cd into '${install_path}"
    exit 1
}

mkdir "res"
# download icon from noita wiki
blue "Downloading application icon from noita wiki"
blue "============================================"
if curl -fLo "${icon_path}" "${icon_url}"; then
    green "Icon downloaded"
else
    printf "\n"
    red "Fail to download application icon. Please check your network connection,
or download it manually from '${icon_url}'.
Then put it at '${install_path}${icon_path}'"
fi
blue "============================================"

# download desktop entry file from github
blue "Downloading \`${desktop_entry_name}\` file from github"
blue "============================================"
if curl -fLo "${desktop_entry_name}" "${desktop_entry_url}"; then
    green "\`${desktop_entry_name}\` file downloaded"
else
    printf "\n"
    red "Fail to download \`${desktop_entry_name}\` file. Please check your network connection,
or download it manually from '${desktop_entry_url}'.
Then put it at '${install_path}'"
fi
blue "============================================"

# link desktop entry
ln -sf "${install_path}/${desktop_entry_name}" "${desktop_entry_link_path}"
green "\`${desktop_entry_name}\` has been linked to '${desktop_entry_link_path}'"

cd "${current_path}" || exit 1
