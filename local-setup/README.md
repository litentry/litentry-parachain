#  How to use the local-setup

## Prerequisite
- worker built with ` SGX_MODE=SW make`
- integritee-node built with `cargo build --release`

In case you have
- a sgx hardware and compile the worker with `SGX_MODE=HW` (default mode)
- a valid intel IAS key (development key is fine)

## Steps
Check possible options to launch both node and worker(s)
```bash
./local-setup/launch.py -h

options:
  -h, --help            show this help message and exit
  -w WORKERS_NUMBER, --workers-number WORKERS_NUMBER
                        Number of workers to run
  -p [PARACHAIN], --parachain [PARACHAIN]
                        Config for parachain selection: local-binary-standalone / local-docker / local-binary / remote
  -l [LOG_CONFIG_PATH], --log-config-path [LOG_CONFIG_PATH]
                        log level config file path
  -o [OFFSET], --offset [OFFSET]
                        offset for port
```


### Launch worker and node in terminal one
By default with the script you can launch node in local-binary-standalone mode + 1 worker:
```bash
./local-setup/launch.py
```
wait a little until all workers have been launched. You can stop the worker and node simply by pressing `Ctrl + c`.

In case you want to launch multiple workers, just use `-w` flag with number, the workers ports will be adjusted accordingly

### Open a second terminal to show logs
```bash
cd local-setup
./tmux_logger.sh
```

You can remove the tmux session of the script by running
```bash
tmux kill-session -t integritee_logger
```
### Open a third terminal to run a demo
```bash
cd <worker directory>/cli
./demo_shielding_unshielding.sh -p 99xx -P 20xx
```
