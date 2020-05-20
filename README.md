# windows_rogue_dhcp
A rogue DHCP monitor made to run on Windows DHCP servers.

This is an application that currently just sends a DHCP request packet on every single IPV4 interface, then listens for a response. If it sees a response from a DHCP server that is not itself, it will list them.

The use case that prompted me to make this is that I manage dozens of Windows-based networks and wanted to monitor for rogue DHCP servers programmatically. When compiled, the executable can be used with a powershell script to create event logs or raise alerts using an RMM application.

Future features may be IPV6 support, monitoring on specific interfaces instead of all of them, or maybe a passive mode where it constantly just listens on port 58 for DHCP responses.
