import os
import pathlib
import shutil
import subprocess
from subprocess import Popen, STDOUT
from typing import Union, TextIO, IO

from .helpers import run_subprocess, setup_working_dir, mkdir_p


class Worker:
    def __init__(
        self,
        worker_bin: str = "./litentry-worker",
        cwd: str = "./",
        source_dir: str = "./",
        std_err: Union[None, int, IO] = STDOUT,
        log_level_dic: {} = {},
    ):
        """
        litentry-worker wrapper.

        Args:
            worker_bin: Path to the worker bin relative to `cwd` or as absolute path.

            cwd:        working directory of the worker.

            source_dir: directory of the source binaries, which will be copied to cwd because
                        the rust worker looks for files relative to cwd.

            std_err:    Were the workers error output will be logged. Note: `std_out` is intended to be unconfigurable
                        because the prints from the rust worker are often intended to be used in scripts. Making this
                        configurable, could cause some weird errors.


        """
        self.cwd = cwd
        self.cli = [worker_bin]
        self.source_dir = source_dir
        self.std_err = std_err
        # cache fields
        self._mrenclave = None
        self.log_level_dic = log_level_dic

    def setup_cwd(self):
        mkdir_p(self.cwd)
        setup_working_dir(self.source_dir, self.cwd)

    def init_clean(self):
        """Purges all db files first and initializes the environment afterwards."""
        mkdir_p(self.cwd)
        print("Copying source files to working directory")
        self.setup_cwd()

    def run_in_background(
        self, log_file: TextIO, flags: [str] = None, subcommand_flags: [str] = None
    ):
        """Runs the worker in the background and writes to the supplied logfile.

        :return: process handle for the spawned background process.
        """

        env = dict(
            os.environ,
            RUST_LOG=self.log_level_dic['litentry-worker'],
        )

        worker_cmd = self._assemble_cmd(flags=flags, subcommand_flags=subcommand_flags)
        print("worker command is: "+ str(worker_cmd))
        return Popen(
            worker_cmd,
            env=env,
            stdout=log_file,
            stderr=STDOUT,
            bufsize=1,
            cwd=self.cwd,
        )

    def _assemble_cmd(self, flags: [str] = None, subcommand_flags: [str] = None):
        """Assembles the cmd skipping None values."""
        cmd = self.cli
        if flags:
            cmd += flags
        cmd += ["run"]
        if subcommand_flags:
            cmd += subcommand_flags
        return cmd
