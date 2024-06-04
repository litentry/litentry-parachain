#!/bin/bash

log_file='./worker.log'

touch ${log_file}

check_env(){
    if [ -z $DATA_DIR ];then
        echo "ENV DATA_DIR not set!"
        exit 1
    fi
}

copy_files(){
    for file in key.txt key_production.txt mrenclave.txt spid.txt spid_production.txt; do
        wkdir_file="${DATA_DIR}/${file}"
        if [ -s  ${wkdir_file} ];then
            echo "Working file ${wkdir_file} exist, not copy"
        else
            echo "Copy working file ${file} to ${DATA_DIR}"
            cp /origin/${file} ${DATA_DIR}/
        fi
    done
    # Must copy,Ensure the consistency of binary files.
    cp /origin/enclave.signed.so ${DATA_DIR}/

}

runtime(){
    /usr/local/bin/bitacross-worker --version
    echo "Worker subcommand is: $@"
    /usr/local/bin/bitacross-worker $@
}

check_env
copy_files
runtime $@ >> ${log_file} 2>&1