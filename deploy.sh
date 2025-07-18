#!/bin/bash
echo "runnning invisible-fun deployscript"

remoteServer=tivasl@nuc.local
remoteDeployDir="invisible-fun"
remotePlusDeployDir="$remoteServer:$remoteDeployDir"
start_script="run_docker.sh"

# build with as release
echo "Building..."
cargo build --release
echo "Building - Done"

# build docker
docker build . -t tivasl/eightt3
docker push tivasl/eightt3

# Deploy
# Convenience for in-house-deployment
ssh $remoteServer mkdir -p $remoteDeployDir
scp -r $start_script "$remotePlusDeployDir/$start_script"
ssh $remoteServer "cd $remoteDeployDir && chmod +x $start_script && ./$start_script"