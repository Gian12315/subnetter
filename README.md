# Subnetter
This is a program that calculates the subnetworks of a network given the ipv4 network, mask and ammount of subnetworks to be calculated.
You can print the results to the terminal (default), or exporting them to a csv file if you ever need to.

## Usage

    USAGE:
        subnetter [OPTIONS] <ip> <mask> <subnetworks>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -p <path>        Save to a csv file

    ARGS:
        <ip>             IP to be subnetted
        <mask>           Mask of the IP to be subnetted
        <subnetworks>    Number of subnetworks to be computed

## Why?
On school I had a homework which was to do what this program does, it was boring and repetitive and I figured that I could calculate what was needed by using a program, because it was simple to do and didn't requiere much effort, but was very very boring.

