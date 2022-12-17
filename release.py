import os
import sys



#-f target/rp2040.cfg -c "program hello_serial.elf verify reset exit"


#interface = "interface/picoprobe.cfg"
interface = "interface/raspberrypi-swd.cfg"
target = "target/rp2040.cfg"
program = f'-c "program {sys.argv[1]} verify reset exit"'
cmd = f'-f {interface} -f {target} {program}'
os.system(f'openocd {cmd}')  # execute openocd
