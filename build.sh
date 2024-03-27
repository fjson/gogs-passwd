#!/bin/bash -e
cross_target=aarch64-unknown-linux-musl
host=hf@172.21.53.84
cross build --target $cross_target --release
upx ./target/$cross_target/release/gpasswd
scp ./target/$cross_target/release/gpasswd $host:/Users/hf/jenkinsWorkspace/jenkins/scripts/gogs-passwd
ssh $host > /dev/null 2>&1 << eeooff
chmod +x /Users/hf/jenkinsWorkspace/jenkins/scripts/gogs-passwd/gpasswd
exit
eeooff